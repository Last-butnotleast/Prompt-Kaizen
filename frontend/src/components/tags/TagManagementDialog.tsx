import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { X } from "lucide-react";
import type { Prompt } from "@/types";

interface TagManagementDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  prompt: Prompt;
  onDeleteTag: (tagName: string) => Promise<void>;
}

export function TagManagementDialog({
  open,
  onOpenChange,
  prompt,
  onDeleteTag,
}: TagManagementDialogProps) {
  const handleDeleteTag = async (tagName: string) => {
    if (confirm(`Remove tag "${tagName}"? The version will not be deleted.`)) {
      await onDeleteTag(tagName);
    }
  };

  const getVersionNumber = (versionId: string): string => {
    return prompt.versions.find((v) => v.id === versionId)?.version || "?";
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-2xl">
        <DialogHeader>
          <DialogTitle>Manage Tags</DialogTitle>
          <DialogDescription>
            View and remove tags. Tags can be added to individual versions.
          </DialogDescription>
        </DialogHeader>
        <div className="py-4">
          {prompt.tags.length === 0 ? (
            <div className="text-center py-8 text-muted-foreground text-sm">
              No tags yet. Add tags to versions for easy reference.
            </div>
          ) : (
            <div className="space-y-3">
              {prompt.tags.map((tag) => (
                <div
                  key={tag.name}
                  className="flex items-center justify-between p-3 border rounded-lg"
                >
                  <div className="flex items-center gap-3">
                    <Badge variant="secondary">{tag.name}</Badge>
                    <span className="text-sm text-muted-foreground">
                      → v{getVersionNumber(tag.version_id)}
                    </span>
                  </div>
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => handleDeleteTag(tag.name)}
                  >
                    <X className="h-4 w-4" />
                  </Button>
                </div>
              ))}
            </div>
          )}
        </div>
      </DialogContent>
    </Dialog>
  );
}
