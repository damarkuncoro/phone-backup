//! Run with: cargo run --example test_native_mtp (if added to examples)
//! Or just use this as a reference for what I\u0027ve added to the library.

use mtp_rs::MtpDevice;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Scanning USB Bus for MTP devices (Native Pure-Rust)...");

    let devices = MtpDevice::list_devices()?;

    if devices.is_empty() {
        println!("❌ No MTP devices found on USB.");
        println!("💡 Tip: Make sure your phone is connected and \u0027File Transfer\u0027 mode is active.");
        return Ok(());
    }

    for (idx, info) in devices.into_iter().enumerate() {
        println!("\n[Device #{}]", idx + 1);
        println!("- Manufacturer: {}", info.manufacturer.as_deref().unwrap_or("Unknown"));
        println!("- Product:      {}", info.product.as_deref().unwrap_or("Unknown"));
        println!("- Serial:       {}", info.serial_number.as_deref().unwrap_or("N/A"));
        println!("- Location ID:  {}", info.location_id);

        println!("⚙️ Attempting to open session...");
        match MtpDevice::open_by_location(info.location_id).await {
            Ok(device) => {
                println!("✅ Session opened successfully!");
                let storages = device.storages().await?;
                println!("Found {} storage(s):", storages.len());
                for s in storages {
                    println!("  -> {} ({} bytes free)", s.info().description, s.info().free_space);
                }
            },
            Err(e) => println!("❌ Failed to open session: {}", e),
        }
    }

    Ok(())
}
