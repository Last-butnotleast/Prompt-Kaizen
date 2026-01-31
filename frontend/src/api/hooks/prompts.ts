import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { apiClient } from "../client";
import type { CreatePromptResponse } from "@/types";

export const useListPrompts = () => {
  return useQuery({
    queryKey: ["prompts"],
    queryFn: async () => {
      const response = await apiClient.GET("/prompts");
      if (response.error) throw new Error(response.error as string);
      return response.data!;
    },
  });
};

export const useGetPrompt = (promptId: string) => {
  return useQuery({
    queryKey: ["prompts", promptId],
    queryFn: async () => {
      const response = await apiClient.GET("/prompts/{prompt_id}", {
        params: { path: { prompt_id: promptId } },
      });
      if (response.error) throw new Error(response.error as string);
      return response.data!;
    },
  });
};

export const useCreatePrompt = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (data: { name: string; description?: string | null }) => {
      const response = await apiClient.POST("/prompts", { body: data });
      if (response.error) throw new Error(response.error as string);
      return response.data as CreatePromptResponse;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["prompts"] });
    },
  });
};

export const useUpdatePrompt = (promptId: string) => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (data: {
      name?: string | null;
      description?: string | null;
    }) => {
      const response = await apiClient.PUT("/prompts/{prompt_id}", {
        params: { path: { prompt_id: promptId } },
        body: data,
      });
      if (response.error) throw new Error(response.error as string);
      return response.data!;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["prompts"] });
      queryClient.invalidateQueries({ queryKey: ["prompts", promptId] });
    },
  });
};

export const useDeletePrompt = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (promptId: string) => {
      const response = await apiClient.DELETE("/prompts/{prompt_id}", {
        params: { path: { prompt_id: promptId } },
      });
      if (response.error) throw new Error(response.error as string);
      return response.data!;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["prompts"] });
    },
  });
};
