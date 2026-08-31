export class Notification extends HTMLElement {
    constructor() {
        super();
        this.innerHTML = `
            <div id="container" class="fixed top-6 right-6 z-[200] flex flex-col gap-3 pointer-events-none"></div>
        `;
    }

    show(message, type = 'info') {
        const container = this.querySelector('#container');
        const toast = document.createElement('div');

        const colors = {
            success: 'bg-green-600',
            error: 'bg-red-600',
            info: 'bg-indigo-600'
        };

        toast.className = `${colors[type] || colors.info} text-white px-6 py-3 rounded-xl shadow-2xl transform translate-x-full transition-all duration-300 pointer-events-auto flex items-center gap-3`;
        toast.innerHTML = `
            <span class="font-bold text-sm">${message}</span>
            <button class="text-white/50 hover:text-white">&times;</button>
        `;

        container.appendChild(toast);

        // Animation entry
        setTimeout(() => toast.classList.remove('translate-x-full'), 10);

        // Auto remove
        const remove = () => {
            toast.classList.add('translate-x-full', 'opacity-0');
            setTimeout(() => toast.remove(), 300);
        };

        toast.querySelector('button').onclick = remove;
        setTimeout(remove, 4000);
    }
}
customElements.define('pb-notification', Notification);
