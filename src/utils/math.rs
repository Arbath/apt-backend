use evalexpr::{eval_with_context, ContextWithMutableVariables, HashMapContext, Value, DefaultNumericTypes};
use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};

use crate::models::accreditation::InputRule;
use crate::utils::response::AppError;

/// Fungsi helper untuk mengeksekusi formula string dengan variabel dinamis
pub async fn calculate_formula(formula: &str, variables: &[InputRule]) -> Result<Decimal, AppError> {
    let mut context: HashMapContext<DefaultNumericTypes> = HashMapContext::new();

    // Masukkan semua variabel dari InputRule ke dalam context
    for rule in variables {
        let val_f64 = rule.val.to_f64().ok_or_else(|| {
            AppError::BadRequest(format!("Gagal membaca nilai desimal pada variabel: {}", rule.var))
        })?;
        
        // Daftarkan variabel ke evalexpr (misal: "A" = 10.0)
        context.set_value(rule.var.clone(), Value::Float(val_f64)).map_err(|e| {
            AppError::BadRequest(format!("Gagal mendaftarkan variabel {}: {}", rule.var, e))
        })?;
    }

    // Eksekusi formula menggunakan context
    let result = eval_with_context(formula, &context).map_err(|e| {
        AppError::BadRequest(format!("Gagal menghitung formula ({}): {}", formula, e))
    })?;
    let result_f64 = result.as_float().map_err(|e| {
        AppError::BadRequest(format!("Hasil kalkulasi bukan angka yang valid: {}", e))
    })?;
    Decimal::from_f64(result_f64).ok_or_else(|| {
        AppError::BadRequest("Gagal mengonversi hasil akhir kalkulasi ke tipe Decimal".to_string())
    })
}

/// Fungsi helper untuk menghitung skor proporsional (0.00 - 3.00)
pub async fn calculate_proportional_score(
    actual_result: Decimal, 
    expectation_result: Decimal
) -> Result<Decimal, AppError> {
    
    // Ekstrak nilai Decimal ke f64 [cite: 90, 91]
    let actual = actual_result.to_f64().unwrap_or(0.0);
    let expected = expectation_result.to_f64().unwrap_or(0.0);

    // Kalkulasi Skor Proporsional
    let raw_score = if expected == 0.0 {
        if actual >= 0.0 { 3.00 } else { 0.00 }
    } else {
        (actual / expected) * 3.00
    };

    // Mengunci nilai (clamping) di range 0.00 - 3.00 [cite: 86] 
    // dan membulatkan ke 2 desimal agar sesuai tipe NUMERIC(3,2) [cite: 88, 91]
    let clamped_score = (raw_score.clamp(0.00, 3.00) * 100.0).round() / 100.0;
    let final_score = Decimal::from_f64(clamped_score).unwrap_or_default();

    Ok(final_score)
}