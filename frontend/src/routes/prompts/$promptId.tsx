import { createFileRoute } from "@tanstack/react-router";
import { PromptDetailManager } from "@/components/prompts/PromptDetailManager";

export const Route = createFileRoute("/prompts/$promptId")({
  component: PromptDetailManager,
});
