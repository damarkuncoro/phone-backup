/**
 * Simple Reactive Store.
 * Memastikan semua komponen mendapatkan data yang sama (Consistency).
 */
class Store extends EventTarget {
    constructor() {
        super();
        this.state = {
            devices: [],
            stats: {},
            currentSnapshots: [],
            isEngineBusy: false
        };
    }

    setState(key, value) {
        this.state[key] = value;
        this.dispatchEvent(new CustomEvent('change', { detail: { key, value } }));
    }

    getState() {
        return this.state;
    }
}

export const store = new Store();
