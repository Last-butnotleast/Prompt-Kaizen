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
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { Star } from "lucide-react";
import type { SubmitFeedbackRequest } from "@/types";

interface FeedbackFormDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onSubmit: (data: SubmitFeedbackRequest) => Promise<void>;
  isLoading: boolean;
  versionId: string;
  versionNumber: string;
}

export function FeedbackFormDialog({
  open,
  onOpenChange,
  onSubmit,
  isLoading,
  versionId,
  versionNumber,
}: FeedbackFormDialogProps) {
  const [rating, setRating] = useState<number>(0);
  const [hoveredRating, setHoveredRating] = useState<number>(0);
  const [comment, setComment] = useState("");
  const [error, setError] = useState("");

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");

    if (rating === 0) {
      setError("Please select a rating");
      return;
    }

    try {
      await onSubmit({
        version_id: versionId,
        rating,
        comment: comment.trim() || null,
      });
      setRating(0);
      setComment("");
    } catch (err) {
      setError("Failed to submit feedback");
    }
  };

  const handleOpenChange = (newOpen: boolean) => {
    if (!isLoading) {
      onOpenChange(newOpen);
      if (!newOpen) {
        setRating(0);
        setHoveredRating(0);
        setComment("");
        setError("");
      }
    }
  };

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogContent>
        <form onSubmit={handleSubmit}>
          <DialogHeader>
            <DialogTitle>Rate Version {versionNumber}</DialogTitle>
            <DialogDescription>
              Share your feedback on this prompt version.
            </DialogDescription>
          </DialogHeader>
          <div className="grid gap-4 py-4">
            <div className="grid gap-2">
              <Label>Rating *</Label>
              <div className="flex gap-1">
                {[1, 2, 3, 4, 5].map((value) => (
                  <button
                    key={value}
                    type="button"
                    onClick={() => setRating(value)}
                    onMouseEnter={() => setHoveredRating(value)}
                    onMouseLeave={() => setHoveredRating(0)}
                    disabled={isLoading}
                    className="transition-transform hover:scale-110"
                  >
                    <Star
                      className={`h-8 w-8 ${
                        value <= (hoveredRating || rating)
                          ? "fill-yellow-400 text-yellow-400"
                          : "text-gray-300"
                      }`}
                    />
                  </button>
                ))}
              </div>
            </div>
            <div className="grid gap-2">
              <Label htmlFor="comment">Comment (optional)</Label>
              <Textarea
                id="comment"
                value={comment}
                onChange={(e) => setComment(e.target.value)}
                placeholder="What did you think of this prompt version?"
                disabled={isLoading}
                rows={4}
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
              {isLoading ? "Submitting..." : "Submit Feedback"}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
