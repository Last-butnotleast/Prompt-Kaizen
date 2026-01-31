import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { apiClient } from "../client";

export const useGetVersionByTag = (promptId: string, tagName: string) => {
  return useQuery({
    queryKey: ["tags", promptId, tagName],
    queryFn: async () => {
      const response = await apiClient.GET(
        "/prompts/{prompt_id}/tags/{tag_name}/version",
        {
          params: { path: { prompt_id: promptId, tag_name: tagName } },
        },
      );
      if (response.error) throw new Error(response.error as string);
      return response.data;
    },
  });
};

export const useTagVersion = (promptId: string) => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (data: { tag_name: string; version_id: string }) => {
      const response = await apiClient.POST("/prompts/{prompt_id}/tags", {
        params: { path: { prompt_id: promptId } },
        body: data,
      });
      if (response.error) throw new Error(response.error as string);
      return response.data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["prompts", promptId] });
      queryClient.invalidateQueries({ queryKey: ["tags", promptId] });
    },
  });
};

export const useDeleteTag = (promptId: string) => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (tagName: string) => {
      const response = await apiClient.DELETE(
        "/prompts/{prompt_id}/tags/{tag_name}",
        {
          params: { path: { prompt_id: promptId, tag_name: tagName } },
        },
      );
      if (response.error) throw new Error(response.error as string);
      return response.data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["prompts", promptId] });
      queryClient.invalidateQueries({ queryKey: ["tags", promptId] });
    },
  });
};
