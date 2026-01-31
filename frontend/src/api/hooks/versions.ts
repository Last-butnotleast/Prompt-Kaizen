import { useMutation, useQueryClient } from "@tanstack/react-query";
import { apiClient } from "../client";

export const useCreateVersion = (promptId: string) => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (data: {
      version: string;
      content: string;
      changelog?: string | null;
    }) => {
      const response = await apiClient.POST("/prompts/{prompt_id}/versions", {
        params: { path: { prompt_id: promptId } },
        body: data,
      });

      if (response.error) {
        throw new Error(response.error as string);
      }

      return response.data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["prompts", promptId] });
      queryClient.invalidateQueries({ queryKey: ["versions", promptId] });
    },
  });
};
