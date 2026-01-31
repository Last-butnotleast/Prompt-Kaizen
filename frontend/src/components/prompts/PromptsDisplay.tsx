import { AppSidebar } from "@/components/app-sidebar";
import { SidebarInset, SidebarProvider } from "@/components/ui/sidebar";
import { Button } from "@/components/ui/button";
import { Plus } from "lucide-react";
import { useState, useEffect } from "react";
import { useNavigate } from "@tanstack/react-router";
import supabase from "@/lib/supabase";
import type { User } from "@supabase/supabase-js";
import type { Prompt, CreatePromptRequest } from "@/types";
import { PromptsList } from "./PromptsList";
import { CreatePromptDialog } from "./CreatePromptDialog";

interface PromptsDisplayProps {
  prompts: Prompt[];
  isLoading: boolean;
  error: Error | null;
  onCreatePrompt: (data: CreatePromptRequest) => Promise<void>;
  onPromptClick: (promptId: string) => void;
  isCreating: boolean;
}

export function PromptsDisplay({
  prompts,
  isLoading,
  error,
  onCreatePrompt,
  onPromptClick,
  isCreating,
}: PromptsDisplayProps) {
  const navigate = useNavigate();
  const [user, setUser] = useState<User | null>(null);
  const [dialogOpen, setDialogOpen] = useState(false);

  useEffect(() => {
    supabase.auth.getSession().then(({ data: { session } }) => {
      if (!session) {
        navigate({ to: "/login" });
      } else {
        setUser(session.user);
      }
    });

    const {
      data: { subscription },
    } = supabase.auth.onAuthStateChange((_event, session) => {
      if (!session) {
        navigate({ to: "/login" });
      } else {
        setUser(session.user);
      }
    });

    return () => subscription.unsubscribe();
  }, [navigate]);

  const handleCreate = async (data: CreatePromptRequest) => {
    await onCreatePrompt(data);
    setDialogOpen(false);
  };

  if (!user) return null;

  return (
    <SidebarProvider>
      <AppSidebar user={user} />
      <SidebarInset>
        <header className="flex h-16 shrink-0 items-center gap-2 border-b px-4">
          <div className="flex items-center gap-2 flex-1">
            <h1 className="text-xl font-semibold">Prompts</h1>
          </div>
          <Button onClick={() => setDialogOpen(true)}>
            <Plus className="h-4 w-4 mr-2" />
            Create Prompt
          </Button>
        </header>
        <main className="flex flex-1 flex-col gap-4 p-4">
          {isLoading ? (
            <div className="flex items-center justify-center py-12">
              <div className="text-muted-foreground">Loading prompts...</div>
            </div>
          ) : error ? (
            <div className="flex items-center justify-center py-12">
              <div className="text-destructive">
                Failed to load prompts: {error.message}
              </div>
            </div>
          ) : prompts.length === 0 ? (
            <div className="flex flex-col items-center justify-center py-12 gap-4">
              <div className="text-center">
                <h2 className="text-lg font-semibold mb-2">No prompts yet</h2>
                <p className="text-muted-foreground mb-4">
                  Create your first prompt to get started with version control
                  and feedback tracking.
                </p>
                <Button onClick={() => setDialogOpen(true)}>
                  <Plus className="h-4 w-4 mr-2" />
                  Create First Prompt
                </Button>
              </div>
            </div>
          ) : (
            <PromptsList prompts={prompts} onPromptClick={onPromptClick} />
          )}
        </main>
      </SidebarInset>
      <CreatePromptDialog
        open={dialogOpen}
        onOpenChange={setDialogOpen}
        onSubmit={handleCreate}
        isLoading={isCreating}
      />
    </SidebarProvider>
  );
}
