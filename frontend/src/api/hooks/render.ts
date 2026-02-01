import { useMutation } from "@tanstack/react-query";
import { apiClient } from "../client";
import type { RenderVersionRequest } from "@/types";

export const useRenderVersion = (promptId: string, versionId: string) => {
  return useMutation({
    mutationFn: async (data: RenderVersionRequest) => {
      const response = await apiClient.POST(
        "/prompts/{prompt_id}/versions/{version_id}/render",
        {
          params: { path: { prompt_id: promptId, version_id: versionId } },
          body: data,
        },
      );
      if (response.error) throw new Error(response.error as string);
      return response.data!;
    },
  });
};

export const useRenderVersionByTag = (promptId: string, tagName: string) => {
  return useMutation({
    mutationFn: async (data: RenderVersionRequest) => {
      const response = await apiClient.POST(
        "/prompts/{prompt_id}/tags/{tag_name}/render",
        {
          params: { path: { prompt_id: promptId, tag_name: tagName } },
          body: data,
        },
      );
      if (response.error) throw new Error(response.error as string);
      return response.data!;
    },
  });
};
