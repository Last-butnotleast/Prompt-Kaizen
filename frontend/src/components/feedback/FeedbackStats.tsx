import { Star } from "lucide-react";

interface FeedbackStatsProps {
  averageRating: number | null | undefined;
  count: number;
  compact?: boolean;
}

export function FeedbackStats({
  averageRating,
  count,
  compact = false,
}: FeedbackStatsProps) {
  if (count === 0 || !averageRating) {
    return compact ? null : (
      <div className="text-sm text-muted-foreground">No ratings yet</div>
    );
  }

  if (compact) {
    return (
      <div className="flex items-center gap-1 text-xs text-muted-foreground">
        <Star className="h-3 w-3 fill-yellow-400 text-yellow-400" />
        <span>{averageRating.toFixed(1)}</span>
        <span>({count})</span>
      </div>
    );
  }

  return (
    <div className="flex items-center gap-2">
      <div className="flex gap-0.5">
        {[1, 2, 3, 4, 5].map((value) => (
          <Star
            key={value}
            className={`h-5 w-5 ${
              value <= Math.round(averageRating)
                ? "fill-yellow-400 text-yellow-400"
                : "text-gray-300"
            }`}
          />
        ))}
      </div>
      <div className="text-sm">
        <span className="font-semibold">{averageRating.toFixed(1)}</span>
        <span className="text-muted-foreground"> ({count} ratings)</span>
      </div>
    </div>
  );
}
