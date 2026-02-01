import { useState, useEffect } from "react";
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

interface AcceptSuggestionDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onSubmit: (newVersion: string, changelog: string | null) => Promise<void>;
  isLoading: boolean;
  currentVersion: string;
}

function incrementPatchVersion(version: string): string {
  const parts = version.split(".");
  if (parts.length !== 3) return version;

  const [major, minor, patch] = parts;
  const newPatch = parseInt(patch, 10) + 1;
  return `${major}.${minor}.${newPatch}`;
}

export function AcceptSuggestionDialog({
  open,
  onOpenChange,
  onSubmit,
  isLoading,
  currentVersion,
}: AcceptSuggestionDialogProps) {
  const [newVersion, setNewVersion] = useState("");
  const [changelog, setChangelog] = useState("");

  useEffect(() => {
    if (open) {
      setNewVersion(incrementPatchVersion(currentVersion));
    }
  }, [open, currentVersion]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newVersion.trim()) return;

    await onSubmit(newVersion.trim(), changelog.trim() || null);
    setNewVersion("");
    setChangelog("");
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-md">
        <form onSubmit={handleSubmit}>
          <DialogHeader>
            <DialogTitle>Accept Improvement</DialogTitle>
            <DialogDescription>
              Create a new version with the suggested improvements
            </DialogDescription>
          </DialogHeader>
          <div className="grid gap-4 py-4">
            <div className="grid gap-2">
              <Label htmlFor="version">New Version *</Label>
              <Input
                id="version"
                value={newVersion}
                onChange={(e) => setNewVersion(e.target.value)}
                placeholder={incrementPatchVersion(currentVersion)}
                disabled={isLoading}
                autoFocus
              />
            </div>
            <div className="grid gap-2">
              <Label htmlFor="changelog">Changelog</Label>
              <Textarea
                id="changelog"
                value={changelog}
                onChange={(e) => setChangelog(e.target.value)}
                placeholder="What changed in this version..."
                disabled={isLoading}
                rows={3}
              />
            </div>
          </div>
          <DialogFooter>
            <Button
              type="button"
              variant="outline"
              onClick={() => onOpenChange(false)}
              disabled={isLoading}
            >
              Cancel
            </Button>
            <Button type="submit" disabled={isLoading || !newVersion.trim()}>
              {isLoading ? "Creating..." : "Accept & Create Version"}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
