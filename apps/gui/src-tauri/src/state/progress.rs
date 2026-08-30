use ports::ProgressPort;
use socketioxide::SocketIo;
use tauri::Emitter;

pub struct CombinedProgress {
    pub app_handle: tauri::AppHandle,
    pub io: SocketIo,
}

impl ProgressPort for CombinedProgress {
    fn start(&self, total: u64, message: &str) {
        let payload = serde_json::json!({ "type": "start", "total": total, "message": message });
        let _ = self.app_handle.emit("progress", &payload);
        let _ = self.io.emit("progress", &payload);
    }
    fn inc(&self, amount: u64, message: &str) {
        let payload = serde_json::json!({ "type": "inc", "amount": amount, "message": message });
        let _ = self.app_handle.emit("progress", &payload);
        let _ = self.io.emit("progress", &payload);
    }
    fn finish(&self, message: &str) {
        let payload = serde_json::json!({ "type": "finish", "message": message });
        let _ = self.app_handle.emit("progress", &payload);
        let _ = self.io.emit("progress", &payload);
    }
    fn error(&self, message: &str) {
        let payload = serde_json::json!({ "type": "error", "message": message });
        let _ = self.app_handle.emit("progress", &payload);
        let _ = self.io.emit("progress", &payload);
    }
    fn log(&self, message: &str) {
        let payload = serde_json::json!({ "type": "log", "message": message });
        let _ = self.app_handle.emit("progress", &payload);
        let _ = self.io.emit("progress", &payload);
    }
}
