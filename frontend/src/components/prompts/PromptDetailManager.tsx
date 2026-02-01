import { useParams, useNavigate } from "@tanstack/react-router";
import {
  useGetPrompt,
  useUpdatePrompt,
  useDeletePrompt,
} from "@/api/hooks/prompts";
import { useCreateVersion, useDeleteVersion } from "@/api/hooks/versions";
import { useTagVersion, useDeleteTag } from "@/api/hooks/tags";
import { useSubmitFeedback, useDeleteFeedback } from "@/api/hooks/feedback";
import { useAnalyzeFeedback } from "@/api/hooks/improvements";
import { PromptDetailDisplay } from "./PromptDetailDisplay";
import type {
  UpdatePromptRequest,
  CreateVersionRequest,
  TagVersionRequest,
  SubmitFeedbackRequest,
  Feedback,
} from "@/types";
import { useMemo, useState } from "react";

export function PromptDetailManager() {
  const { promptId } = useParams({ from: "/prompts/$promptId" });
  const navigate = useNavigate();
  const [deletingFeedback, setDeletingFeedback] = useState<{
    versionId: string;
    feedbackId: string;
  } | null>(null);
  const [analyzingVersionId, setAnalyzingVersionId] = useState<string | null>(
    null,
  );

  const { data: prompt, isLoading, error } = useGetPrompt(promptId);
  const updatePrompt = useUpdatePrompt(promptId);
  const deletePrompt = useDeletePrompt();
  const createVersion = useCreateVersion(promptId);
  const deleteVersion = useDeleteVersion(promptId);
  const tagVersion = useTagVersion(promptId);
  const deleteTag = useDeleteTag(promptId);
  const submitFeedback = useSubmitFeedback(promptId);
  const deleteFeedbackMutation = useDeleteFeedback(
    promptId,
    deletingFeedback?.versionId || "",
  );
  const analyzeFeedback = useAnalyzeFeedback(
    promptId,
    analyzingVersionId || "",
  );

  const versionFeedback = useMemo(() => {
    if (!prompt?.versions) return {};
    const feedbackMap: Record<string, Feedback[]> = {};
    prompt.versions.forEach((version) => {
      feedbackMap[version.id] = version.feedback || [];
    });
    return feedbackMap;
  }, [prompt]);

  const handleUpdatePrompt = async (data: UpdatePromptRequest) => {
    await updatePrompt.mutateAsync(data);
  };

  const handleDeletePrompt = async () => {
    await deletePrompt.mutateAsync(promptId);
    navigate({ to: "/prompts" });
  };

  const handleCreateVersion = async (data: CreateVersionRequest) => {
    await createVersion.mutateAsync(data);
  };

  const handleDeleteVersion = async (versionId: string) => {
    await deleteVersion.mutateAsync(versionId);
  };

  const handleTagVersion = async (data: TagVersionRequest) => {
    await tagVersion.mutateAsync(data);
  };

  const handleDeleteTag = async (tagName: string) => {
    await deleteTag.mutateAsync(tagName);
  };

  const handleSubmitFeedback = async (data: SubmitFeedbackRequest) => {
    await submitFeedback.mutateAsync(data);
  };

  const handleDeleteFeedback = async (
    versionId: string,
    feedbackId: string,
  ) => {
    setDeletingFeedback({ versionId, feedbackId });
    await deleteFeedbackMutation.mutateAsync(feedbackId);
    setDeletingFeedback(null);
  };

  const handleAnalyzeFeedback = async (versionId: string) => {
    setAnalyzingVersionId(versionId);
    await analyzeFeedback.mutateAsync();
    setAnalyzingVersionId(null);
  };

  const handleBack = () => {
    navigate({ to: "/prompts" });
  };

  return (
    <PromptDetailDisplay
      prompt={prompt}
      isLoading={isLoading}
      error={error}
      onUpdatePrompt={handleUpdatePrompt}
      onDeletePrompt={handleDeletePrompt}
      onCreateVersion={handleCreateVersion}
      onDeleteVersion={handleDeleteVersion}
      onTagVersion={handleTagVersion}
      onDeleteTag={handleDeleteTag}
      onSubmitFeedback={handleSubmitFeedback}
      onDeleteFeedback={handleDeleteFeedback}
      onAnalyzeFeedback={handleAnalyzeFeedback}
      onBack={handleBack}
      isUpdating={updatePrompt.isPending}
      isDeleting={deletePrompt.isPending}
      isCreatingVersion={createVersion.isPending}
      isTagging={tagVersion.isPending}
      isSubmittingFeedback={submitFeedback.isPending}
      isAnalyzing={analyzeFeedback.isPending}
      analyzingVersionId={analyzingVersionId}
      versionFeedback={versionFeedback}
    />
  );
}
