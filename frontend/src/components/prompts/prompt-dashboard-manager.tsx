import { PromptDashboardDisplay } from "./prompt-dashboard-display";

export function PromptDashboardManager() {
  return (
    <PromptDashboardDisplay
      promptCount={0}
      versionCount={0}
      feedbackCount={0}
    />
  );
}
