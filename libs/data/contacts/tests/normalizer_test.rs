use phone_backup_contacts::PhoneNormalizer;

#[test]
fn test_indonesian_phone_normalization() {
    assert_eq!(PhoneNormalizer::normalize("081234567890", "+62"), "+6281234567890");
    assert_eq!(PhoneNormalizer::normalize("+62 812-3456-7890", "+62"), "+6281234567890");
    assert_eq!(PhoneNormalizer::normalize("6281234567890", "+62"), "+6281234567890");
    assert_eq!(PhoneNormalizer::normalize("006281234567890", "+62"), "+6281234567890");
    assert_eq!(PhoneNormalizer::normalize("(021) 555-1234", "+62"), "+62215551234");
}

#[test]
fn test_international_phone_normalization() {
    assert_eq!(PhoneNormalizer::normalize("1-800-555-0199", "+1"), "+18005550199");
    assert_eq!(PhoneNormalizer::normalize("+44 20 7946 0958", "+62"), "+442079460958");
    assert_eq!(PhoneNormalizer::normalize("+81 3 1234 5678", "+81"), "+81312345678");
}
