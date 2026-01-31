import { useState } from "react";
import {
  useListApiKeys,
  useCreateApiKey,
  useDeleteApiKey,
} from "@/api/hooks/api-keys";
import { SettingsDisplay } from "./SettingsDisplay";
import { DashboardLayout } from "@/components/layout/DashboardLayout";

export function SettingsManager() {
  const [isCreateDialogOpen, setIsCreateDialogOpen] = useState(false);
  const [createdKey, setCreatedKey] = useState<string | null>(null);

  const { data: apiKeys, isLoading } = useListApiKeys();
  const createMutation = useCreateApiKey();
  const deleteMutation = useDeleteApiKey();

  const handleCreate = async (name: string) => {
    const result = await createMutation.mutateAsync({ name });
    setCreatedKey(result.api_key);
    setIsCreateDialogOpen(false);
  };

  const handleDelete = async (id: string) => {
    await deleteMutation.mutateAsync(id);
  };

  const handleCloseKeyDialog = () => {
    setCreatedKey(null);
  };

  return (
    <DashboardLayout title="Settings">
      <SettingsDisplay
        apiKeys={apiKeys || []}
        isLoading={isLoading}
        isCreateDialogOpen={isCreateDialogOpen}
        setIsCreateDialogOpen={setIsCreateDialogOpen}
        createdKey={createdKey}
        onCreateKey={handleCreate}
        onDeleteKey={handleDelete}
        onCloseKeyDialog={handleCloseKeyDialog}
        isCreating={createMutation.isPending}
        isDeleting={deleteMutation.isPending}
      />
    </DashboardLayout>
  );
}
