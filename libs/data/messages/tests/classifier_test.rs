use phone_backup_messages::{MessageCategory, MessageClassifier};

#[test]
fn test_classify_otp_and_security_codes() {
    let cat1 = MessageClassifier::classify("Google", "G-849201 is your Google verification code. Do not share.");
    assert_eq!(cat1, MessageCategory::Otp);

    let cat2 = MessageClassifier::classify("WhatsApp", "Kode verifikasi WhatsApp Anda: 492-104.");
    assert_eq!(cat2, MessageCategory::Otp);
}

#[test]
fn test_classify_banking_alerts() {
    let cat1 = MessageClassifier::classify("BankBCA", "Rekening 12345678 debit Rp 500.000 untuk QRIS.");
    assert_eq!(cat1, MessageCategory::Banking);

    let cat2 = MessageClassifier::classify("MANDIRI", "Transfer berhasil IDR 1.250.000 ke rekening 9876.");
    assert_eq!(cat2, MessageCategory::Banking);
}

#[test]
fn test_classify_promotions_and_personal() {
    let cat_promo = MessageClassifier::classify("INDOSAT", "Dapatkan promo diskon kuota 50GB hanya 50rb klik link berikut.");
    assert_eq!(cat_promo, MessageCategory::Promo);

    let cat_personal = MessageClassifier::classify("+6281234567890", "Halo bro, nanti siang jadi ketemuan di kantor?");
    assert_eq!(cat_personal, MessageCategory::Personal);
}
