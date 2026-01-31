import { useNavigate } from "@tanstack/react-router";
import { useListPrompts, useCreatePrompt } from "@/api/hooks/prompts";
import { PromptsDisplay } from "./PromptsDisplay";
import type { CreatePromptRequest } from "@/types";

export function PromptsManager() {
  const navigate = useNavigate();
  const { data: prompts, isLoading, error } = useListPrompts();
  const createPrompt = useCreatePrompt();

  const handleCreate = async (data: CreatePromptRequest) => {
    try {
      const result = await createPrompt.mutateAsync(data);
      if (result?.id) {
        navigate({ to: `/prompts/${result.id}` });
      }
    } catch (err) {
      console.error("Failed to create prompt:", err);
      throw err;
    }
  };

  const handlePromptClick = (promptId: string) => {
    navigate({ to: `/prompts/${promptId}` });
  };

  return (
    <PromptsDisplay
      prompts={prompts || []}
      isLoading={isLoading}
      error={error}
      onCreatePrompt={handleCreate}
      onPromptClick={handlePromptClick}
      isCreating={createPrompt.isPending}
    />
  );
}
