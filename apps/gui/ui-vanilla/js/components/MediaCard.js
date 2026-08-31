export class MediaCard extends HTMLElement {
    set data(file) {
        const info = file.media_info || {};
        const isVideo = file.mime_type.startsWith('video/');

        this.className = "group relative bg-slate-900 rounded-3xl overflow-hidden shadow-lg hover:shadow-2xl transition-all duration-500 hover:-translate-y-2 aspect-square";

        this.innerHTML = `
            <!-- Placeholder Visual -->
            <div class="absolute inset-0 flex items-center justify-center bg-gradient-to-br from-indigo-500/20 to-purple-500/20 group-hover:scale-110 transition-transform duration-700">
                <span class="text-4xl opacity-30">${isVideo ? '🎬' : '📸'}</span>
            </div>

            <!-- Overlay Info -->
            <div class="absolute inset-0 bg-gradient-to-t from-slate-900 via-transparent to-transparent opacity-80 group-hover:opacity-100 transition-opacity"></div>

            <div class="absolute bottom-0 left-0 right-0 p-5 text-white transform translate-y-2 group-hover:translate-y-0 transition-transform">
                <p class="text-[10px] font-black text-indigo-400 uppercase tracking-[0.2em] mb-1">
                    ${info.camera_model || 'Standard Media'}
                </p>
                <h4 class="text-xs font-bold truncate w-full mb-2">${file.name}</h4>

                <div class="flex items-center justify-between opacity-0 group-hover:opacity-100 transition-opacity duration-300">
                    <span class="text-[9px] font-mono text-slate-400">${(file.size_bytes/1024/1024).toFixed(2)} MB</span>
                    <button class="restore-btn bg-white/10 hover:bg-white text-white hover:text-indigo-950 p-2 rounded-xl backdrop-blur-md transition-all">
                        <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-width="3" d="M4 16v1a2 2 0 002 2h10a2 2 0 002-2v-1m-4-4l-4 4m0 0l-4-4m4 4V4"/></svg>
                    </button>
                </div>
            </div>

            <!-- Date Tag -->
            ${info.taken_at ? `
                <div class="absolute top-4 left-4 bg-white/10 backdrop-blur-md px-3 py-1 rounded-full border border-white/10">
                    <p class="text-[8px] font-black text-white uppercase">${new Date(info.taken_at).toLocaleDateString()}</p>
                </div>
            ` : ''}
        `;

        this.querySelector('.restore-btn').onclick = (e) => {
            e.stopPropagation();
            window.dispatchEvent(new CustomEvent('restore-file', {
                detail: { path: file.path }
            }));
        };
    }
}
customElements.define('pb-media-card', MediaCard);
