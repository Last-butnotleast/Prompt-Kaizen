import { useMutation, useQueryClient } from "@tanstack/react-query";
import { apiClient } from "../client";

export const useCreatePrompt = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (data: { name: string; description?: string | null }) => {
      const response = await apiClient.POST("/prompts", { body: data });

      if (response.error) {
        throw new Error(response.error as string);
      }

      return response.data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["prompts"] });
    },
  });
};
