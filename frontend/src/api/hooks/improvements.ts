import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { apiClient } from "../client";
import type {
  CreateImprovementSuggestionRequest,
  AcceptImprovementSuggestionRequest,
  DeclineImprovementSuggestionRequest,
} from "@/types";

export const useCreateImprovementSuggestion = (promptId: string) => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (data: CreateImprovementSuggestionRequest) => {
      const response = await apiClient.POST(
        "/prompts/{prompt_id}/improvements",
        {
          params: { path: { prompt_id: promptId } },
          body: data,
        },
      );
      if (response.error) throw new Error(response.error as string);
      return response.data!;
    },
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({
        queryKey: ["improvements", promptId, variables.version_id],
      });
      queryClient.invalidateQueries({
        queryKey: ["versions", promptId, variables.version_id],
      });
    },
  });
};

export const useListSuggestionsForVersion = (
  promptId: string,
  versionId: string,
) => {
  return useQuery({
    queryKey: ["improvements", promptId, versionId],
    queryFn: async () => {
      const response = await apiClient.GET(
        "/prompts/{prompt_id}/versions/{version_id}/improvements",
        {
          params: { path: { prompt_id: promptId, version_id: versionId } },
        },
      );
      if (response.error) throw new Error(response.error as string);
      return response.data!;
    },
  });
};

export const useAcceptImprovementSuggestion = (
  promptId: string,
  versionId: string,
) => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (
      data: AcceptImprovementSuggestionRequest & { suggestion_id: string },
    ) => {
      const { suggestion_id, ...body } = data;
      const response = await apiClient.POST(
        "/prompts/{prompt_id}/versions/{version_id}/improvements/{suggestion_id}/accept",
        {
          params: {
            path: { prompt_id: promptId, version_id: versionId, suggestion_id },
          },
          body,
        },
      );
      if (response.error) throw new Error(response.error as string);
      return response.data!;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["prompts", promptId] });
      queryClient.invalidateQueries({ queryKey: ["versions", promptId] });
      queryClient.invalidateQueries({
        queryKey: ["improvements", promptId, versionId],
      });
    },
  });
};

export const useDeclineImprovementSuggestion = (
  promptId: string,
  versionId: string,
) => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (
      data: DeclineImprovementSuggestionRequest & { suggestion_id: string },
    ) => {
      const { suggestion_id, ...body } = data;
      const response = await apiClient.POST(
        "/prompts/{prompt_id}/versions/{version_id}/improvements/{suggestion_id}/decline",
        {
          params: {
            path: { prompt_id: promptId, version_id: versionId, suggestion_id },
          },
          body,
        },
      );
      if (response.error) throw new Error(response.error as string);
      return response.data!;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ["improvements", promptId, versionId],
      });
    },
  });
};

export const useAnalyzeFeedback = (promptId: string, versionId: string) => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async () => {
      const response = await apiClient.POST(
        "/prompts/{prompt_id}/versions/{version_id}/analyze-feedback",
        {
          params: { path: { prompt_id: promptId, version_id: versionId } },
        },
      );
      if (response.error) throw new Error(response.error as string);
      return response.data!;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ["improvements", promptId, versionId],
      });
    },
  });
};
