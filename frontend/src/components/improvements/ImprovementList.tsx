import { ImprovementCard } from "./ImprovementCard";
import type { ImprovementSuggestion } from "@/types";

interface ImprovementsListProps {
  suggestions: ImprovementSuggestion[];
  onAccept: (id: string) => void;
  onDecline: (id: string) => void;
  isLoading: boolean;
}

export function ImprovementsList({
  suggestions,
  onAccept,
  onDecline,
  isLoading,
}: ImprovementsListProps) {
  if (suggestions.length === 0) {
    return (
      <div className="text-center py-12 text-muted-foreground text-sm">
        No suggestions yet. Analyze feedback to generate improvements.
      </div>
    );
  }

  const pending = suggestions.filter((s) => s.status === "pending");
  const processed = suggestions.filter((s) => s.status !== "pending");

  return (
    <div className="space-y-4">
      {pending.length > 0 && (
        <div className="space-y-3">
          <h3 className="text-sm font-medium">Pending Review</h3>
          {pending.map((suggestion) => (
            <ImprovementCard
              key={suggestion.id}
              suggestion={suggestion}
              onAccept={() => onAccept(suggestion.id)}
              onDecline={() => onDecline(suggestion.id)}
              isLoading={isLoading}
            />
          ))}
        </div>
      )}

      {processed.length > 0 && (
        <div className="space-y-3">
          <h3 className="text-sm font-medium text-muted-foreground">
            Processed
          </h3>
          {processed.map((suggestion) => (
            <ImprovementCard
              key={suggestion.id}
              suggestion={suggestion}
              onAccept={() => onAccept(suggestion.id)}
              onDecline={() => onDecline(suggestion.id)}
              isLoading={isLoading}
            />
          ))}
        </div>
      )}
    </div>
  );
}
