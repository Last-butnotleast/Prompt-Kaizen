import { useState, useEffect } from "react";
import { AppSidebar } from "@/components/app-sidebar";
import { SidebarInset, SidebarProvider } from "@/components/ui/sidebar";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { ArrowLeft, Plus, Tag } from "lucide-react";
import { useNavigate } from "@tanstack/react-router";
import { formatDistanceToNow } from "date-fns";
import supabase from "@/lib/supabase";
import type { User } from "@supabase/supabase-js";
import type {
  Prompt,
  UpdatePromptRequest,
  CreateVersionRequest,
  TagVersionRequest,
  SubmitFeedbackRequest,
  Version,
} from "@/types";
import { CreateVersionDialog } from "@/components/versions/CreateVersionDialog";
import { VersionContentDialog } from "@/components/versions/VersionContentDialog";
import { VersionCard } from "@/components/versions/VersionCard";

interface PromptDetailDisplayProps {
  prompt?: Prompt;
  isLoading: boolean;
  error: Error | null;
  onUpdatePrompt: (data: UpdatePromptRequest) => Promise<void>;
  onDeletePrompt: () => Promise<void>;
  onCreateVersion: (data: CreateVersionRequest) => Promise<void>;
  onDeleteVersion: (versionId: string) => Promise<void>;
  onTagVersion: (data: TagVersionRequest) => Promise<void>;
  onDeleteTag: (tagName: string) => Promise<void>;
  onSubmitFeedback: (data: SubmitFeedbackRequest) => Promise<void>;
  onBack: () => void;
  isUpdating: boolean;
  isDeleting: boolean;
  isCreatingVersion: boolean;
}

export function PromptDetailDisplay({
  prompt,
  isLoading,
  error,
  onCreateVersion,
  onDeleteVersion,
  onBack,
  isCreatingVersion,
}: PromptDetailDisplayProps) {
  const navigate = useNavigate();
  const [user, setUser] = useState<User | null>(null);
  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  const [viewDialogOpen, setViewDialogOpen] = useState(false);
  const [selectedVersion, setSelectedVersion] = useState<Version | null>(null);

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

  const handleCreateVersion = async (data: CreateVersionRequest) => {
    await onCreateVersion(data);
    setCreateDialogOpen(false);
  };

  const handleViewVersion = (version: Version) => {
    setSelectedVersion(version);
    setViewDialogOpen(true);
  };

  const handleDeleteVersion = async (versionId: string) => {
    if (confirm("Are you sure you want to delete this version?")) {
      await onDeleteVersion(versionId);
    }
  };

  const getVersionTags = (versionId: string): string[] => {
    if (!prompt) return [];
    return prompt.tags
      .filter((t) => t.version_id === versionId)
      .map((t) => t.name);
  };

  const latestVersion = prompt?.versions.sort((a, b) =>
    b.version.localeCompare(a.version),
  )[0]?.version;

  if (!user) return null;

  return (
    <SidebarProvider>
      <AppSidebar user={user} />
      <SidebarInset>
        <header className="flex h-16 shrink-0 items-center gap-2 border-b px-4">
          <Button variant="ghost" size="icon" onClick={onBack}>
            <ArrowLeft className="h-4 w-4" />
          </Button>
          <div className="flex items-center gap-2 flex-1">
            <h1 className="text-xl font-semibold">
              {prompt?.name || "Prompt"}
            </h1>
          </div>
          <Button size="sm" onClick={() => setCreateDialogOpen(true)}>
            <Plus className="h-4 w-4 mr-2" />
            New Version
          </Button>
        </header>
        <main className="flex flex-1 flex-col gap-4 p-4">
          {isLoading ? (
            <div className="flex items-center justify-center py-12">
              <div className="text-muted-foreground">Loading prompt...</div>
            </div>
          ) : error ? (
            <div className="flex items-center justify-center py-12">
              <div className="text-destructive">
                Failed to load prompt: {error.message}
              </div>
            </div>
          ) : !prompt ? (
            <div className="flex items-center justify-center py-12">
              <div className="text-muted-foreground">Prompt not found</div>
            </div>
          ) : (
            <>
              <Card>
                <CardHeader>
                  <CardTitle>Details</CardTitle>
                  {prompt.description && (
                    <CardDescription>{prompt.description}</CardDescription>
                  )}
                </CardHeader>
                <CardContent>
                  <div className="space-y-2 text-sm">
                    <div className="flex justify-between">
                      <span className="text-muted-foreground">Created</span>
                      <span>
                        {formatDistanceToNow(new Date(prompt.created_at), {
                          addSuffix: true,
                        })}
                      </span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-muted-foreground">Versions</span>
                      <span>{prompt.versions.length}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-muted-foreground">Tags</span>
                      <span>{prompt.tags.length}</span>
                    </div>
                  </div>
                </CardContent>
              </Card>

              <Card>
                <CardHeader>
                  <div className="flex items-center justify-between">
                    <CardTitle>Versions</CardTitle>
                    <Button variant="outline" size="sm">
                      <Tag className="h-4 w-4 mr-2" />
                      Manage Tags
                    </Button>
                  </div>
                </CardHeader>
                <CardContent>
                  {prompt.versions.length === 0 ? (
                    <div className="text-center py-8 text-muted-foreground">
                      No versions yet. Create your first version to get started.
                    </div>
                  ) : (
                    <div className="space-y-3">
                      {prompt.versions.map((version) => (
                        <VersionCard
                          key={version.id}
                          version={version}
                          tags={getVersionTags(version.id)}
                          onView={() => handleViewVersion(version)}
                          onDelete={() => handleDeleteVersion(version.id)}
                        />
                      ))}
                    </div>
                  )}
                </CardContent>
              </Card>
            </>
          )}
        </main>
      </SidebarInset>

      <CreateVersionDialog
        open={createDialogOpen}
        onOpenChange={setCreateDialogOpen}
        onSubmit={handleCreateVersion}
        isLoading={isCreatingVersion}
        latestVersion={latestVersion}
      />

      <VersionContentDialog
        open={viewDialogOpen}
        onOpenChange={setViewDialogOpen}
        version={selectedVersion}
        tags={selectedVersion ? getVersionTags(selectedVersion.id) : []}
      />
    </SidebarProvider>
  );
}
