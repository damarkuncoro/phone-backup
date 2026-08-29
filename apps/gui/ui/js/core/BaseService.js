import { api } from './api.js';

/**
 * Base Service untuk menyediakan fungsionalitas umum bagi service lainnya.
 * Menerapkan prinsip DRY untuk pemanggilan API.
 */
export class BaseService {
    static async call(command, args = {}) {
        try {
            return await api.invoke(command, args);
        } catch (err) {
            console.error(`API Error [${command}]:`, err);
            throw err;
        }
    }
}
