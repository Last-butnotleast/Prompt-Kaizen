import { useState } from "react";
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
import { Plus } from "lucide-react";
import { formatDistanceToNow } from "date-fns";
import type { Version, Feedback } from "@/types";
import { FeedbackItem } from "@/components/feedback/FeedbackItem";
import { ImprovementsList } from "@/components/improvements/ImprovementList";
import { AcceptSuggestionDialog } from "@/components/improvements/AcceptSuggestionDialog";
import {
  useAcceptImprovementSuggestion,
  useDeclineImprovementSuggestion,
} from "@/api/hooks/improvements";

interface VersionDetailDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  version: Version;
  tags: string[];
  feedback: Feedback[];
  onAddFeedback: () => void;
  onDeleteFeedback: (feedbackId: string) => void;
  promptId: string;
}

export function VersionDetailDialog({
  open,
  onOpenChange,
  version,
  tags,
  feedback,
  onAddFeedback,
  onDeleteFeedback,
  promptId,
}: VersionDetailDialogProps) {
  const [acceptDialogOpen, setAcceptDialogOpen] = useState(false);
  const [selectedSuggestionId, setSelectedSuggestionId] = useState<
    string | null
  >(null);

  const acceptSuggestion = useAcceptImprovementSuggestion(promptId, version.id);
  const declineSuggestion = useDeclineImprovementSuggestion(
    promptId,
    version.id,
  );

  const handleAcceptClick = (suggestionId: string) => {
    setSelectedSuggestionId(suggestionId);
    setAcceptDialogOpen(true);
  };

  const handleAcceptSubmit = async (
    newVersion: string,
    changelog: string | null,
  ) => {
    if (!selectedSuggestionId) return;
    await acceptSuggestion.mutateAsync({
      suggestion_id: selectedSuggestionId,
      new_version: newVersion,
      changelog,
    });
    setAcceptDialogOpen(false);
    setSelectedSuggestionId(null);
  };

  const handleDecline = async (suggestionId: string) => {
    await declineSuggestion.mutateAsync({
      suggestion_id: suggestionId,
      reason: "Declined by user",
    });
  };

  const pendingCount =
    version.improvement_suggestions?.filter((s) => s.status === "pending")
      .length || 0;

  return (
    <>
      <Dialog open={open} onOpenChange={onOpenChange}>
        <DialogContent className="max-w-3xl max-h-[80vh] overflow-y-auto">
          <DialogHeader>
            <div className="flex items-start justify-between gap-4">
              <div>
                <DialogTitle>Version {version.version}</DialogTitle>
                <DialogDescription>
                  Created{" "}
                  {formatDistanceToNow(new Date(version.created_at), {
                    addSuffix: true,
                  })}
                </DialogDescription>
              </div>
              <div className="flex gap-2">
                {tags.map((tag) => (
                  <Badge key={tag} variant="secondary">
                    {tag}
                  </Badge>
                ))}
              </div>
            </div>
          </DialogHeader>

          <Tabs defaultValue="content" className="w-full">
            <TabsList className="grid w-full grid-cols-3">
              <TabsTrigger value="content">Content</TabsTrigger>
              <TabsTrigger value="feedback">
                Feedback ({feedback.length})
              </TabsTrigger>
              <TabsTrigger value="improvements">
                Improvements {pendingCount > 0 && `(${pendingCount})`}
              </TabsTrigger>
            </TabsList>

            <TabsContent value="content" className="space-y-4">
              <div className="rounded-lg border bg-muted/50 p-4">
                <pre className="text-sm whitespace-pre-wrap font-mono">
                  {version.content}
                </pre>
              </div>
              {version.changelog && (
                <div className="text-sm">
                  <span className="font-medium">Changelog: </span>
                  <span className="text-muted-foreground">
                    {version.changelog}
                  </span>
                </div>
              )}
            </TabsContent>

            <TabsContent value="feedback" className="space-y-4">
              <div className="flex justify-end">
                <Button size="sm" onClick={onAddFeedback}>
                  <Plus className="h-3 w-3 mr-1" />
                  Add Feedback
                </Button>
              </div>
              {feedback.length === 0 ? (
                <div className="text-center py-12 text-muted-foreground text-sm">
                  No feedback yet. Add feedback to help improve this prompt.
                </div>
              ) : (
                <div className="space-y-3">
                  {feedback.map((fb) => (
                    <FeedbackItem
                      key={fb.id}
                      feedback={fb}
                      onDelete={() => onDeleteFeedback(fb.id)}
                    />
                  ))}
                </div>
              )}
            </TabsContent>

            <TabsContent value="improvements" className="space-y-4">
              <ImprovementsList
                suggestions={version.improvement_suggestions || []}
                onAccept={handleAcceptClick}
                onDecline={handleDecline}
                isLoading={
                  acceptSuggestion.isPending || declineSuggestion.isPending
                }
              />
            </TabsContent>
          </Tabs>
        </DialogContent>
      </Dialog>

      <AcceptSuggestionDialog
        open={acceptDialogOpen}
        onOpenChange={setAcceptDialogOpen}
        onSubmit={handleAcceptSubmit}
        isLoading={acceptSuggestion.isPending}
        currentVersion={version.version}
      />
    </>
  );
}
