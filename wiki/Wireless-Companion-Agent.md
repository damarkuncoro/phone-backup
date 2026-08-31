# 📱 Wireless Companion Agent

Platform **phone-backup** menyediakan solusi pencadangan nirkabel melalui arsitektur **Companion Agent APK** (`apps/android-agent/` dan `adapters/agent`).

---

## 1. Mengapa Perlu Agen Nirkabel?

Secara arsitektural:
- **Kabel Biasa (MTP)**: Android memblokir pembacaan Kontak, SMS, dan Call Logs demi keamanan.
- **Kabel ADB (USB Debugging)**: Memerlukan mode pengembang yang cukup rumit bagi pengguna umum dan membutuhkan koneksi kabel fisik.
- **Wireless Companion Agent**: Memberikan kemampuan pencadangan **100% nirkabel melalui jaringan Wi-Fi lokal** tanpa perlu mengaktifkan mode pengembang (USB Debugging).

---

## 2. Arsitektur Komunikasi Agen

```text
+-----------------------+                    +-------------------------+
|    Computer Node      |                    |   Android Smartphone    |
| (phone-backup Engine) |                    |  (Companion Agent APK)  |
+-----------+-----------+                    +------------+------------+
            |                                             |
            | <--------- 1. Discovery (mDNS / IP) ------> |
            | <--------- 2. Handshake & Auth -----------> |
            |                                             |
            | --------- 3. Query Contacts / SMS --------> | (Content Providers)
            | <-------- 4. Stream Structured Data ------- |
            |                                             |
            | --------- 5. Request File Stream ---------> | (Local Storage)
            | <-------- 6. Binary Stream Payload -------- |
            |                                             |
```

---

## 3. Protokol & Kontrak Data (`adapters/agent`)

Adapter agen Rust (`phone-backup-adapter-agent`) mendefinisikan kontrak komunikasi nirkabel berbasis JSON / binary streaming:
- `AgentHandshake`: Negosiasi protokol, ID perangkat, dan matriks izin.
- `AgentFileScanResponse`: Daftar berkas dan metadata yang tersedia di smartphone.
- `AgentStructuredDataResponse`: Data terstruktur kontak, pesan SMS, riwayat panggilan, dan APK.
- `AgentHeartbeat`: Status kesehatan baterai dan suhu smartphone secara berkala.

---

## 4. Cara Penggunaan Adapter Nirkabel

### A. Di Smartphone Android:
1. Pasang dan buka aplikasi **Phone Backup Agent** (`apps/android-agent/`).
2. Berikan izin akses Kontak, SMS, dan Media saat diminta.
3. Ketuk tombol **Start Agent Service** untuk membuka listener jaringan lokal.

### B. Di Komputer (Terminal CLI):
Gunakan flag `--adapter agent`:

```bash
# 1. Pindai agen nirkabel yang aktif di jaringan Wi-Fi lokal
phone-backup --adapter agent devices

# 2. Periksa kapabilitas agen nirkabel
phone-backup --adapter agent device-info AGENT_WIRELESS_01

# 3. Jalankan backup nirkabel terenkripsi
phone-backup --adapter agent backup -p "SandiNirkabel123" AGENT_WIRELESS_01
```

---
*Lanjutkan ke: [Contacts & Data Management](Contacts-and-Data-Management.md) atau [Troubleshooting & FAQ](Troubleshooting-and-FAQ.md).*
