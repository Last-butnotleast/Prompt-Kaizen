import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import { useNavigate } from "@tanstack/react-router";
import supabase from "@/lib/supabase";
import type { User } from "@supabase/supabase-js";
import type {
  Prompt,
  UpdatePromptRequest,
  CreateVersionRequest,
  TagVersionRequest,
  SubmitFeedbackRequest,
  Feedback,
} from "@/types";
import { CreateVersionDialog } from "@/components/versions/CreateVersionDialog";
import { VersionDetailDialog } from "@/components/versions/VersionDetailDialog";
import { SimpleHeader } from "@/components/layout/SimpleHeader";
import { formatDistanceToNow } from "date-fns";
import { Star } from "lucide-react";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import { toast } from "sonner";

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
  onDeleteFeedback: (versionId: string, feedbackId: string) => Promise<void>;
  onBack: () => void;
  isUpdating: boolean;
  isDeleting: boolean;
  isCreatingVersion: boolean;
  isTagging: boolean;
  isSubmittingFeedback: boolean;
  versionFeedback: Record<string, Feedback[]>;
  onAnalyzeFeedback: (versionId: string) => Promise<void>;
  isAnalyzing: boolean;
  analyzingVersionId: string | null;
}

const STANDARD_TAGS = [
  "latest",
  "production",
  "staging",
  "experimental",
  "deprecated",
];

