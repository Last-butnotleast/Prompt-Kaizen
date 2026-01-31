import { Card, CardContent } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Star, Trash2 } from "lucide-react";
import { formatDistanceToNow } from "date-fns";
import type { Feedback } from "@/types";

interface FeedbackItemProps {
  feedback: Feedback;
  onDelete?: () => void;
}

export function FeedbackItem({ feedback, onDelete }: FeedbackItemProps) {
  return (
    <Card>
      <CardContent className="pt-4">
        <div className="flex items-start justify-between mb-2">
          <div className="flex gap-0.5">
            {[1, 2, 3, 4, 5].map((value) => (
              <Star
                key={value}
                className={`h-4 w-4 ${
                  value <= feedback.rating
                    ? "fill-yellow-400 text-yellow-400"
                    : "text-gray-300"
                }`}
              />
            ))}
          </div>
          {onDelete && (
            <Button
              variant="ghost"
              size="icon"
              className="h-6 w-6"
              onClick={onDelete}
            >
              <Trash2 className="h-3 w-3" />
            </Button>
          )}
        </div>
        {feedback.comment && (
          <p className="text-sm text-muted-foreground mb-2">
            {feedback.comment}
          </p>
        )}
        <p className="text-xs text-muted-foreground">
          {formatDistanceToNow(new Date(feedback.created_at), {
            addSuffix: true,
          })}
        </p>
      </CardContent>
    </Card>
  );
}
