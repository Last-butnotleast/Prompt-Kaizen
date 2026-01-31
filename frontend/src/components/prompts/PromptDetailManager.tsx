import { useParams, useNavigate } from "@tanstack/react-router";
import {
  useGetPrompt,
  useUpdatePrompt,
  useDeletePrompt,
} from "@/api/hooks/prompts";
import { useCreateVersion, useDeleteVersion } from "@/api/hooks/versions";
import { useTagVersion, useDeleteTag } from "@/api/hooks/tags";
import { useSubmitFeedback } from "@/api/hooks/feedback";
import { PromptDetailDisplay } from "./PromptDetailDisplay";
import type {
  UpdatePromptRequest,
  CreateVersionRequest,
  TagVersionRequest,
  SubmitFeedbackRequest,
} from "@/types";

export function PromptDetailManager() {
  const { promptId } = useParams({ from: "/prompts/$promptId" });
  const navigate = useNavigate();

  const { data: prompt, isLoading, error } = useGetPrompt(promptId);
  const updatePrompt = useUpdatePrompt(promptId);
  const deletePrompt = useDeletePrompt();
  const createVersion = useCreateVersion(promptId);
  const deleteVersion = useDeleteVersion(promptId);
  const tagVersion = useTagVersion(promptId);
  const deleteTag = useDeleteTag(promptId);
  const submitFeedback = useSubmitFeedback(promptId);

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
      onBack={handleBack}
      isUpdating={updatePrompt.isPending}
      isDeleting={deletePrompt.isPending}
      isCreatingVersion={createVersion.isPending}
      isTagging={tagVersion.isPending}
    />
  );
}
