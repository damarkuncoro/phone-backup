/**
 * API Bridge untuk komunikasi dengan Rust Backend.
 * Memudahkan Mocking saat melakukan TDD.
 */
export const api = {
    async invoke(cmd, args = {}) {
        if (window.__TAURI__) {
            return await window.__TAURI__.core.invoke(cmd, args);
        }
        console.warn(`Mocking API Call: ${cmd}`, args);
        return null;
    },

    listen(event, callback) {
        if (window.__TAURI__) {
            return window.__TAURI__.event.listen(event, callback);
        }
    }
};
