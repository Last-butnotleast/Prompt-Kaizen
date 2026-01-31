import { AppSidebar } from "@/components/app-sidebar";
import { SidebarInset, SidebarProvider } from "@/components/ui/sidebar";

interface PromptDashboardDisplayProps {
  promptCount?: number;
  versionCount?: number;
  feedbackCount?: number;
}

export function PromptDashboardDisplay({
  promptCount = 0,
  versionCount = 0,
  feedbackCount = 0,
}: PromptDashboardDisplayProps) {
  return (
    <SidebarProvider>
      <AppSidebar />
      <SidebarInset>
        <main className="flex flex-1 flex-col gap-4 p-4">
          <div className="flex items-center gap-2 px-4 py-2">
            <h1 className="text-2xl font-bold">Prompt Dashboard</h1>
          </div>
          <div className="grid auto-rows-min gap-4 md:grid-cols-3">
            <div className="aspect-video rounded-xl bg-muted/50 flex items-center justify-center">
              <div className="text-center">
                <div className="text-3xl font-bold">{promptCount}</div>
                <div className="text-sm text-muted-foreground">Prompts</div>
              </div>
            </div>
            <div className="aspect-video rounded-xl bg-muted/50 flex items-center justify-center">
              <div className="text-center">
                <div className="text-3xl font-bold">{versionCount}</div>
                <div className="text-sm text-muted-foreground">Versions</div>
              </div>
            </div>
            <div className="aspect-video rounded-xl bg-muted/50 flex items-center justify-center">
              <div className="text-center">
                <div className="text-3xl font-bold">{feedbackCount}</div>
                <div className="text-sm text-muted-foreground">Feedback</div>
              </div>
            </div>
          </div>
          <div className="min-h-screen flex-1 rounded-xl bg-muted/50 md:min-h-min" />
        </main>
      </SidebarInset>
    </SidebarProvider>
  );
}
