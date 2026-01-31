import type { components } from "./api/generated/schema";

export type Prompt = components["schemas"]["PromptResponse"];
export type Version = components["schemas"]["VersionResponse"];
export type Tag = components["schemas"]["TagResponse"];
export type Feedback = components["schemas"]["FeedbackResponse"];
export type CreatePromptRequest = components["schemas"]["CreatePromptRequest"];
export type CreatePromptResponse =
  components["schemas"]["CreatePromptResponse"];
export type UpdatePromptRequest = components["schemas"]["UpdatePromptRequest"];
export type CreateVersionRequest =
  components["schemas"]["CreateVersionRequest"];
export type CreateVersionResponse =
  components["schemas"]["CreateVersionResponse"];
export type TagVersionRequest = components["schemas"]["TagVersionRequest"];
export type SubmitFeedbackRequest =
  components["schemas"]["SubmitFeedbackRequest"];
export type SubmitFeedbackResponse =
  components["schemas"]["SubmitFeedbackResponse"];
export type UpdateFeedbackRequest =
  components["schemas"]["UpdateFeedbackRequest"];
