export class StatCard extends HTMLElement {
    constructor() {
        super();
        this.render();
    }

    static get observedAttributes() { return ['label', 'value', 'subtext']; }

    attributeChangedCallback() {
        this.update();
    }

    update() {
        if (!this.querySelector('#label')) return;
        this.querySelector('#label').textContent = this.getAttribute('label');
        this.querySelector('#value').textContent = this.getAttribute('value');
        this.querySelector('#subtext').textContent = this.getAttribute('subtext');
    }

    render() {
        this.innerHTML = `
            <div class="glass-card p-6 rounded-2xl shadow-sm border border-slate-100 bg-white/80">
                <p class="text-xs font-bold text-slate-400 uppercase tracking-widest mb-1" id="label"></p>
                <h3 id="value" class="text-4xl font-bold text-indigo-600">-</h3>
                <p id="subtext" class="text-sm text-slate-500 mt-2"></p>
            </div>
        `;
    }
}
customElements.define('pb-stat', StatCard);
