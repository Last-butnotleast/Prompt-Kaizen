import type { Feedback } from "@/types";
import { FeedbackItem } from "./FeedbackItem";

interface FeedbackListProps {
  feedback: Feedback[];
  onDelete: (feedbackId: string) => void;
}

export function FeedbackList({ feedback, onDelete }: FeedbackListProps) {
  if (feedback.length === 0) {
    return (
      <div className="text-center py-8 text-sm text-muted-foreground">
        No feedback yet. Be the first to rate this version!
      </div>
    );
  }

  return (
    <div className="space-y-3">
      {feedback.map((item) => (
        <FeedbackItem
          key={item.id}
          feedback={item}
          onDelete={() => onDelete(item.id)}
        />
      ))}
    </div>
  );
}
