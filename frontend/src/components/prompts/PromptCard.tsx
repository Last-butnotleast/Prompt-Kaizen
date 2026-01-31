import type { Prompt } from "@/types";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { formatDistanceToNow } from "date-fns";

interface PromptCardProps {
  prompt: Prompt;
  onClick: () => void;
}

export function PromptCard({ prompt, onClick }: PromptCardProps) {
  const versionCount = prompt.versions?.length || 0;
  const latestTag = prompt.tags?.find((tag) => tag.name === "latest");
  const latestVersion = latestTag
    ? prompt.versions?.find((v) => v.id === latestTag.version_id)
    : null;

  return (
    <Card
      className="cursor-pointer hover:border-primary transition-colors"
      onClick={onClick}
    >
      <CardHeader>
        <div className="flex items-start justify-between gap-2">
          <CardTitle className="text-lg">{prompt.name}</CardTitle>
          {latestVersion && (
            <Badge variant="secondary" className="shrink-0">
              v{latestVersion.version}
            </Badge>
          )}
        </div>
        {prompt.description && (
          <CardDescription className="line-clamp-2">
            {prompt.description}
          </CardDescription>
        )}
      </CardHeader>
      <CardContent>
        <div className="flex items-center justify-between text-sm text-muted-foreground">
          <span>
            {versionCount} version{versionCount !== 1 ? "s" : ""}
          </span>
          <span>
            {formatDistanceToNow(new Date(prompt.created_at), {
              addSuffix: true,
            })}
          </span>
        </div>
      </CardContent>
    </Card>
  );
}
