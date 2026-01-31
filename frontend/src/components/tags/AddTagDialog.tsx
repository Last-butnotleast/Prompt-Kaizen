import { useState } from "react";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import type { TagVersionRequest } from "@/types";

interface AddTagDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onSubmit: (data: TagVersionRequest) => Promise<void>;
  isLoading: boolean;
  versionId: string;
  versionNumber: string;
  existingTags: string[];
}

export function AddTagDialog({
  open,
  onOpenChange,
  onSubmit,
  isLoading,
  versionId,
  versionNumber,
  existingTags,
}: AddTagDialogProps) {
  const [tagName, setTagName] = useState("");
  const [error, setError] = useState("");

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");

    if (!tagName.trim()) {
      setError("Tag name is required");
      return;
    }

    const normalized = tagName.trim().toLowerCase();
    if (existingTags.some((t) => t.toLowerCase() === normalized)) {
      setError("This tag already exists on this version");
      return;
    }

    try {
      await onSubmit({
        tag_name: tagName.trim(),
        version_id: versionId,
      });
      setTagName("");
    } catch (err) {
      setError("Failed to add tag");
    }
  };

  const handleOpenChange = (newOpen: boolean) => {
    if (!isLoading) {
      onOpenChange(newOpen);
      if (!newOpen) {
        setTagName("");
        setError("");
      }
    }
  };

  const suggestedTags = ["latest", "production", "staging", "experimental"];
  const availableSuggestions = suggestedTags.filter(
    (tag) => !existingTags.some((t) => t.toLowerCase() === tag),
  );

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogContent>
        <form onSubmit={handleSubmit}>
          <DialogHeader>
            <DialogTitle>Add Tag to Version {versionNumber}</DialogTitle>
            <DialogDescription>
              Tag this version for easy reference (e.g., latest, production).
            </DialogDescription>
          </DialogHeader>
          <div className="grid gap-4 py-4">
            <div className="grid gap-2">
              <Label htmlFor="tag-name">Tag Name *</Label>
              <Input
                id="tag-name"
                value={tagName}
                onChange={(e) => setTagName(e.target.value)}
                placeholder="e.g., latest"
                disabled={isLoading}
                autoFocus
              />
            </div>
            {availableSuggestions.length > 0 && (
              <div className="grid gap-2">
                <Label className="text-xs text-muted-foreground">
                  Suggestions
                </Label>
                <div className="flex flex-wrap gap-2">
                  {availableSuggestions.map((tag) => (
                    <Button
                      key={tag}
                      type="button"
                      variant="outline"
                      size="sm"
                      onClick={() => setTagName(tag)}
                      disabled={isLoading}
                    >
                      {tag}
                    </Button>
                  ))}
                </div>
              </div>
            )}
            {error && <div className="text-sm text-destructive">{error}</div>}
          </div>
          <DialogFooter>
            <Button
              type="button"
              variant="outline"
              onClick={() => handleOpenChange(false)}
              disabled={isLoading}
            >
              Cancel
            </Button>
            <Button type="submit" disabled={isLoading}>
              {isLoading ? "Adding..." : "Add Tag"}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
