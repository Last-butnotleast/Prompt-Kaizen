import { createFileRoute } from "@tanstack/react-router";
import { PromptsManager } from "@/components/prompts/PromptsManager";

export const Route = createFileRoute("/prompts")({
  component: PromptsManager,
});
