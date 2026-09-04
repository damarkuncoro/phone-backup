import { useState } from 'react';
import { Download } from 'lucide-react';
import { type CallLogItem } from './callsUtils';
import { CallStatsPane } from './CallStatsPane';
import { CallListPane } from './CallListPane';

interface CallsExplorerProps {
  calls: CallLogItem[];
  snapshotId: string;
}

export function CallsExplorer({ calls, snapshotId }: CallsExplorerProps) {
  const [filterType, setFilterType] = useState<'all' | 'incoming' | 'outgoing' | 'missed'>('all');
  const [exporting, setExporting] = useState(false);

  const handleExportJson = () => {
    setExporting(true);
    try {
      const dataStr = "data:text/json;charset=utf-8," + encodeURIComponent(JSON.stringify(calls, null, 2));
      const downloadAnchor = document.createElement('a');
      downloadAnchor.setAttribute("href", dataStr);
      downloadAnchor.setAttribute("download", `calls_${snapshotId.substring(0, 8)}.json`);
      document.body.appendChild(downloadAnchor);
      downloadAnchor.click();
      downloadAnchor.remove();
    } catch (e) {
      console.error("Export call logs failed", e);
    } finally {
      setExporting(false);
    }
  };

  return (
    <div className="flex flex-col h-full gap-4">
      {/* Top Bar with Stats and Export */}
      <div className="flex flex-col gap-3">
        <div className="flex items-center justify-between">
          <div>
            <h2 className="text-base font-black text-slate-900 tracking-tight">Riwayat Panggilan Telepon</h2>
            <p className="text-[10px] text-slate-400 font-bold uppercase tracking-wider">
              {calls.length} Total Catatan Panggilan Terekstrak
            </p>
          </div>
          <button
            onClick={handleExportJson}
            disabled={exporting || calls.length === 0}
            className="flex items-center gap-2 px-4 py-2 bg-slate-900 hover:bg-slate-800 text-white rounded-xl text-xs font-bold transition-all shadow-sm active:scale-95 disabled:opacity-40"
          >
            <Download className="w-3.5 h-3.5" />
            <span>Export JSON</span>
          </button>
        </div>

        {/* Aggregate KPI Stats */}
        <CallStatsPane calls={calls} />
      </div>

      {/* Main List Pane */}
      <div className="flex-1 min-h-0">
        <CallListPane
          calls={calls}
          filterType={filterType}
          onFilterChange={setFilterType}
        />
      </div>
    </div>
  );
}
