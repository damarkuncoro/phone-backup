use phone_backup_application::ProgressEstimator;

#[test]
fn test_progress_estimator_initial_state() {
    let estimator = ProgressEstimator::new(10 * 1024 * 1024);
    assert_eq!(estimator.total_bytes(), 10 * 1024 * 1024);
    assert_eq!(estimator.processed_bytes(), 0);
    assert_eq!(estimator.format_eta(), "Calculating...");
}

#[test]
fn test_progress_estimator_update() {
    let mut estimator = ProgressEstimator::new(10 * 1024 * 1024);
    estimator.update(5 * 1024 * 1024);
    assert_eq!(estimator.processed_bytes(), 5 * 1024 * 1024);

    estimator.set_processed_bytes(10 * 1024 * 1024);
    assert_eq!(estimator.processed_bytes(), 10 * 1024 * 1024);
    assert_eq!(estimator.format_eta(), "Completed");
}
