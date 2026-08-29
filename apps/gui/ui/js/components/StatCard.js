export class StatCard extends HTMLElement {
    constructor() {
        super();
        this.innerHTML = `
            <div class="glass-card p-6 rounded-2xl shadow-sm border border-slate-100 bg-white/80">
                <p class="text-xs font-bold text-slate-400 uppercase tracking-widest mb-1" id="label"></p>
                <h3 id="value" class="text-4xl font-bold text-indigo-600">-</h3>
                <p id="subtext" class="text-sm text-slate-500 mt-2"></p>
            </div>
        `;
    }

    static get observedAttributes() { return ['label', 'value', 'subtext']; }

    attributeChangedCallback(name, oldVal, newVal) {
        if (name === 'label') this.querySelector('#label').textContent = newVal;
        if (name === 'value') this.querySelector('#value').textContent = newVal;
        if (name === 'subtext') this.querySelector('#subtext').textContent = newVal;
    }
}
customElements.define('pb-stat', StatCard);
