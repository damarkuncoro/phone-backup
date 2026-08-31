import { invoke as tauriInvoke } from "@tauri-apps/api/core";
import { listen, type Event, type UnlistenFn } from "@tauri-apps/api/event";

/**
 * Wrapper untuk invoke yang memberikan feedback lebih baik jika gagal.
 */
export async function safeInvoke<T>(
    command: string,
    args: Record<string, any> = {},
    options: { silent?: boolean } = {}
): Promise<T> {
  try {
    if (!(window as any).__TAURI_INTERNALS__) {
      if (!options.silent) console.warn(`[IPC Mock] Command "${command}" called outside Tauri`);

      if (command === 'get_devices') return [] as any;
      if (command === 'get_all_known_devices') return [] as any;
      if (command === 'get_snapshots') return [] as any;
      if (command === 'get_storage_stats') return { total_logical_bytes: 0, total_deduped_bytes: 0, total_snapshots: 0 } as any;

      throw new Error("Tauri environment not detected.");
    }

    return await tauriInvoke<T>(command, args);
  } catch (error) {
    if (!options.silent) {
        console.error(`[IPC Error] "${command}":`, error);
    }
    throw error;
  }
}

/**
 * Safe listener wrapper for Tauri events in React components.
 */
export function safeListen<T>(
    eventName: string,
    handler: (event: Event<T>) => void
): () => void {
    let unlistenFn: UnlistenFn | null = null;
    let mounted = true;

    // We don't return the promise, we handle the cleanup internally
    const setup = async () => {
        try {
            const fn = await listen<T>(eventName, (event) => {
                if (mounted) handler(event);
            });
            if (mounted) {
                unlistenFn = fn;
            } else {
                try { fn(); } catch (_) {}
            }
        } catch (err) {
            console.error(`[Listen Error] Failed "${eventName}":`, err);
        }
    };

    setup();

    return () => {
        mounted = false;
        if (unlistenFn) {
            try { unlistenFn(); } catch (_) {}
        }
    };
}
