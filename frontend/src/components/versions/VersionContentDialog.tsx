import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Copy, Check } from "lucide-react";
import { useState } from "react";
import { formatDistanceToNow } from "date-fns";
import type { Version } from "@/types";

interface VersionContentDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  version: Version | null;
  tags?: string[];
}

export function VersionContentDialog({
  open,
  onOpenChange,
  version,
  tags = [],
}: VersionContentDialogProps) {
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    if (version) {
      await navigator.clipboard.writeText(version.content);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
  };

  if (!version) return null;

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-4xl max-h-[90vh] flex flex-col">
        <DialogHeader>
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2 flex-wrap">
              <DialogTitle>Version {version.version}</DialogTitle>
              {tags.map((tag) => (
                <Badge key={tag} variant="secondary">
                  {tag}
                </Badge>
              ))}
            </div>
            <Button variant="outline" size="sm" onClick={handleCopy}>
              {copied ? (
                <>
                  <Check className="h-4 w-4 mr-2" />
                  Copied
                </>
              ) : (
                <>
                  <Copy className="h-4 w-4 mr-2" />
                  Copy
                </>
              )}
            </Button>
          </div>
          {version.changelog && (
            <DialogDescription>{version.changelog}</DialogDescription>
          )}
        </DialogHeader>
        <div className="flex-1 overflow-hidden flex flex-col gap-4">
          <div className="flex-1 overflow-y-auto">
            <pre className="bg-muted p-4 rounded-lg text-sm font-mono whitespace-pre-wrap wrap-break-word">
              {version.content}
            </pre>
          </div>
          <div className="text-xs text-muted-foreground border-t pt-4">
            <div className="flex items-center justify-between">
              <span>Digest: {version.digest.substring(0, 16)}...</span>
              <span>
                Created{" "}
                {formatDistanceToNow(new Date(version.created_at), {
                  addSuffix: true,
                })}
              </span>
            </div>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
