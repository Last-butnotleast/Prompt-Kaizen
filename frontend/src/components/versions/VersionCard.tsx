import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Trash2, Star, Eye } from "lucide-react";
import { formatDistanceToNow } from "date-fns";
import type { Version } from "@/types";

interface VersionCardProps {
  version: Version;
  tags: string[];
  onView: () => void;
  onDelete: () => void;
}

export function VersionCard({
  version,
  tags,
  onView,
  onDelete,
}: VersionCardProps) {
  return (
    <div className="border rounded-lg p-4">
      <div className="flex items-start justify-between mb-2">
        <div className="flex items-center gap-2 flex-wrap">
          <Badge variant="outline">v{version.version}</Badge>
          {tags.map((tag) => (
            <Badge key={tag} variant="secondary">
              {tag}
            </Badge>
          ))}
        </div>
        <div className="flex items-center gap-1">
          <Button variant="ghost" size="icon" onClick={onView}>
            <Eye className="h-4 w-4" />
          </Button>
          <Button variant="ghost" size="icon" onClick={onDelete}>
            <Trash2 className="h-4 w-4" />
          </Button>
        </div>
      </div>
      {version.changelog && (
        <p className="text-sm text-muted-foreground mb-2">
          {version.changelog}
        </p>
      )}
      <div className="flex items-center justify-between text-xs text-muted-foreground">
        <span>
          {formatDistanceToNow(new Date(version.created_at), {
            addSuffix: true,
          })}
        </span>
        {version.feedback_count > 0 && (
          <div className="flex items-center gap-1">
            <Star className="h-3 w-3 fill-current" />
            <span>{version.average_rating?.toFixed(1)}</span>
            <span>({version.feedback_count})</span>
          </div>
        )}
      </div>
    </div>
  );
}
