import { useState, useMemo } from "react";
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
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";
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
  const [contentType, setContentType] = useState<"static" | "template">(
    "static",
  );
  const [error, setError] = useState("");

  const extractedVariables = useMemo(() => {
    if (contentType !== "template") return [];
    const matches = content.matchAll(/\{\{(\w+)\}\}/g);
    return Array.from(new Set(Array.from(matches, (m) => m[1])));
  }, [content, contentType]);

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

    if (contentType === "template" && extractedVariables.length === 0) {
      setError(
        "Template must contain at least one variable in {{variable}} format",
      );
      return;
    }

    try {
      await onSubmit({
        version: version.trim(),
        content: content.trim(),
        content_type: contentType,
        variables: contentType === "template" ? extractedVariables : null,
        changelog: changelog.trim() || null,
      });
      setVersion("");
      setContent("");
      setChangelog("");
      setContentType("static");
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
        setContentType("static");
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
              <Label>Content Type *</Label>
              <RadioGroup
                value={contentType}
                onValueChange={(value) =>
                  setContentType(value as "static" | "template")
                }
                disabled={isLoading}
              >
                <div className="flex items-center space-x-2">
                  <RadioGroupItem value="static" id="static" />
                  <Label
                    htmlFor="static"
                    className="font-normal cursor-pointer"
                  >
                    Static - Fixed content without variables
                  </Label>
                </div>
                <div className="flex items-center space-x-2">
                  <RadioGroupItem value="template" id="template" />
                  <Label
                    htmlFor="template"
                    className="font-normal cursor-pointer"
                  >
                    Template - Contains variables like {`{{name}}`}
                  </Label>
                </div>
              </RadioGroup>
            </div>

            <div className="grid gap-2">
              <Label htmlFor="content">Prompt Content *</Label>
              <Textarea
                id="content"
                value={content}
                onChange={(e) => setContent(e.target.value)}
                placeholder={
                  contentType === "template"
                    ? "Enter your prompt with variables like {{name}}, {{topic}}..."
                    : "Enter your prompt content here..."
                }
                disabled={isLoading}
                rows={12}
                className="font-mono text-sm"
              />
              {contentType === "template" && extractedVariables.length > 0 && (
                <div className="text-xs text-muted-foreground">
                  <strong>Detected variables:</strong>{" "}
                  {extractedVariables.join(", ")}
                </div>
              )}
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
