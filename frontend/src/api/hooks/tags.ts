import { useMutation, useQueryClient } from "@tanstack/react-query";
import { apiClient } from "../client";

export const useTagVersion = (promptId: string) => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (data: {
      tag_name: "latest" | "production" | "experimental";
      version_id: string;
    }) => {
      const response = await apiClient.POST("/prompts/{prompt_id}/tags", {
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
      queryClient.invalidateQueries({ queryKey: ["tags", promptId] });
    },
  });
};
