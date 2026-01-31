import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Copy, Check, MessageSquare } from "lucide-react";
import { useState } from "react";
import { formatDistanceToNow } from "date-fns";
import type { Version, Feedback } from "@/types";
import { FeedbackStats } from "@/components/feedback/FeedbackStats";
import { FeedbackList } from "@/components/feedback/FeedbackList";

interface VersionDetailDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  version: Version | null;
  tags?: string[];
  feedback?: Feedback[];
  onAddFeedback: () => void;
  onDeleteFeedback: (feedbackId: string) => void;
}

export function VersionDetailDialog({
  open,
  onOpenChange,
  version,
  tags = [],
  feedback = [],
  onAddFeedback,
  onDeleteFeedback,
}: VersionDetailDialogProps) {
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

        <Tabs
          defaultValue="content"
          className="flex-1 flex flex-col overflow-hidden"
        >
          <TabsList>
            <TabsTrigger value="content">Content</TabsTrigger>
            <TabsTrigger value="feedback">
              Feedback ({feedback.length})
            </TabsTrigger>
          </TabsList>

          <TabsContent value="content" className="flex-1 overflow-y-auto mt-4">
            <pre className="bg-muted p-4 rounded-lg text-sm font-mono whitespace-pre-wrap break-words">
              {version.content}
            </pre>
          </TabsContent>

          <TabsContent value="feedback" className="flex-1 overflow-y-auto mt-4">
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <FeedbackStats
                  averageRating={version.average_rating}
                  count={version.feedback_count}
                />
                <Button size="sm" onClick={onAddFeedback}>
                  <MessageSquare className="h-4 w-4 mr-2" />
                  Add Feedback
                </Button>
              </div>
              <FeedbackList feedback={feedback} onDelete={onDeleteFeedback} />
            </div>
          </TabsContent>
        </Tabs>

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
      </DialogContent>
    </Dialog>
  );
}
