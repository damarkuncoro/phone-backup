use anyhow::Result;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CloudTestResult {
    pub success: bool,
    pub message: String,
    pub provider: String,
}

#[tauri::command(rename_all = "snake_case")]
pub async fn test_cloud_connection(
    provider: String,
    bucket: String,
    endpoint: Option<String>,
    region: Option<String>,
    access_key: Option<String>,
    secret_key: Option<String>,
) -> Result<CloudTestResult, String> {
    if bucket.trim().is_empty() {
        return Ok(CloudTestResult {
            success: false,
            message: "Bucket name cannot be empty".to_string(),
            provider,
        });
    }

    match provider.to_lowercase().as_str() {
        "s3" | "s3-compatible" | "minio" => {
            let mut builder = opendal::services::S3::default();
            builder = builder.bucket(&bucket);
            if let Some(reg) = region.filter(|s| !s.trim().is_empty()) {
                builder = builder.region(&reg);
            }
            if let Some(ep) = endpoint.filter(|s| !s.trim().is_empty()) {
                builder = builder.endpoint(&ep);
            }
            if let Some(ak) = access_key.filter(|s| !s.trim().is_empty()) {
                builder = builder.access_key_id(&ak);
            }
            if let Some(sk) = secret_key.filter(|s| !s.trim().is_empty()) {
                builder = builder.secret_access_key(&sk);
            }

            match opendal::Operator::new(builder) {
                Ok(op_builder) => {
                    let op = op_builder.finish();
                    match op.check().await {
                        Ok(_) => Ok(CloudTestResult {
                            success: true,
                            message: format!("Successfully connected to S3 bucket '{}'", bucket),
                            provider,
                        }),
                        Err(e) => Ok(CloudTestResult {
                            success: false,
                            message: format!("S3 connection failed: {}", e),
                            provider,
                        }),
                    }
                }
                Err(e) => Ok(CloudTestResult {
                    success: false,
                    message: format!("Invalid S3 configuration: {}", e),
                    provider,
                }),
            }
        }
        "webdav" | "nextcloud" => {
            let ep = endpoint.unwrap_or_default();
            if ep.trim().is_empty() {
                return Ok(CloudTestResult {
                    success: false,
                    message: "WebDAV endpoint URL cannot be empty".to_string(),
                    provider,
                });
            }
            let mut builder = opendal::services::Webdav::default();
            builder = builder.endpoint(&ep);
            if let Some(user) = access_key.filter(|s| !s.trim().is_empty()) {
                builder = builder.username(&user);
            }
            if let Some(pass) = secret_key.filter(|s| !s.trim().is_empty()) {
                builder = builder.password(&pass);
            }

            match opendal::Operator::new(builder) {
                Ok(op_builder) => {
                    let op = op_builder.finish();
                    match op.check().await {
                        Ok(_) => Ok(CloudTestResult {
                            success: true,
                            message: format!("Successfully connected to WebDAV server at '{}'", ep),
                            provider,
                        }),
                        Err(e) => Ok(CloudTestResult {
                            success: false,
                            message: format!("WebDAV connection failed: {}", e),
                            provider,
                        }),
                    }
                }
                Err(e) => Ok(CloudTestResult {
                    success: false,
                    message: format!("Invalid WebDAV configuration: {}", e),
                    provider,
                }),
            }
        }
        _ => Ok(CloudTestResult {
            success: true,
            message: format!("Mock test passed for provider: {}", provider),
            provider,
        }),
    }
}
