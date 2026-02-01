import { useState } from "react";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";
import type { CreatePromptRequest } from "@/types";

interface CreatePromptDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onSubmit: (data: CreatePromptRequest) => Promise<void>;
  isLoading: boolean;
}

export function CreatePromptDialog({
  open,
  onOpenChange,
  onSubmit,
  isLoading,
}: CreatePromptDialogProps) {
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [promptType, setPromptType] = useState<"system" | "user">("user");
  const [error, setError] = useState("");

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");

    if (!name.trim()) {
      setError("Name is required");
      return;
    }

    try {
      await onSubmit({
        name: name.trim(),
        description: description.trim() || null,
        prompt_type: promptType,
      });
      setName("");
      setDescription("");
      setPromptType("user");
    } catch (err) {
      setError("Failed to create prompt");
    }
  };

  const handleOpenChange = (newOpen: boolean) => {
    if (!isLoading) {
      onOpenChange(newOpen);
      if (!newOpen) {
        setName("");
        setDescription("");
        setPromptType("user");
        setError("");
      }
    }
  };

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogContent>
        <form onSubmit={handleSubmit}>
          <DialogHeader>
            <DialogTitle>Create New Prompt</DialogTitle>
            <DialogDescription>
              Create a new prompt to start tracking versions and feedback.
            </DialogDescription>
          </DialogHeader>
          <div className="grid gap-4 py-4">
            <div className="grid gap-2">
              <Label htmlFor="name">Name *</Label>
              <Input
                id="name"
                value={name}
                onChange={(e) => setName(e.target.value)}
                placeholder="e.g., Customer Support Assistant"
                disabled={isLoading}
                autoFocus
              />
            </div>

            <div className="grid gap-2">
              <Label>Prompt Type *</Label>
              <RadioGroup
                value={promptType}
                onValueChange={(value) =>
                  setPromptType(value as "system" | "user")
                }
                disabled={isLoading}
              >
                <div className="flex items-center space-x-2">
                  <RadioGroupItem value="system" id="system" />
                  <Label
                    htmlFor="system"
                    className="font-normal cursor-pointer"
                  >
                    System - Instructions for AI behavior
                  </Label>
                </div>
                <div className="flex items-center space-x-2">
                  <RadioGroupItem value="user" id="user" />
                  <Label htmlFor="user" className="font-normal cursor-pointer">
                    User - User-facing prompts
                  </Label>
                </div>
              </RadioGroup>
            </div>

            <div className="grid gap-2">
              <Label htmlFor="description">Description</Label>
              <Textarea
                id="description"
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                placeholder="Brief description of the prompt's purpose..."
                disabled={isLoading}
                rows={3}
              />
            </div>
            {error && <div className="text-sm text-destructive">{error}</div>}
          </div>
          <DialogFooter>
            <Button
              type="button"
              variant="outline"
              onClick={() => handleOpenChange(false)}
              disabled={isLoading}
            >
              Cancel
            </Button>
            <Button type="submit" disabled={isLoading}>
              {isLoading ? "Creating..." : "Create Prompt"}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
