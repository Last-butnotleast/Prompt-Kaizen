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
import { Textarea } from "@/components/ui/textarea";
import type { CreateVersionRequest } from "@/types";

interface CreateVersionDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onSubmit: (data: CreateVersionRequest) => Promise<void>;
  isLoading: boolean;
  latestVersion?: string;
}

export function CreateVersionDialog({
  open,
  onOpenChange,
  onSubmit,
  isLoading,
  latestVersion,
}: CreateVersionDialogProps) {
  const [version, setVersion] = useState("");
  const [content, setContent] = useState("");
  const [changelog, setChangelog] = useState("");
  const [error, setError] = useState("");

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");

    if (!version.trim()) {
      setError("Version number is required");
      return;
    }

    if (!content.trim()) {
      setError("Content is required");
      return;
    }

    const versionRegex = /^\d+\.\d+\.\d+$/;
    if (!versionRegex.test(version.trim())) {
      setError("Version must be in format: 0.0.1");
      return;
    }

    try {
      await onSubmit({
        version: version.trim(),
        content: content.trim(),
        changelog: changelog.trim() || null,
      });
      setVersion("");
      setContent("");
      setChangelog("");
    } catch (err) {
      setError("Failed to create version");
    }
  };

  const handleOpenChange = (newOpen: boolean) => {
    if (!isLoading) {
      onOpenChange(newOpen);
      if (!newOpen) {
        setVersion("");
        setContent("");
        setChangelog("");
        setError("");
      }
    }
  };

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogContent className="max-w-3xl max-h-[90vh] overflow-y-auto">
        <form onSubmit={handleSubmit}>
          <DialogHeader>
            <DialogTitle>Create New Version</DialogTitle>
            <DialogDescription>
              {latestVersion
                ? `Latest version: ${latestVersion}. Create a new version with updated content.`
                : "Create the first version of this prompt."}
            </DialogDescription>
          </DialogHeader>
          <div className="grid gap-4 py-4">
            <div className="grid gap-2">
              <Label htmlFor="version">Version Number *</Label>
              <Input
                id="version"
                value={version}
                onChange={(e) => setVersion(e.target.value)}
                placeholder="0.0.1"
                disabled={isLoading}
                autoFocus
              />
              <p className="text-xs text-muted-foreground">
                Use semantic versioning (e.g., 0.0.1, 1.2.3)
              </p>
            </div>
            <div className="grid gap-2">
              <Label htmlFor="content">Prompt Content *</Label>
              <Textarea
                id="content"
                value={content}
                onChange={(e) => setContent(e.target.value)}
                placeholder="Enter your prompt content here..."
                disabled={isLoading}
                rows={12}
                className="font-mono text-sm"
              />
            </div>
            <div className="grid gap-2">
              <Label htmlFor="changelog">Changelog</Label>
              <Textarea
                id="changelog"
                value={changelog}
                onChange={(e) => setChangelog(e.target.value)}
                placeholder="What changed in this version?"
                disabled={isLoading}
                rows={3}
              />
            </div>
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
              {isLoading ? "Creating..." : "Create Version"}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
