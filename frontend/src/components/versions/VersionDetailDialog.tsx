import { useState } from "react";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import { Input } from "@/components/ui/input";
import { Star, ArrowRight, Check, X } from "lucide-react";
import { formatDistanceToNow } from "date-fns";
import { toast } from "sonner";
import type { Version, Feedback, SubmitFeedbackRequest } from "@/types";
import {
  useAcceptImprovementSuggestion,
  useAnalyzeFeedback,
  useDeclineImprovementSuggestion,
} from "@/api/hooks/improvements";
import { useSubmitFeedback } from "@/api/hooks/feedback";
import { AcceptSuggestionDialog } from "@/components/improvements/AcceptSuggestionDialog";

interface VersionDetailDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  version: Version;
  tags: string[];
  feedback: Feedback[];
  onDeleteFeedback: (feedbackId: string) => void;
  promptId: string;
}

export function VersionDetailDialog({
  open,
  onOpenChange,
  version,
  tags,
  feedback,
  onDeleteFeedback,
  promptId,
}: VersionDetailDialogProps) {
  const [activeTab, setActiveTab] = useState<
    "content" | "feedback" | "improvements"
  >("content");
  const [feedbackRating, setFeedbackRating] = useState(0);
  const [feedbackComment, setFeedbackComment] = useState("");
  const [hoveredRating, setHoveredRating] = useState(0);
  const [testInput, setTestInput] = useState("");
  const [testActualOutput, setTestActualOutput] = useState("");
  const [testExpectedOutput, setTestExpectedOutput] = useState("");
  const [showTestScenario, setShowTestScenario] = useState(false);
  const [acceptDialogOpen, setAcceptDialogOpen] = useState(false);
  const [selectedSuggestionId, setSelectedSuggestionId] = useState<string>("");

  const submitFeedback = useSubmitFeedback(promptId);
  const acceptSuggestion = useAcceptImprovementSuggestion(promptId, version.id);
  const declineSuggestion = useDeclineImprovementSuggestion(
    promptId,
    version.id,
  );
  const analyzeFeedback = useAnalyzeFeedback(promptId, version.id);

  const handleSubmitFeedback = async () => {
    const data: SubmitFeedbackRequest = {
      version_id: version.id,
      rating: feedbackRating,
      comment: feedbackComment || null,
      test_input: showTestScenario && testInput ? testInput : null,
      test_actual_output:
        showTestScenario && testActualOutput ? testActualOutput : null,
      test_expected_output:
        showTestScenario && testExpectedOutput ? testExpectedOutput : null,
    };

    toast.promise(submitFeedback.mutateAsync(data), {
      loading: "Submitting feedback...",
      success: "Feedback submitted",
      error: "Failed to submit feedback",
    });

    setFeedbackRating(0);
    setFeedbackComment("");
    setTestInput("");
    setTestActualOutput("");
    setTestExpectedOutput("");
    setShowTestScenario(false);
  };

  const handleDeleteFeedback = (feedbackId: string) => {
    toast.promise(Promise.resolve(onDeleteFeedback(feedbackId)), {
      loading: "Deleting feedback...",
      success: "Feedback deleted",
      error: "Failed to delete feedback",
    });
  };

  const handleAnalyzeFeedback = async () => {
    toast.promise(analyzeFeedback.mutateAsync(), {
      loading: "Analyzing feedback...",
      success: "Analysis complete",
      error: "Failed to analyze feedback",
    });
  };

  const handleAccept = (suggestionId: string) => {
    setSelectedSuggestionId(suggestionId);
    setAcceptDialogOpen(true);
  };

  const handleAcceptSubmit = async (
    newVersion: string,
    changelog: string | null,
  ) => {
    await acceptSuggestion.mutateAsync({
      suggestion_id: selectedSuggestionId,
      new_version: newVersion,
      changelog,
    });

    setAcceptDialogOpen(false);
    toast.success(`Created v${newVersion}`);
    onOpenChange(false);
  };

  const handleDecline = async (suggestionId: string) => {
    toast.promise(
      declineSuggestion.mutateAsync({
        suggestion_id: suggestionId,
        reason: "Declined",
      }),
      {
        loading: "Declining suggestion...",
        success: "Suggestion declined",
        error: "Failed to decline suggestion",
      },
    );
  };

  const pendingSuggestions =
    version.improvement_suggestions?.filter((s) => s.status === "pending") ||
    [];
  const processedSuggestions =
    version.improvement_suggestions?.filter((s) => s.status !== "pending") ||
    [];

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-4xl max-h-[90vh] flex flex-col p-0">
        <DialogHeader className="px-8 pt-8 pb-6 space-y-3">
          <div className="flex items-baseline justify-between">
            <DialogTitle className="font-mono text-2xl font-normal tracking-tight">
              v{version.version}
            </DialogTitle>
            {tags.length > 0 && (
              <div className="flex gap-2">
                {tags.map((tag) => (
                  <span
                    key={tag}
                    className="text-xs font-mono text-neutral-500 uppercase tracking-wider"
                  >
                    {tag}
                  </span>
                ))}
              </div>
            )}
          </div>
          {version.changelog && (
            <p className="text-sm text-neutral-600 leading-relaxed">
              {version.changelog}
            </p>
          )}
          <div className="text-xs text-neutral-400 font-mono">
            {formatDistanceToNow(new Date(version.created_at), {
              addSuffix: true,
            })}
          </div>
        </DialogHeader>

        <nav className="flex gap-8 px-8 border-b border-neutral-200">
          {[
            { id: "content", label: "Content" },
            { id: "feedback", label: "Feedback", count: feedback.length },
            {
              id: "improvements",
              label: "Improvements",
              count: pendingSuggestions.length || null,
            },
          ].map((tab) => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id as typeof activeTab)}
              className={`pb-4 text-sm transition-colors relative ${
                activeTab === tab.id
                  ? "text-neutral-900"
                  : "text-neutral-400 hover:text-neutral-600"
              }`}
            >
              {tab.label}
              {!!tab.count && (
                <span className="ml-1.5 text-xs font-mono">({tab.count})</span>
              )}
              {activeTab === tab.id && (
                <div className="absolute bottom-0 left-0 right-0 h-0.5 bg-neutral-900" />
              )}
            </button>
          ))}
        </nav>

        <div className="flex-1 overflow-y-auto px-8 py-6">
          {activeTab === "content" && (
            <pre className="text-sm font-mono leading-relaxed whitespace-pre-wrap text-neutral-800 bg-neutral-50/50 p-6 rounded">
              {version.content}
            </pre>
          )}

          {activeTab === "feedback" && (
            <div className="space-y-6">
              <div className="bg-neutral-50/50 border border-neutral-200 rounded p-6 space-y-4">
                <div className="text-xs uppercase tracking-wider text-neutral-500 font-medium">
                  Add Feedback
                </div>

                <div className="flex items-center gap-2">
                  {[1, 2, 3, 4, 5].map((value) => (
                    <button
                      key={value}
                      type="button"
                      onClick={() => setFeedbackRating(value)}
                      onMouseEnter={() => setHoveredRating(value)}
                      onMouseLeave={() => setHoveredRating(0)}
                      className="transition-transform hover:scale-110"
                    >
                      <Star
                        className={`h-5 w-5 transition-colors ${
                          value <= (hoveredRating || feedbackRating)
                            ? "fill-neutral-900 text-neutral-900"
                            : "text-neutral-300"
                        }`}
                      />
                    </button>
                  ))}
                </div>

                <Textarea
                  value={feedbackComment}
                  onChange={(e) => setFeedbackComment(e.target.value)}
                  placeholder="Optional comment"
                  rows={2}
                  className="resize-none bg-white border-neutral-200"
                />

                <div className="flex items-center justify-between pt-2 border-t border-neutral-200">
                  <button
                    type="button"
                    onClick={() => setShowTestScenario(!showTestScenario)}
                    className="text-xs text-neutral-500 hover:text-neutral-900 transition-colors font-medium"
                  >
                    {showTestScenario ? "− Remove" : "+ Add"} test scenario
                  </button>

                  <Button
                    size="sm"
                    disabled={feedbackRating === 0 || submitFeedback.isPending}
                    onClick={handleSubmitFeedback}
                    className="h-8 px-4"
                  >
                    Submit
                  </Button>
                </div>

                {showTestScenario && (
                  <div className="space-y-3 pt-4 border-t border-neutral-200">
                    <Input
                      value={testInput}
                      onChange={(e) => setTestInput(e.target.value)}
                      placeholder="Input"
                      className="bg-white border-neutral-200 font-mono text-xs h-9"
                    />
                    <Input
                      value={testActualOutput}
                      onChange={(e) => setTestActualOutput(e.target.value)}
                      placeholder="Actual output"
                      className="bg-white border-neutral-200 font-mono text-xs h-9"
                    />
                    <Input
                      value={testExpectedOutput}
                      onChange={(e) => setTestExpectedOutput(e.target.value)}
                      placeholder="Expected output (optional)"
                      className="bg-white border-neutral-200 font-mono text-xs h-9"
                    />
                  </div>
                )}
              </div>

              {feedback.length === 0 ? (
                <div className="py-16 text-center text-sm text-neutral-400">
                  No feedback yet
                </div>
              ) : (
                <div className="space-y-4">
                  {feedback.map((fb) => (
                    <div
                      key={fb.id}
                      className="space-y-3 pb-4 border-b border-neutral-100 last:border-0 last:pb-0"
                    >
                      <div className="flex items-start justify-between gap-4">
                        <div className="flex gap-0.5">
                          {[1, 2, 3, 4, 5].map((value) => (
                            <Star
                              key={value}
                              className={`h-4 w-4 ${
                                value <= fb.rating
                                  ? "fill-neutral-900 text-neutral-900"
                                  : "text-neutral-300"
                              }`}
                            />
                          ))}
                        </div>
                        <div className="flex items-center gap-3 text-xs">
                          <span className="text-neutral-400 font-mono">
                            {formatDistanceToNow(new Date(fb.created_at), {
                              addSuffix: true,
                            })}
                          </span>
                          <button
                            onClick={() => handleDeleteFeedback(fb.id)}
                            className="text-neutral-400 hover:text-neutral-900 transition-colors"
                          >
                            Delete
                          </button>
                        </div>
                      </div>

                      {fb.comment && (
                        <p className="text-sm text-neutral-700 leading-relaxed">
                          {fb.comment}
                        </p>
                      )}

                      {fb.test_scenario && (
                        <div className="space-y-2 bg-neutral-50/50 p-3 rounded border border-neutral-100">
                          <div className="space-y-1">
                            <div className="text-neutral-400 uppercase tracking-wider text-[10px] font-medium">
                              Input
                            </div>
                            <div className="text-xs font-mono text-neutral-800 leading-relaxed">
                              {fb.test_scenario.input}
                            </div>
                          </div>

                          <div className="flex items-center gap-2 py-1">
                            <ArrowRight className="h-3 w-3 text-neutral-300" />
                          </div>

                          <div className="space-y-1">
                            <div className="text-neutral-400 uppercase tracking-wider text-[10px] font-medium">
                              Actual Output
                            </div>
                            <div className="text-xs font-mono text-neutral-800 leading-relaxed">
                              {fb.test_scenario.actual_output}
                            </div>
                          </div>

                          {fb.test_scenario.expected_output && (
                            <>
                              <div className="border-t border-neutral-200 my-2" />
                              <div className="space-y-1">
                                <div className="text-neutral-400 uppercase tracking-wider text-[10px] font-medium">
                                  Expected Output
                                </div>
                                <div className="text-xs font-mono text-neutral-800 leading-relaxed">
                                  {fb.test_scenario.expected_output}
                                </div>
                              </div>
                            </>
                          )}
                        </div>
                      )}
                    </div>
                  ))}
                </div>
              )}
            </div>
          )}

          {activeTab === "improvements" && (
            <div className="space-y-8">
              {version.improvement_suggestions?.length === 0 &&
              feedback.length > 0 ? (
                <div className="py-16 text-center space-y-4">
                  <p className="text-sm text-neutral-600">
                    {feedback.length} feedback item
                    {feedback.length !== 1 ? "s" : ""} available for analysis
                  </p>
                  <Button
                    onClick={handleAnalyzeFeedback}
                    disabled={analyzeFeedback.isPending}
                    className="h-9 px-6"
                  >
                    {analyzeFeedback.isPending
                      ? "Analyzing..."
                      : "Generate Improvement Suggestions"}
                  </Button>
                </div>
              ) : version.improvement_suggestions?.length === 0 ? (
                <div className="py-16 text-center text-sm text-neutral-400">
                  Add feedback first to generate improvement suggestions
                </div>
              ) : (
                <>
                  {pendingSuggestions.length > 0 && (
                    <div className="space-y-4">
                      {pendingSuggestions.map((suggestion) => (
                        <div
                          key={suggestion.id}
                          className="space-y-4 pb-6 border-b border-neutral-200"
                        >
                          <pre className="text-sm font-mono leading-relaxed whitespace-pre-wrap text-neutral-800 bg-neutral-50/50 p-4 rounded">
                            {suggestion.suggested_content}
                          </pre>

                          {suggestion.ai_rationale && (
                            <div className="text-sm text-neutral-600 leading-relaxed pl-4 border-l-2 border-neutral-200">
                              {suggestion.ai_rationale}
                            </div>
                          )}

                          <div className="flex gap-3 pt-2">
                            <Button
                              size="sm"
                              onClick={() => handleAccept(suggestion.id)}
                              disabled={acceptSuggestion.isPending}
                              className="h-9 px-6"
                            >
                              <Check className="h-4 w-4 mr-1.5" />
                              Accept
                            </Button>
                            <Button
                              size="sm"
                              variant="ghost"
                              onClick={() => handleDecline(suggestion.id)}
                              disabled={declineSuggestion.isPending}
                              className="h-9 px-6"
                            >
                              <X className="h-4 w-4 mr-1.5" />
                              Decline
                            </Button>
                          </div>
                        </div>
                      ))}
                    </div>
                  )}

                  {processedSuggestions.length > 0 && (
                    <div className="space-y-4">
                      <div className="text-xs text-neutral-400 uppercase tracking-wider">
                        Processed
                      </div>
                      {processedSuggestions.map((suggestion) => (
                        <div
                          key={suggestion.id}
                          className="space-y-2 opacity-40 pb-4 border-b border-neutral-100 last:border-0"
                        >
                          <div className="flex items-center gap-2 text-xs font-mono">
                            <span
                              className={
                                suggestion.status === "accepted"
                                  ? "text-green-700"
                                  : "text-neutral-500"
                              }
                            >
                              {suggestion.status}
                            </span>
                          </div>
                          <pre className="text-sm font-mono leading-relaxed whitespace-pre-wrap text-neutral-700">
                            {suggestion.suggested_content}
                          </pre>
                          {suggestion.decline_reason && (
                            <p className="text-xs text-neutral-500">
                              {suggestion.decline_reason}
                            </p>
                          )}
                        </div>
                      ))}
                    </div>
                  )}
                </>
              )}
            </div>
          )}
        </div>
      </DialogContent>
      <AcceptSuggestionDialog
        open={acceptDialogOpen}
        onOpenChange={setAcceptDialogOpen}
        onSubmit={handleAcceptSubmit}
        isLoading={acceptSuggestion.isPending}
        currentVersion={version.version}
      />
    </Dialog>
  );
}
