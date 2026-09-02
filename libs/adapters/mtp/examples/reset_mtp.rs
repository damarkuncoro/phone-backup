//! cargo run --example reset_mtp
use mtp_rs::MtpDevice;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let serial = "10DDAJ0G7D0002L";
    println!("🔄 Attempting to reset USB MTP transport for serial {}...", serial);

    match MtpDevice::reset_by_serial(serial).await {
        Ok(_) => println!("✅ Reset command sent successfully! Wait a few seconds before retrying."),
        Err(e) => println!("❌ Reset failed: {}", e),
    }

    Ok(())
}
