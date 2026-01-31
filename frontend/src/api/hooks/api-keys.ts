import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { apiClient } from "../client";
import type { CreateApiKeyResponse } from "@/types";

export const useListApiKeys = () => {
  return useQuery({
    queryKey: ["api-keys"],
    queryFn: async () => {
      const response = await apiClient.GET("/api-keys");
      if (response.error) throw new Error(response.error as string);
      return response.data!;
    },
  });
};

export const useCreateApiKey = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (data: { name: string }) => {
      const response = await apiClient.POST("/api-keys", { body: data });
      if (response.error) throw new Error(response.error as string);
      return response.data as CreateApiKeyResponse;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["api-keys"] });
    },
  });
};

export const useDeleteApiKey = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (apiKeyId: string) => {
      const response = await apiClient.DELETE("/api-keys/{api_key_id}", {
        params: { path: { api_key_id: apiKeyId } },
      });
      if (response.error) throw new Error(response.error as string);
      return response.data!;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["api-keys"] });
    },
  });
};
