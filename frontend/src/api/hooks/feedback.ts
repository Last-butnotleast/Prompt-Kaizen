import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { apiClient } from "../client";

export const useSubmitFeedback = (promptId: string) => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (data: {
      version_id: string;
      rating: number;
      comment?: string | null;
    }) => {
      const response = await apiClient.POST("/prompts/{prompt_id}/feedback", {
        params: { path: { prompt_id: promptId } },
        body: data,
      });
      if (response.error) throw new Error(response.error as string);
      return response.data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["prompts", promptId] });
      queryClient.invalidateQueries({ queryKey: ["feedback", promptId] });
    },
  });
};

export const useUpdateFeedback = (promptId: string, versionId: string) => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (data: {
      feedback_id: string;
      rating?: number | null;
      comment?: string | null;
    }) => {
      const { feedback_id, ...body } = data;
      const response = await apiClient.PUT(
        "/prompts/{prompt_id}/versions/{version_id}/feedback/{feedback_id}",
        {
          params: {
            path: { prompt_id: promptId, version_id: versionId, feedback_id },
          },
          body,
        },
      );
      if (response.error) throw new Error(response.error as string);
      return response.data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["prompts", promptId] });
      queryClient.invalidateQueries({
        queryKey: ["feedback", promptId, versionId],
      });
    },
  });
};

export const useDeleteFeedback = (promptId: string, versionId: string) => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (feedbackId: string) => {
      const response = await apiClient.DELETE(
        "/prompts/{prompt_id}/versions/{version_id}/feedback/{feedback_id}",
        {
          params: {
            path: {
              prompt_id: promptId,
              version_id: versionId,
              feedback_id: feedbackId,
            },
          },
        },
      );
      if (response.error) throw new Error(response.error as string);
      return response.data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["prompts", promptId] });
      queryClient.invalidateQueries({
        queryKey: ["feedback", promptId, versionId],
      });
    },
  });
};
