import { createFileRoute } from "@tanstack/react-router";
import { SettingsManager } from "@/components/settings/SettingsManager";

export const Route = createFileRoute("/settings")({
  component: SettingsManager,
});
