import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { X } from "lucide-react";

interface TagBadgeProps {
  name: string;
  onDelete?: () => void;
  variant?: "default" | "secondary" | "outline";
}

export function TagBadge({
  name,
  onDelete,
  variant = "secondary",
}: TagBadgeProps) {
  return (
    <Badge variant={variant} className="flex items-center gap-1">
      {name}
      {onDelete && (
        <Button
          variant="ghost"
          size="icon"
          className="h-3 w-3 p-0 hover:bg-transparent"
          onClick={(e) => {
            e.stopPropagation();
            onDelete();
          }}
        >
          <X className="h-3 w-3" />
        </Button>
      )}
    </Badge>
  );
}
