use anyhow::Result;
use mtp_rs::MtpDevice;
use phone_backup_adapter_mtp::MtpConflictResolver;

#[tokio::main]
async fn main() -> Result<()> {
    println!("============================================================");
    println!("   📱 UJI COBA MTP DENGAN REAL HP (Infinix NOTE 30)        ");
    println!("============================================================");

    println!("\n[1] Mencari perangkat USB MTP...");
    let devices = MtpDevice::list_devices()?;
    println!("   Ditemukan {} perangkat MTP:", devices.len());
    for (idx, d) in devices.iter().enumerate() {
        println!("   [{}] Manufaktur: {:?}", idx + 1, d.manufacturer);
        println!("       Model:       {:?}", d.product);
        println!("       Serial USB:  {:?}", d.serial_number);
        println!("       Location ID: 0x{:x}", d.location_id);
    }

    if devices.is_empty() {
        println!("\n❌ Tidak ada perangkat Android MTP yang terdeteksi via USB.");
        println!("👉 Pastikan kabel USB terpasang dengan baik ke Mac.");
        return Ok(());
    }

    let dev_info = &devices[0];
    let serial = dev_info.serial_number.as_deref().unwrap_or("unknown");

    println!("\n[2] Membersihkan agen/proses macOS pengganggu (PTPCamera/Photos)...");
    let _ = MtpConflictResolver::resolve_conflicts(serial);

    println!("\n[3] Membuka sesi MTP ke perangkat...");
    let mut device_opt = None;
    for attempt in 1..=5 {
        let res = if let Some(s) = dev_info.serial_number.as_deref() {
            MtpDevice::open_by_serial(s).await
        } else {
            MtpDevice::open_by_location(dev_info.location_id).await
        };

        match res {
            Ok(dev) => {
                println!("   ✅ Sesi MTP berhasil dibuka!");
                device_opt = Some(dev);
                break;
            }
            Err(e) => {
                println!("   ⚠️ Percobaan #{} gagal ({}). Mencoba lagi...", attempt, e);
                let _ = std::process::Command::new("killall").args(["-9", "PTPCamera", "ptpcamera", "ptpcamerad"]).output();
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
            }
        }
    }

    let mut device = match device_opt {
        Some(d) => d,
        None => {
            println!("\n❌ Gagal membuka koneksi MTP.");
            return Ok(());
        }
    };

    println!("\n[4] Informasi Perangkat:");
    let info = device.device_info();
    println!("   - Manufaktur:     {}", info.manufacturer);
    println!("   - Model:          {}", info.model);
    println!("   - Versi Perangkat: {}", info.device_version);
    println!("   - Serial Internal: {}", info.serial_number);

    println!("\n[5] Memeriksa Partisi Penyimpanan (Storage)...");
    let mut storages = device.storages().await?;
    
    if storages.is_empty() {
        println!("\n⚠️  HP TERKUNCI ATAU DALAM MODE 'CHARGING ONLY'");
        println!("👉 Silakan lakukan langkah berikut di HP Anda:");
        println!("   1. Buka kunci layar (Unlock screen: PIN/Pola/Sidik Jari).");
        println!("   2. Tarik bar notifikasi -> pilih 'USB untuk transfer file' / 'File Transfer (MTP)'.");
        println!("\n⏳ Menunggu perangkat dibuka kuncinya (tekan Ctrl+C untuk berhenti)...");

        for i in 1..=30 {
            tokio::time::sleep(tokio::time::Duration::from_millis(1500)).await;
            
            // Reopen device session if needed
            let _ = MtpConflictResolver::resolve_conflicts(serial);
            if let Ok(new_dev) = if let Some(s) = dev_info.serial_number.as_deref() {
                MtpDevice::open_by_serial(s).await
            } else {
                MtpDevice::open_by_location(dev_info.location_id).await
            } {
                device = new_dev;
                if let Ok(st) = device.storages().await {
                    if !st.is_empty() {
                        println!("\n🎉 HP BERHASIL DIBUKA KUNCINYA!");
                        storages = st;
                        break;
                    }
                }
            }
            print!(".");
            use std::io::Write;
            let _ = std::io::stdout().flush();
        }
        println!();
    }

    if storages.is_empty() {
        println!("\n⚠️ Partisi penyimpanan belum tersedia. Silakan coba jalankan lagi setelah membuka kunci HP.");
        return Ok(());
    }

    println!("\n[6] Ditemukan {} partisi penyimpanan:", storages.len());
    for (i, storage) in storages.iter().enumerate() {
        let s_info = storage.info();
        println!("\n📦 Partisi #{}: {}", i + 1, s_info.description);
        println!("   - Storage ID:     0x{:x}", s_info.id.0);
        println!("   - Volume Label:   {}", s_info.volume_identifier);
        println!("   - Total Kapasitas: {:.2} GB ({} bytes)", 
            s_info.total_capacity as f64 / (1024.0 * 1024.0 * 1024.0), s_info.total_capacity);
        println!("   - Ruang Bebas:    {:.2} GB ({} bytes)", 
            s_info.free_space as f64 / (1024.0 * 1024.0 * 1024.0), s_info.free_space);
        let used = s_info.total_capacity.saturating_sub(s_info.free_space);
        println!("   - Terpakai:       {:.2} GB ({:.1}%)", 
            used as f64 / (1024.0 * 1024.0 * 1024.0), 
            if s_info.total_capacity > 0 { (used as f64 / s_info.total_capacity as f64) * 100.0 } else { 0.0 });

        println!("\n[7] Memindai file & folder di root penyimpanan...");
        match storage.list_objects(None).await {
            Ok(objects) => {
                println!("   📁 Total item di direktori utama: {}", objects.len());
                for (idx, obj) in objects.iter().take(20).enumerate() {
                    if let Ok(item_info) = storage.get_object_info(obj.handle).await {
                        let is_dir = item_info.format.is_association();
                        let tag = if is_dir { "[FOLDER]" } else { "[FILE]  " };
                        println!("     {:2}. {} {:<25} (Size: {:>9} B, Handle: 0x{:x})",
                            idx + 1, tag, obj.filename, item_info.size, obj.handle.0);

                        // If folder is DCIM or Download or Pictures, list subfolder items
                        if is_dir && (obj.filename == "DCIM" || obj.filename == "Download" || obj.filename == "Pictures" || obj.filename == "Documents") {
                            println!("        ↳ Isi dari '{}':", obj.filename);
                            if let Ok(sub_items) = storage.list_objects(Some(obj.handle)).await {
                                println!("          (Ditemukan {} item)", sub_items.len());
                                for (sub_idx, sub) in sub_items.iter().take(5).enumerate() {
                                    if let Ok(sub_info) = storage.get_object_info(sub.handle).await {
                                        let sub_tag = if sub_info.format.is_association() { "[DIR] " } else { "[FILE]" };
                                        println!("            {}.{} {} {:<22} ({} B)", idx + 1, sub_idx + 1, sub_tag, sub.filename, sub_info.size);
                                    }
                                }
                                if sub_items.len() > 5 {
                                    println!("            ... dan {} item lainnya", sub_items.len() - 5);
                                }
                            }
                        }
                    }
                }
                if objects.len() > 20 {
                    println!("     ... dan {} item lainnya di root", objects.len() - 20);
                }
            }
            Err(e) => {
                println!("   ❌ Gagal memindai objek: {}", e);
            }
        }
    }

    println!("\n============================================================");
    println!("   ✅ UJI COBA MTP SELESAI DENGAN SUKSES!                   ");
    println!("============================================================");
    Ok(())
}
