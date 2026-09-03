use phone_backup_messages::OtpExtractor;

#[test]
fn test_extract_numeric_otp_codes() {
    let res1 = OtpExtractor::extract("Google", "Your Google security code is 492019. Jangan berikan kepada siapapun.");
    assert!(res1.is_some());
    let otp1 = res1.unwrap();
    assert_eq!(otp1.code, "492019");
    assert_eq!(otp1.service_name.as_deref(), Some("Google"));

    let res2 = OtpExtractor::extract("Tokopedia", "Gunakan kode 839201 untuk login ke akun Tokopedia Anda.");
    assert!(res2.is_some());
    let otp2 = res2.unwrap();
    assert_eq!(otp2.code, "839201");
    assert_eq!(otp2.service_name.as_deref(), Some("Tokopedia"));
}

#[test]
fn test_ignore_messages_without_otp() {
    let res = OtpExtractor::extract("+6281234567890", "Halo bro, apa kabar?");
    assert!(res.is_none());
}
