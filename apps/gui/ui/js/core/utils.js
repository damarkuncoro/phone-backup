/**
 * Utility functions for the application.
 */
export const utils = {
    /**
     * Safely extracts a string ID from a Rust-provided ID (could be string or array/tuple).
     */
    getSafeId(obj) {
        if (!obj) return "unknown";
        const id = typeof obj === 'object' && obj.id ? obj.id : obj;
        if (Array.isArray(id)) return id[0];
        return id || "unknown";
    },

    formatBytes(bytes) {
        if (bytes === 0) return '0 B';
        const k = 1024;
        const sizes = ['B', 'KB', 'MB', 'GB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
    }
};
