use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageCategory {
    Otp,
    Banking,
    Promo,
    Personal,
}

pub struct MessageClassifier;

impl MessageClassifier {
    pub fn classify(address: &str, body: &str) -> MessageCategory {
        let lower_body = body.to_lowercase();
        let lower_addr = address.to_lowercase();

        // 1. OTP / Verification checks
        if lower_body.contains("kode verifikasi")
            || lower_body.contains("verification code")
            || lower_body.contains("otp")
            || lower_body.contains("security code")
            || lower_body.contains("kode rahasia")
            || lower_body.contains("jangan bagikan kode")
        {
            return MessageCategory::Otp;
        }

        // 2. Banking / Transaction alerts
        if lower_body.contains("rekening")
            || lower_body.contains("debit")
            || lower_body.contains("kredit")
            || lower_body.contains("saldo")
            || lower_body.contains("transfer berhasil")
            || lower_body.contains("rp ")
            || lower_body.contains("idr ")
            || lower_addr.contains("bca")
            || lower_addr.contains("mandiri")
            || lower_addr.contains("bri")
            || lower_addr.contains("bni")
        {
            return MessageCategory::Banking;
        }

        // 3. Promo / Marketing / Spam
        if lower_body.contains("promo")
            || lower_body.contains("diskon")
            || lower_body.contains("cashback")
            || lower_body.contains("voucher")
            || lower_body.contains("selamat anda mendapatkan")
            || lower_body.contains("klik link berikut")
            || lower_body.contains("bonus kuota")
        {
            return MessageCategory::Promo;
        }

        // 4. Default to Personal
        MessageCategory::Personal
    }
}
