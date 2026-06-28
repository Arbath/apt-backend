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