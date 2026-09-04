const { default: makeWASocket, useMultiFileAuthState, DisconnectReason } = require('@whiskeysockets/baileys');
const qrcodeTerminal = require('qrcode-terminal');
const QRCode = require('qrcode');
const pino = require('pino');
const fs = require('fs');
const path = require('path');

const SESSION_DIR = path.resolve(__dirname, 'session_auth');
const OUTPUT_JSON = path.resolve(__dirname, '../../workspace/synced_whatsapp.json');
const OUTPUT_HTML = path.resolve(__dirname, '../../workspace/synced_whatsapp_viewer.html');
const QR_HTML_PATH = path.resolve(__dirname, '../../workspace/scan_whatsapp_qr.html');

const store = {
    chats: new Map(),
    contacts: new Map(),
    messages: []
};

async function startSync() {
    const { state, saveCreds } = await useMultiFileAuthState(SESSION_DIR);

    const sock = makeWASocket({
        auth: state,
        printQRInTerminal: false,
        logger: pino({ level: 'silent' }),
        browser: ['PhoneBackup', 'Desktop', '1.0.0'],
        syncFullHistory: true
    });

    sock.ev.on('creds.update', saveCreds);

    sock.ev.on('connection.update', async (update) => {
        const { connection, lastDisconnect, qr } = update;

        if (qr) {
            console.log('\n======================================================');
            console.log('📱 SCAN QR CODE DENGAN WHATSAPP DI HP ANDA:');
            console.log('Buka WhatsApp ➔ Titik Tiga ⋮ ➔ Perangkat Tertaut ➔ Tautkan Perangkat');
            console.log('======================================================\n');
            qrcodeTerminal.generate(qr, { small: true });

            // Also generate HTML visual QR code
            try {
                const qrDataUrl = await QRCode.toDataURL(qr, { width: 350, margin: 2 });
                const qrHtml = `<!DOCTYPE html>
<html>
<head>
<title>Scan WhatsApp QR Code</title>
<style>
body { font-family: -apple-system, BlinkMacSystemFont, sans-serif; background: #0b141a; color: #e9edef; display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh; margin: 0; }
.card { background: #111b21; padding: 32px; border-radius: 20px; border: 1px solid #222e35; text-align: center; box-shadow: 0 10px 30px rgba(0,0,0,0.5); }
h1 { color: #00a884; margin-top: 0; font-size: 1.5rem; }
p { color: #8696a0; font-size: 0.95rem; margin-bottom: 24px; line-height: 1.5; }
img { border-radius: 12px; background: white; padding: 12px; }
.steps { text-align: left; background: #202c33; padding: 16px; border-radius: 12px; margin-top: 20px; font-size: 0.85rem; color: #d1d7db; }
.steps ol { margin: 0; padding-left: 20px; }
.steps li { margin-bottom: 6px; }
</style>
</head>
<body>
<div class="card">
<h1>📱 Tautkan WhatsApp Anda</h1>
<p>Scan QR Code di bawah menggunakan WhatsApp di HP Anda</p>
<img src="${qrDataUrl}" alt="WhatsApp QR Code" />
<div class="steps">
<ol>
<li>Buka <b>WhatsApp</b> di HP Vivo Anda</li>
<li>Ketuk <b>Titik Tiga ⋮</b> (Kanan Atas) ➔ <b>Perangkat Tertaut</b></li>
<li>Ketuk <b>Tautkan Perangkat</b> dan arahkan kamera ke QR di atas</li>
</ol>
</div>
</div>
</body>
</html>`;
                fs.writeFileSync(QR_HTML_PATH, qrHtml);
                console.log(`\n🌐 File QR HTML dibuat: ${QR_HTML_PATH}`);
            } catch (e) {
                console.error('Failed to generate HTML QR', e);
            }
        }

        if (connection === 'close') {
            const shouldReconnect = (lastDisconnect?.error)?.output?.statusCode !== DisconnectReason.loggedOut;
            console.log('Koneksi terputus. Mencoba menghubungkan kembali...', shouldReconnect);
            if (shouldReconnect) {
                startSync();
            }
        } else if (connection === 'open') {
            console.log('\n🎉 BERHASIL TERHUBUNG DENGAN WHATSAPP!');
            console.log('Sedang menyinkronkan seluruh riwayat percakapan dari server WhatsApp...\n');
        }
    });

    sock.ev.on('messaging-history.set', ({ chats, contacts, messages, isLatest }) => {
        console.log(`📥 Menerima riwayat pesan: ${chats?.length || 0} chats, ${messages?.length || 0} pesan.`);
        if (chats) chats.forEach(c => store.chats.set(c.id, c));
        if (contacts) contacts.forEach(c => store.contacts.set(c.id, c));
        if (messages) store.messages.push(...messages);
        saveAndRender();
    });

    sock.ev.on('messages.upsert', ({ messages }) => {
        store.messages.push(...messages);
        saveAndRender();
    });
}

