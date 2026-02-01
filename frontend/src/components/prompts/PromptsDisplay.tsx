import { Button } from "@/components/ui/button";
import { useState, useEffect } from "react";
import { useNavigate } from "@tanstack/react-router";
import supabase from "@/lib/supabase";
import type { User } from "@supabase/supabase-js";
import type { Prompt, CreatePromptRequest } from "@/types";
import { PromptsList } from "./PromptsList";
import { CreatePromptDialog } from "./CreatePromptDialog";
import { SimpleHeader } from "@/components/layout/SimpleHeader";
import { toast } from "sonner";

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
    try {
      await onCreatePrompt(data);
      setDialogOpen(false);
      toast.success("Prompt created", {
        description: data.name,
      });
    } catch (error) {
      toast.error("Failed to create prompt");
    }
  };

  if (!user) return null;

  return (
    <div className="min-h-screen bg-neutral-50">
      <SimpleHeader user={user} breadcrumbs={[{ label: "Prompts" }]} />
      <main className="max-w-7xl mx-auto px-8 py-8">
        <div className="flex items-center justify-between mb-6">
          <h1 className="text-2xl font-semibold">Prompts</h1>
          <Button onClick={() => setDialogOpen(true)} size="sm">
            New
          </Button>
        </div>
        {isLoading ? (
          <div className="text-center py-20 text-muted-foreground">
            Loading...
          </div>
        ) : error ? (
          <div className="text-center py-20 text-destructive">
            Failed to load prompts
          </div>
        ) : prompts.length === 0 ? (
          <div className="text-center py-20">
            <p className="text-muted-foreground mb-4">No prompts yet</p>
            <Button onClick={() => setDialogOpen(true)}>
              Create first prompt
            </Button>
          </div>
        ) : (
          <PromptsList prompts={prompts} onPromptClick={onPromptClick} />
        )}
      </main>
      <CreatePromptDialog
        open={dialogOpen}
        onOpenChange={setDialogOpen}
        onSubmit={handleCreate}
        isLoading={isCreating}
      />
    </div>
  );
}
