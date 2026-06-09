# Sistem Akreditasi Perguruan Tinggi

Backend service untuk mengelola proses akreditasi perguruan tinggi secara terpusat, mulai dari pengumpulan dokumen, sinkronisasi data eksternal, perhitungan indikator akreditasi, hingga audit aktivitas pengguna.

> **Catatan Portofolio**
>
> Proyek ini dirancang sebagai studi kasus implementasi sistem informasi skala institusi dengan fokus pada arsitektur backend, efisiensi resource, keamanan data, dan kemampuan menangani ribuan dokumen akreditasi secara bersamaan.

---

## Gambaran Umum

Aplikasi menggunakan pendekatan **Decoupled Architecture**, di mana frontend dan backend berjalan secara terpisah dan berkomunikasi melalui REST API.

Pendekatan ini memberikan beberapa keuntungan:

* Skalabilitas lebih baik antara frontend dan backend.
* Deployment yang lebih fleksibel.
* Memudahkan integrasi dengan aplikasi eksternal.
* Mendukung pengembangan paralel oleh tim frontend dan backend.

---

## Tech Stack

### Frontend

* React

React digunakan untuk membangun antarmuka pengguna yang responsif dan interaktif sehingga proses pengisian borang akreditasi dapat dilakukan dengan lebih efisien.

### Backend

* Rust
* Axum
* Tokio

Backend dibangun menggunakan Rust dan framework Axum untuk mendapatkan:

* Memory footprint yang rendah.
* Performa tinggi.
* Dukungan asynchronous programming yang kuat.
* Kemampuan menangani ribuan request secara bersamaan tanpa blocking.

Pemanfaatan Tokio Runtime memungkinkan proses seperti:

* Upload dokumen.
* Sinkronisasi data eksternal.
* Perhitungan indikator.
* Logging aktivitas.

berjalan secara concurrent tanpa mengganggu request utama pengguna.

### Database

* PostgreSQL

Dipilih karena:

* Mendukung transaksi ACID.
* Konsistensi data yang tinggi.
* Cocok untuk sistem informasi yang memerlukan integritas data.
* Mendukung query kompleks dan relasi yang kuat.

### Deployment

* Docker

Seluruh service dikemas menggunakan Docker untuk memastikan:

* Konsistensi lingkungan deployment.
* Kemudahan distribusi.
* Proses provisioning server yang lebih sederhana.

---

# Arsitektur Sistem

```text
┌──────────────────┐
│     Frontend     │
│      React       │
└────────┬─────────┘
         │ REST API
         ▼
┌──────────────────┐
│   API Service    │
│   Rust + Axum    │
└───────┬──────────┘
        │
        ├─────────────────┐
        │                 │
        ▼                 ▼
┌──────────────┐   ┌──────────────┐
│ PostgreSQL   │   │ File Storage │
└──────────────┘   └──────────────┘
        ▲
        │
        ▼
┌──────────────────┐
│ Scheduler Worker │
│ Background Jobs  │
└──────────────────┘
```

Sistem dirancang menggunakan pendekatan service separation:

1. API Service
2. Background Scheduler
3. Document Storage Service

Pendekatan ini memungkinkan setiap komponen berkembang secara independen sesuai kebutuhan beban kerja.

---

# Fitur Utama Backend

## 1. Role-Based Access Control (RBAC)

Sistem menerapkan otorisasi multilevel untuk memastikan setiap pengguna hanya dapat mengakses fitur sesuai perannya.

### Operator Program Studi / Fakultas

* Upload dokumen bukti
* Mengisi formulir akreditasi
* Melihat status verifikasi

### Tim Penjaminan Mutu

* Approve dokumen
* Reject dokumen
* Validasi hasil perhitungan indikator

### Administrator

* Manajemen pengguna
* Manajemen fakultas
* Manajemen program studi
* Konfigurasi sistem

### Pimpinan / Asesor Internal

* Read-only access
* Monitoring seluruh dokumen
* Monitoring dashboard capaian indikator

---

## 2. Document Management System (DMS)

Sistem menerima dan mengelola ribuan dokumen pendukung akreditasi seperti:

* SK
* Renstra
* Laporan Audit Mutu
* Laporan Evaluasi Diri
* Dokumen Kebijakan

### Karakteristik Sistem

* Upload file PDF menggunakan multipart/form-data.
* Penyimpanan file pada local file system.
* Metadata file tersimpan di PostgreSQL.
* Mendukung concurrent upload.
* Efisiensi penggunaan memori saat transfer file berukuran besar.