function saveAndRender() {
    const chatList = Array.from(store.chats.values());
    const data = {
        total_chats: chatList.length,
        total_messages: store.messages.length,
        synced_at: new Date().toISOString(),
        chats: chatList,
        messages: store.messages
    };
    fs.writeFileSync(OUTPUT_JSON, JSON.stringify(data, null, 2));
    renderHtmlViewer();
}

function renderHtmlViewer() {
    const msgsByChat = {};
    for (const m of store.messages) {
        const jid = m.key.remoteJid;
        if (!msgsByChat[jid]) msgsByChat[jid] = [];
        msgsByChat[jid].push(m);
    }

    let chatBlocks = '';
    for (const [jid, msgs] of Object.entries(msgsByChat)) {
        const chatInfo = store.chats.get(jid) || {};
        const title = chatInfo.name || jid.replace('@s.whatsapp.net', '');
        
        let msgRows = '';
        for (const m of msgs.slice(-100)) { // last 100 per thread
            const fromMe = m.key.fromMe;
            const text = m.message?.conversation || m.message?.extendedTextMessage?.text || (m.message?.imageMessage ? '📷 Foto' : m.message?.audioMessage ? '🎙️ Audio' : '');
            if (!text) continue;
            const time = m.messageTimestamp ? new Date(Number(m.messageTimestamp) * 1000).toLocaleString('id-ID') : '';
            const cls = fromMe ? 'from-me' : 'from-other';
            msgRows += `<div class="msg ${cls}"><div>${escapeHtml(text)}</div><div class="time">${time}</div></div>`;
        }

        if (msgRows) {
            chatBlocks += `<div class="chat-box"><div class="chat-title">💬 ${escapeHtml(title)} (${msgs.length} pesan)</div><div class="messages-container">${msgRows}</div></div>`;
        }
    }

    const html = `<!DOCTYPE html>
<html>
<head>
<title>WhatsApp Offline Synced Archive</title>
<style>
body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; background: #0b141a; color: #e9edef; margin: 0; padding: 20px; }
.header { background: #202c33; padding: 20px; border-radius: 16px; margin-bottom: 24px; border-left: 6px solid #00a884; }
.header h1 { margin: 0; font-size: 1.6rem; color: #00a884; }
.chat-box { background: #111b21; border-radius: 14px; margin-bottom: 24px; padding: 18px; border: 1px solid #222e35; }
.chat-title { font-weight: bold; color: #53bdeb; margin-bottom: 14px; padding-bottom: 8px; border-bottom: 1px solid #222e35; font-size: 1.1rem; }
.messages-container { display: flex; flex-direction: column; gap: 8px; }
.msg { display: flex; flex-direction: column; max-width: 75%; padding: 10px 14px; border-radius: 10px; font-size: 0.95rem; line-height: 1.4; }
.from-me { align-self: flex-end; background: #005c4b; color: #e9edef; border-top-right-radius: 2px; }
.from-other { align-self: flex-start; background: #202c33; color: #e9edef; border-top-left-radius: 2px; }
.time { font-size: 0.7rem; color: #8696a0; align-self: flex-end; margin-top: 4px; }
</style>
</head>
<body>
<div class="header">
<h1>📱 WhatsApp Synced Archive (${store.messages.length} Pesan)</h1>
<p style="color:#8696a0;margin:4px 0 0 0;font-size:0.85rem;">Tersinkronisasi langsung via WhatsApp Multi-Device Protocol</p>
</div>
${chatBlocks || '<p style="color:#8696a0">Menunggu sinkronisasi riwayat pesan...</p>'}
</body>
</html>`;

    fs.writeFileSync(OUTPUT_HTML, html);
    console.log(`✨ Transkrip Chat Diperbarui: ${OUTPUT_HTML} (${store.messages.length} pesan)`);
}

function escapeHtml(str) {
    return String(str).replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');
}

startSync();
