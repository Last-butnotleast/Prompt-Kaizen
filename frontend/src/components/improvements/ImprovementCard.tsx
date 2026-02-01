import { Card, CardContent } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { CheckCircle, XCircle } from "lucide-react";
import type { ImprovementSuggestion } from "@/types";
import { formatDistanceToNow } from "date-fns";

interface ImprovementCardProps {
  suggestion: ImprovementSuggestion;
  onAccept: () => void;
  onDecline: () => void;
  isLoading: boolean;
}

export function ImprovementCard({
  suggestion,
  onAccept,
  onDecline,
  isLoading,
}: ImprovementCardProps) {
  const statusConfig = {
    pending: { variant: "secondary" as const, label: "Pending" },
    accepted: { variant: "default" as const, label: "Accepted" },
    declined: { variant: "outline" as const, label: "Declined" },
  };

  const config = statusConfig[suggestion.status];

  return (
    <Card>
      <CardContent className="pt-6">
        <div className="flex items-start justify-between gap-4 mb-3">
          <Badge variant={config.variant}>{config.label}</Badge>
          <span className="text-xs text-muted-foreground">
            {formatDistanceToNow(new Date(suggestion.created_at), {
              addSuffix: true,
            })}
          </span>
        </div>

        <p className="text-sm mb-3 whitespace-pre-wrap">
          {suggestion.suggested_content}
        </p>

        {suggestion.ai_rationale && (
          <p className="text-xs text-muted-foreground mb-4 border-l-2 pl-3">
            {suggestion.ai_rationale}
          </p>
        )}

        {suggestion.decline_reason && (
          <p className="text-xs text-muted-foreground mb-4 border-l-2 border-destructive pl-3">
            Declined: {suggestion.decline_reason}
          </p>
        )}

        {suggestion.status === "pending" && (
          <div className="flex gap-2">
            <Button size="sm" onClick={onAccept} disabled={isLoading}>
              <CheckCircle className="h-3 w-3 mr-1" />
              Accept
            </Button>
            <Button
              size="sm"
              variant="outline"
              onClick={onDecline}
              disabled={isLoading}
            >
              <XCircle className="h-3 w-3 mr-1" />
              Decline
            </Button>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
