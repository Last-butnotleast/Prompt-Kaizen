import { useState, useEffect } from "react";
import { AppSidebar } from "@/components/app-sidebar";
import { SidebarInset, SidebarProvider } from "@/components/ui/sidebar";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { ArrowLeft, Plus, Tag, Trash2, Star } from "lucide-react";
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
} from "@/types";

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
  onBack,
}: PromptDetailDisplayProps) {
  const navigate = useNavigate();
  const [user, setUser] = useState<User | null>(null);

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
          <Button size="sm">
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
                      {prompt.versions.map((version) => {
                        const versionTags = prompt.tags.filter(
                          (t) => t.version_id === version.id,
                        );
                        return (
                          <div
                            key={version.id}
                            className="border rounded-lg p-4"
                          >
                            <div className="flex items-start justify-between mb-2">
                              <div className="flex items-center gap-2 flex-wrap">
                                <Badge variant="outline">
                                  v{version.version}
                                </Badge>
                                {versionTags.map((tag) => (
                                  <Badge key={tag.name} variant="secondary">
                                    {tag.name}
                                  </Badge>
                                ))}
                              </div>
                              <Button variant="ghost" size="icon">
                                <Trash2 className="h-4 w-4" />
                              </Button>
                            </div>
                            {version.changelog && (
                              <p className="text-sm text-muted-foreground mb-2">
                                {version.changelog}
                              </p>
                            )}
                            <div className="flex items-center justify-between text-xs text-muted-foreground">
                              <span>
                                {formatDistanceToNow(
                                  new Date(version.created_at),
                                  { addSuffix: true },
                                )}
                              </span>
                              {version.feedback_count > 0 && (
                                <div className="flex items-center gap-1">
                                  <Star className="h-3 w-3 fill-current" />
                                  <span>
                                    {version.average_rating?.toFixed(1)}
                                  </span>
                                  <span>({version.feedback_count})</span>
                                </div>
                              )}
                            </div>
                          </div>
                        );
                      })}
                    </div>
                  )}
                </CardContent>
              </Card>
            </>
          )}
        </main>
      </SidebarInset>
    </SidebarProvider>
  );
}
