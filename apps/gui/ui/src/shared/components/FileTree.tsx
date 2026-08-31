import { useState, useMemo } from 'react';
import { Folder, FileText, ChevronRight, ChevronDown, CheckSquare, Square } from 'lucide-react';
import { type FileEntry } from '@/services/deviceService';
import { cn } from "../lib/utils";
import { formatBytes } from '@/shared/lib/formatters';
import { buildFileTree, type TreeNode, shouldShowNode } from '../../features/explorer/lib/explorerUtils';

interface FileTreeProps {
  files: FileEntry[];
  searchQuery: string;
  selectedPaths: Set<string>;
  onToggle: (path: string, isFolder: boolean, childrenPaths: string[]) => void;
}

export function FileTree({ files, searchQuery, selectedPaths, onToggle }: FileTreeProps) {
  const tree = useMemo(() => buildFileTree(files), [files]);

  const sortedKeys = (children: Record<string, TreeNode>) => {
    return Object.keys(children).sort((a, b) => {
        const nodeA = children[a];
        const nodeB = children[b];
        if (nodeA.type !== nodeB.type) return nodeA.type === 'folder' ? -1 : 1;
        return a.localeCompare(b);
    });
  };

  return (
    <div className="space-y-1">
      {sortedKeys(tree.children).map(key => (
        <FileTreeItem
            key={tree.children[key].path}
            node={tree.children[key]}
            depth={0}
            searchQuery={searchQuery}
            selectedPaths={selectedPaths}
            onToggle={onToggle}
        />
      ))}
    </div>
  );
}

function FileTreeItem({ node, depth, searchQuery, selectedPaths, onToggle }: {
    node: TreeNode,
    depth: number,
    searchQuery: string,
    selectedPaths: Set<string>,
    onToggle: (path: string, isFolder: boolean, childrenPaths: string[]) => void
}) {
  const [isExpanded, setIsExpanded] = useState(depth < 2);
  const isSelected = selectedPaths.has(node.path);

  const getAllChildPaths = (n: TreeNode): string[] => {
      let paths: string[] = [];
      Object.values(n.children).forEach(child => {
          paths.push(child.path);
          paths = paths.concat(getAllChildPaths(child));
      });
      return paths;
  };

  const isVisible = useMemo(() => shouldShowNode(node, searchQuery), [node, searchQuery]);

  if (!isVisible) return null;

  return (
    <div className="select-none">
      <div
        className={cn(
            "flex items-center gap-2 p-2 rounded-xl transition-all group",
            node.type === 'folder' ? "hover:bg-slate-50" : "hover:bg-indigo-50/30",
            isSelected ? "bg-indigo-50/50" : ""
        )}
        style={{ paddingLeft: `${depth * 1.5 + 0.5}rem` }}
      >
        <div className="w-4 h-4 flex items-center justify-center cursor-pointer" onClick={() => setIsExpanded(!isExpanded)}>
            {node.type === 'folder' && (
                isExpanded ? <ChevronDown className="w-3 h-3 text-slate-400" /> : <ChevronRight className="w-3 h-3 text-slate-400" />
            )}
        </div>

        <button
            onClick={() => onToggle(node.path, node.type === 'folder', getAllChildPaths(node))}
            className={cn("transition-all", isSelected ? "text-indigo-600" : "text-slate-300 group-hover:text-slate-400")}
        >
            {isSelected ? <CheckSquare className="w-4 h-4" /> : <Square className="w-4 h-4" />}
        </button>

        <div
            onClick={() => node.type === 'folder' ? setIsExpanded(!isExpanded) : onToggle(node.path, false, [])}
            className="flex items-center gap-2 flex-1 min-w-0 cursor-pointer"
        >
            <div className={cn(
                "w-8 h-8 rounded-lg flex items-center justify-center shrink-0",
                node.type === 'folder' ? (isSelected ? "bg-indigo-600 text-white" : "bg-indigo-50 text-indigo-500") : "bg-slate-50 text-slate-400"
            )}>
                {node.type === 'folder' ? <Folder className="w-4 h-4" /> : <FileText className="w-4 h-4" />}
            </div>

            <div className="flex-1 min-w-0">
                <p className={cn(
                    "text-xs truncate",
                    node.type === 'folder' ? "font-black text-slate-700" : "font-medium text-slate-600"
                )}>
                    {node.name}
                </p>
            </div>

            {node.type === 'file' && (
                <span className="text-[10px] font-mono text-slate-400 opacity-0 group-hover:opacity-100 transition-all mr-2">
                    {formatBytes(node.size)}
                </span>
            )}
        </div>
      </div>

      {node.type === 'folder' && (isExpanded || searchQuery) && (
        <div className="animate-in fade-in slide-in-from-top-1 duration-200">
          {Object.keys(node.children)
            .sort((a, b) => {
              const nodeA = node.children[a];
              const nodeB = node.children[b];
              if (nodeA.type !== nodeB.type) return nodeA.type === 'folder' ? -1 : 1;
              return a.localeCompare(b);
            })
            .map(key => (
              <FileTreeItem
                key={node.children[key].path}
                node={node.children[key]}
                depth={depth + 1}
                searchQuery={searchQuery}
                selectedPaths={selectedPaths}
                onToggle={onToggle}
              />
            ))}
        </div>
      )}
    </div>
  );
}