export function PromptDetailDisplay({
  prompt,
  isLoading,
  error,
  onCreateVersion,
  onDeleteVersion,
  onTagVersion,
  onDeleteTag,
  onDeleteFeedback,
  isCreatingVersion,
  versionFeedback,
}: PromptDetailDisplayProps) {
  const navigate = useNavigate();
  const [user, setUser] = useState<User | null>(null);
  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  const [viewDialogOpen, setViewDialogOpen] = useState(false);
  const [selectedVersionId, setSelectedVersionId] = useState<string | null>(
    null,
  );
  const [addingTagToVersion, setAddingTagToVersion] = useState<string | null>(
    null,
  );
  const [hoveredVersion, setHoveredVersion] = useState<string | null>(null);

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
    toast.promise(() => onCreateVersion(data), {
      loading: "Creating version...",
      success: `v${data.version} created successfully`,
      error: "Failed to create version",
    });
    setCreateDialogOpen(false);
  };

  const handleViewVersion = (versionId: string) => {
    setSelectedVersionId(versionId);
    setViewDialogOpen(true);
  };

  const handleDeleteVersion = async (
    versionId: string,
    versionNumber: string,
  ) => {
    toast.promise(() => onDeleteVersion(versionId), {
      loading: "Deleting version...",
      success: `v${versionNumber} removed`,
      error: "Failed to delete version",
    });
  };

  const handleSelectTag = async (versionId: string, tagName: string) => {
    toast.promise(
      () => onTagVersion({ tag_name: tagName, version_id: versionId }),
      {
        loading: "Adding tag...",
        success: `Tagged with "${tagName}"`,
        error: "Failed to add tag",
      },
    );
    setAddingTagToVersion(null);
  };

  const handleDeleteTag = async (tagName: string) => {
    toast.promise(() => onDeleteTag(tagName), {
      loading: "Removing tag...",
      success: `"${tagName}" removed`,
      error: "Failed to remove tag",
    });
  };

  const handleDeleteFeedback = async (feedbackId: string) => {
    if (selectedVersionId) {
      toast.promise(() => onDeleteFeedback(selectedVersionId, feedbackId), {
        loading: "Deleting feedback...",
        success: "Feedback deleted",
        error: "Failed to delete feedback",
      });
    }
  };

  const getVersionTags = (versionId: string): string[] => {
    if (!prompt) return [];
    return prompt.tags
      .filter((t) => t.version_id === versionId)
      .map((t) => t.name);
  };

  const getAvailableTags = (versionId: string): string[] => {
    const existingTags = getVersionTags(versionId);
    return STANDARD_TAGS.filter((tag) => !existingTags.includes(tag));
  };

  const latestVersion = prompt?.versions.sort((a, b) =>
    b.version.localeCompare(a.version),
  )[0]?.version;

  const selectedVersion = prompt?.versions.find(
    (v) => v.id === selectedVersionId,
  );

  if (!user) return null;

  return (
    <div className="min-h-screen bg-neutral-50">
      <SimpleHeader
        user={user}
        breadcrumbs={[
          { label: "Prompts", to: "/prompts" },
          { label: prompt?.name || "..." },
        ]}
      />
      <main className="max-w-7xl mx-auto px-8 py-8">
        {isLoading ? (
          <div className="text-center py-20 text-muted-foreground">
            Loading...
          </div>
        ) : error ? (
          <div className="text-center py-20 text-destructive">
            Failed to load prompt
          </div>
        ) : !prompt ? (
          <div className="text-center py-20 text-muted-foreground">
            Prompt not found
          </div>
        ) : (
          <div className="space-y-8">
            <div className="space-y-4">
              <div className="flex items-center gap-3">
                <h1 className="text-3xl font-semibold">{prompt.name}</h1>
                <span className="text-xs px-2 py-1 bg-neutral-100 text-muted-foreground rounded uppercase font-medium">
                  {prompt.prompt_type}
                </span>
              </div>
              {prompt.description && (
                <p className="text-lg text-muted-foreground max-w-3xl">
                  {prompt.description}
                </p>
              )}
            </div>

            <div>
              <div className="flex items-center justify-between mb-4">
                <h2 className="text-sm font-medium">Versions</h2>
                <Button onClick={() => setCreateDialogOpen(true)} size="sm">
                  New version
                </Button>
              </div>
              {prompt.versions.length === 0 ? (
                <div className="text-center py-20 text-muted-foreground bg-white border rounded">
                  No versions yet
                </div>
              ) : (
                <div className="bg-white border rounded overflow-hidden">
                  <table className="w-full">
                    <thead className="bg-neutral-50 border-b text-sm text-muted-foreground">
                      <tr>
                        <th className="text-left py-3 px-6 font-medium">
                          Version
                        </th>
                        <th className="text-left py-3 px-6 font-medium">
                          Tags
                        </th>
                        <th className="text-left py-3 px-6 font-medium">
                          Rating
                        </th>
                        <th className="text-left py-3 px-6 font-medium">
                          Suggestions
                        </th>
                        <th className="text-left py-3 px-6 font-medium">
                          Created
                        </th>
                        <th className="w-24"></th>
                      </tr>
                    </thead>
                    <tbody>
                      {prompt.versions.map((version) => {
                        const tags = getVersionTags(version.id);
                        const availableTags = getAvailableTags(version.id);
                        const pending =
                          version.improvement_suggestions?.filter(
                            (s) => s.status === "pending",
                          ).length || 0;
                        const isHovered = hoveredVersion === version.id;

                        return (
                          <tr
                            key={version.id}
                            className="border-b last:border-b-0 hover:bg-neutral-50 cursor-pointer"
                            onClick={() => handleViewVersion(version.id)}
                            onMouseEnter={() => setHoveredVersion(version.id)}
                            onMouseLeave={() => setHoveredVersion(null)}
                          >
                            <td className="py-4 px-6">
                              <span className="font-mono text-sm">
                                v{version.version}
                              </span>
                              {version.changelog && (
                                <div className="text-sm text-muted-foreground mt-1 line-clamp-1">
                                  {version.changelog}
                                </div>
                              )}
                            </td>
                            <td className="py-4 px-6">
                              <div className="flex gap-2 flex-wrap items-center">
                                {tags.map((tag) => (
                                  <span
                                    key={tag}
                                    className="group text-xs px-2 py-1 bg-neutral-100 rounded flex items-center gap-1 cursor-pointer"
                                    onClick={(e) => {
                                      e.stopPropagation();
                                      handleDeleteTag(tag);
                                    }}
                                  >
                                    {tag}
                                    <span className="opacity-0 group-hover:opacity-100 text-muted-foreground">
                                      ×
                                    </span>
                                  </span>
                                ))}
                                <Popover
                                  open={addingTagToVersion === version.id}
                                  onOpenChange={(open) => {
                                    if (!open) setAddingTagToVersion(null);
                                  }}
                                >
                                  <PopoverTrigger asChild>
                                    <button
                                      onClick={(e) => {
                                        e.stopPropagation();
                                        setAddingTagToVersion(version.id);
                                      }}
                                      className={`text-xs text-muted-foreground hover:text-foreground transition-opacity ${
                                        isHovered ? "opacity-100" : "opacity-0"
                                      }`}
                                    >
                                      + tag
                                    </button>
                                  </PopoverTrigger>
                                  <PopoverContent
                                    className="w-48 p-0"
                                    align="start"
                                    onClick={(e) => e.stopPropagation()}
                                  >
                                    <Command>
                                      <CommandInput placeholder="Search or type..." />
                                      <CommandList>
                                        <CommandEmpty>
                                          Type to create custom tag
                                        </CommandEmpty>
                                        {availableTags.length > 0 && (
                                          <CommandGroup heading="Standard">
                                            {availableTags.map((tag) => (
                                              <CommandItem
                                                key={tag}
                                                value={tag}
                                                onSelect={() =>
                                                  handleSelectTag(
                                                    version.id,
                                                    tag,
                                                  )
                                                }
                                              >
                                                {tag}
                                              </CommandItem>
                                            ))}
                                          </CommandGroup>
                                        )}
                                      </CommandList>
                                    </Command>
                                  </PopoverContent>
                                </Popover>
                              </div>
                            </td>
                            <td className="py-4 px-6">
                              {version.feedback_count > 0 ? (
                                <div className="flex items-center gap-2 text-sm">
                                  <Star className="h-3 w-3 fill-yellow-400 text-yellow-400" />
                                  <span>
                                    {version.average_rating?.toFixed(1)}
                                  </span>
                                  <span className="text-muted-foreground">
                                    ({version.feedback_count})
                                  </span>
                                </div>
                              ) : (
                                <span className="text-sm text-muted-foreground">
                                  —
                                </span>
                              )}
                            </td>
                            <td className="py-4 px-6">
                              {pending > 0 ? (
                                <span className="text-sm">
                                  {pending} pending
                                </span>
                              ) : (
                                <span className="text-sm text-muted-foreground">
                                  —
                                </span>
                              )}
                            </td>
                            <td className="py-4 px-6 text-sm text-muted-foreground">
                              {formatDistanceToNow(
                                new Date(version.created_at),
                                { addSuffix: true },
                              )}
                            </td>
                            <td className="py-4 px-6">
                              <button
                                onClick={(e) => {
                                  e.stopPropagation();
                                  handleDeleteVersion(
                                    version.id,
                                    version.version,
                                  );
                                }}
                                className={`text-sm text-muted-foreground hover:text-destructive transition-opacity ${
                                  isHovered ? "opacity-100" : "opacity-0"
                                }`}
                              >
                                Delete
                              </button>
                            </td>
                          </tr>
                        );
                      })}
                    </tbody>
                  </table>
                </div>
              )}
            </div>
          </div>
        )}
      </main>

      <CreateVersionDialog
        open={createDialogOpen}
        onOpenChange={setCreateDialogOpen}
        onSubmit={handleCreateVersion}
        isLoading={isCreatingVersion}
        latestVersion={latestVersion}
      />

      {selectedVersion && prompt && (
        <VersionDetailDialog
          open={viewDialogOpen}
          onOpenChange={setViewDialogOpen}
          version={selectedVersion}
          tags={getVersionTags(selectedVersion.id)}
          feedback={versionFeedback?.[selectedVersion.id] || []}
          onDeleteFeedback={handleDeleteFeedback}
          promptId={prompt.id}
        />
      )}
    </div>
  );
}