### Tipe Bukti yang Didukung

#### Upload Dokumen

Digunakan untuk indikator yang memerlukan dokumen resmi dalam bentuk file.

#### Input URL

Digunakan untuk indikator yang sumber buktinya berasal dari website perguruan tinggi.

#### API Synchronization

Digunakan untuk indikator yang sumber datanya berasal dari sistem eksternal seperti:

* PD Dikti
* Sistem Pelaporan SPMI

---

## 3. Background Scheduler & API Aggregator

Data tertentu tidak boleh diinput secara manual dan harus diperoleh langsung dari sumber resmi.

Untuk menghindari latency tinggi pada request pengguna, proses sinkronisasi dilakukan oleh background worker.

### Mekanisme

1. Scheduler berjalan secara periodik.
2. Worker melakukan fetch data dari API eksternal.
3. Data hasil sinkronisasi disimpan ke database lokal.
4. Frontend membaca data dari cache lokal.

### Keuntungan

* Respon API lebih cepat.
* Mengurangi ketergantungan terhadap availability API eksternal.
* Mengurangi jumlah request ke sistem pihak ketiga.

Contoh data yang disinkronkan:

* Jumlah dosen aktif
* Jumlah mahasiswa aktif
* Persentase guru besar
* Data SPMI
* Data penelitian

---

## 4. Dynamic Formula Engine

Backend memiliki engine kalkulasi yang bertugas menerjemahkan indikator BAN-PT menjadi fungsi perhitungan otomatis.

### Contoh Formula

#### Rasio Guru Besar dan Lektor Kepala

```text
DITA = ((NDTGB + NDTLK) / NDT) × 100%
```

#### Rasio Luaran Penelitian

```text
RLP = (NA2 + NA3 + NA4 + NB2 + NB3 + NC2 + NC3) / NDT × 100%
```

### Kemampuan Engine

* Menghitung indikator secara otomatis.
* Melakukan validasi terhadap data masukan.
* Menentukan status pencapaian indikator.
* Memberikan warning jika target belum tercapai.
* Mendukung penambahan formula baru tanpa mengubah struktur sistem utama.

Contoh:

```text
Status:
Belum Memenuhi Syarat Unggul

Alasan:
Persentase dosen bergelar doktor < 30%
```

---

## 5. Audit Trail

Seluruh aktivitas pengguna direkam untuk menjaga akuntabilitas data akreditasi.

### Aktivitas yang Dicatat

* Login pengguna
* Upload dokumen
* Hapus dokumen
* Approve dokumen
* Reject dokumen
* Sinkronisasi data eksternal
* Perubahan data master

### Informasi Log

```text
User ID
Role
Action
Target Resource
Timestamp
IP Address
```

Audit trail membantu proses investigasi ketika terjadi:

* Kehilangan dokumen
* Kesalahan data
* Ketidaksesuaian bukti saat asesmen

---

# Tantangan Backend yang Diselesaikan

### High Concurrency

Menangani banyak pengguna yang melakukan pengisian dokumen secara bersamaan menjelang batas waktu akreditasi.

### Efficient File Handling

Mengelola ribuan file PDF tanpa membebani memori server.

### External API Reliability

Menjaga sistem tetap responsif meskipun API eksternal mengalami gangguan.

### Data Consistency

Memastikan seluruh proses bisnis berjalan secara konsisten melalui transaksi database PostgreSQL.

### Security & Authorization

Mencegah akses tidak sah terhadap dokumen dan data institusi.

---

# Fokus Kompetensi Backend yang Ditunjukkan

Melalui proyek ini saya mengimplementasikan:

* REST API Development
* Asynchronous Programming
* Background Job Processing
* RBAC Authorization
* File Upload Service
* PostgreSQL Database Design
* Audit Logging
* Docker Containerization
* System Scalability
* High-Concurrency Architecture
* Data Synchronization Service
* Formula Processing Engine

---

# Status

Portfolio Project — Backend System Design & Implementation Study

Proyek ini dibuat untuk menunjukkan kemampuan perancangan dan implementasi backend modern menggunakan Rust, Axum, PostgreSQL, dan Docker pada studi kasus sistem informasi akreditasi perguruan tinggi yang memiliki kebutuhan keamanan, konsistensi data, dan skalabilitas tinggi.
