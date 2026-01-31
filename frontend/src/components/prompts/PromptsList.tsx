import type { Prompt } from "@/types";
import { PromptCard } from "./PromptCard";

interface PromptsListProps {
  prompts: Prompt[];
  onPromptClick: (promptId: string) => void;
}

export function PromptsList({ prompts, onPromptClick }: PromptsListProps) {
  return (
    <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
      {prompts.map((prompt) => (
        <PromptCard
          key={prompt.id}
          prompt={prompt}
          onClick={() => onPromptClick(prompt.id)}
        />
      ))}
    </div>
  );
}
