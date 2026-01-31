import { useMutation, useQueryClient } from "@tanstack/react-query";
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

      if (response.error) {
        throw new Error(response.error as string);
      }

      return response.data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["feedback", promptId] });
      queryClient.invalidateQueries({ queryKey: ["versions", promptId] });
    },
  });
};
