import type { Prompt } from "@/types";
import { formatDistanceToNow } from "date-fns";

interface PromptsListProps {
  prompts: Prompt[];
  onPromptClick: (promptId: string) => void;
}

export function PromptsList({ prompts, onPromptClick }: PromptsListProps) {
  return (
    <div className="bg-white border rounded">
      {prompts.map((prompt) => {
        const versionCount = prompt.versions?.length || 0;
        const latestTag = prompt.tags?.find((tag) => tag.name === "latest");
        const latestVersion = latestTag
          ? prompt.versions?.find((v) => v.id === latestTag.version_id)
          : null;

        return (
          <button
            key={prompt.id}
            onClick={() => onPromptClick(prompt.id)}
            className="w-full text-left px-6 py-4 hover:bg-neutral-50 transition-colors border-b last:border-b-0 flex items-center justify-between"
          >
            <div className="flex-1">
              <div className="font-medium mb-1">{prompt.name}</div>
              {prompt.description && (
                <div className="text-sm text-muted-foreground line-clamp-1">
                  {prompt.description}
                </div>
              )}
            </div>
            <div className="flex items-center gap-8 text-sm text-muted-foreground">
              {latestVersion && <span>v{latestVersion.version}</span>}
              <span>
                {versionCount} {versionCount === 1 ? "version" : "versions"}
              </span>
              <span className="w-32 text-right">
                {formatDistanceToNow(new Date(prompt.created_at), {
                  addSuffix: true,
                })}
              </span>
            </div>
          </button>
        );
      })}
    </div>
  );
}
