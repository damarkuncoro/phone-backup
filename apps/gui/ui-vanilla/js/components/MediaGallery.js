import './MediaCard.js';

export class MediaGallery extends HTMLElement {
    constructor() {
        super();
        this._files = [];
        this.innerHTML = `
            <div class="space-y-8 p-8">
                <div id="gallery-grid" class="grid grid-cols-2 md:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-6">
                </div>
            </div>
        `;
    }

    set media(files) {
        this._files = files.filter(f => f.mime_type.startsWith('image/') || f.mime_type.startsWith('video/'));
        this.render();
    }

    render() {
        const grid = this.querySelector('#gallery-grid');
        if (this._files.length === 0) {
            grid.innerHTML = `
                <div class="col-span-full py-32 text-center flex flex-col items-center">
                    <div class="text-4xl mb-4 opacity-20">🖼️</div>
                    <p class="text-slate-400 font-black text-xs uppercase tracking-widest">No Media Found in this Snapshot</p>
                </div>
            `;
            return;
        }

        grid.innerHTML = "";
        this._files.forEach(file => {
            const card = document.createElement('pb-media-card');
            card.data = file;
            grid.appendChild(card);
        });
    }
}
customElements.define('pb-media-grid', MediaGallery);
