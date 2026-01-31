import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Trash2, Eye, Tag } from "lucide-react";
import { formatDistanceToNow } from "date-fns";
import type { Version } from "@/types";
import { FeedbackStats } from "@/components/feedback/FeedbackStats";

interface VersionCardProps {
  version: Version;
  tags: string[];
  onView: () => void;
  onDelete: () => void;
  onAddTag: () => void;
  onDeleteTag: (tagName: string) => void;
}

export function VersionCard({
  version,
  tags,
  onView,
  onDelete,
  onAddTag,
  onDeleteTag,
}: VersionCardProps) {
  const handleDeleteTag = (tagName: string) => {
    if (confirm(`Remove tag "${tagName}" from this version?`)) {
      onDeleteTag(tagName);
    }
  };

  return (
    <div
      className="border rounded-lg p-4 hover:border-primary/50 transition-colors cursor-pointer"
      onClick={onView}
    >
      <div className="flex items-start justify-between mb-2">
        <div className="flex items-center gap-2 flex-wrap">
          <Badge variant="outline">v{version.version}</Badge>
          {tags.map((tag) => (
            <Badge
              key={tag}
              variant="secondary"
              className="flex items-center gap-1 cursor-pointer hover:bg-secondary/80"
              onClick={(e) => {
                e.stopPropagation();
                handleDeleteTag(tag);
              }}
            >
              {tag}
              <span className="text-xs opacity-50">×</span>
            </Badge>
          ))}
        </div>
        <div className="flex items-center gap-1">
          <Button
            variant="ghost"
            size="icon"
            onClick={(e) => {
              e.stopPropagation();
              onAddTag();
            }}
          >
            <Tag className="h-4 w-4" />
          </Button>
          <Button
            variant="ghost"
            size="icon"
            onClick={(e) => {
              e.stopPropagation();
              onView();
            }}
          >
            <Eye className="h-4 w-4" />
          </Button>
          <Button
            variant="ghost"
            size="icon"
            onClick={(e) => {
              e.stopPropagation();
              onDelete();
            }}
          >
            <Trash2 className="h-4 w-4" />
          </Button>
        </div>
      </div>
      {version.changelog && (
        <p className="text-sm text-muted-foreground mb-2">
          {version.changelog}
        </p>
      )}
      <div className="flex items-center justify-between">
        <span className="text-xs text-muted-foreground">
          {formatDistanceToNow(new Date(version.created_at), {
            addSuffix: true,
          })}
        </span>
        <FeedbackStats
          averageRating={version.average_rating}
          count={version.feedback_count}
          compact
        />
      </div>
    </div>
  );
}
