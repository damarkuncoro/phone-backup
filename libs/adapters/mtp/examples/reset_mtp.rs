//! cargo run --example reset_mtp
use mtp_rs::MtpDevice;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Looking for connected MTP devices to reset...");
    let devices = MtpDevice::list_devices()?;
    if devices.is_empty() {
        println!("❌ No MTP devices found on USB.");
        return Ok(());
    }

    for dev in devices {
        if let Some(serial) = dev.serial_number {
            println!(
                "🔄 Resetting MTP USB transport for device {} ({:?})...",
                serial, dev.product
            );
            match MtpDevice::reset_by_serial(&serial).await {
                Ok(_) => println!("✅ Reset command sent successfully to {}!", serial),
                Err(e) => println!("⚠️ Reset notice for {}: {}", serial, e),
            }
        }
    }

    println!("✨ Done! Wait 2 seconds before reopening session.");
    Ok(())
}
