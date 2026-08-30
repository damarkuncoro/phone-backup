/**
 * Utilities & Helper functions.
 * Mengikuti prinsip DRY (Don't Repeat Yourself).
 */
export const utils = {
    formatBytes(bytes, decimals = 2) {
        if (bytes === 0) return '0 B';
        const k = 1024;
        const dm = decimals < 0 ? 0 : decimals;
        const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
    },

    getSafeId(data) {
        if (!data) return "";
        if (typeof data === 'string') return data;
        if (data.id && typeof data.id === 'string') return data.id;
        if (data.id && typeof data.id === 'object' && data.id[0]) return data.id[0];
        return String(data.id || "");
    }
};
