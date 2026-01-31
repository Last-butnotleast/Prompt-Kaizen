import { PromptDashboardManager } from "@/components/prompts";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/dashboard")({
  component: PromptDashboardManager,
});
