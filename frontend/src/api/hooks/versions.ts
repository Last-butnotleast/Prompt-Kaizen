import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { apiClient } from "../client";
import type { CreateVersionRequest } from "@/types";

export const useGetVersion = (promptId: string, versionId: string) => {
  return useQuery({
    queryKey: ["versions", promptId, versionId],
    queryFn: async () => {
      const response = await apiClient.GET(
        "/prompts/{prompt_id}/versions/{version_id}",
        {
          params: { path: { prompt_id: promptId, version_id: versionId } },
        },
      );
      if (response.error) throw new Error(response.error as string);
      return response.data!;
    },
  });
};

export const useCreateVersion = (promptId: string) => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (data: CreateVersionRequest) => {
      const response = await apiClient.POST("/prompts/{prompt_id}/versions", {
        params: { path: { prompt_id: promptId } },
        body: data,
      });
      if (response.error) throw new Error(response.error as string);
      return response.data!;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["prompts", promptId] });
      queryClient.invalidateQueries({ queryKey: ["versions", promptId] });
    },
  });
};

export const useDeleteVersion = (promptId: string) => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (versionId: string) => {
      const response = await apiClient.DELETE(
        "/prompts/{prompt_id}/versions/{version_id}",
        {
          params: { path: { prompt_id: promptId, version_id: versionId } },
        },
      );
      if (response.error) throw new Error(response.error as string);
      return response.data!;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["prompts", promptId] });
      queryClient.invalidateQueries({ queryKey: ["versions", promptId] });
    },
  });
};
