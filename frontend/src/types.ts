import type { components } from "@/api/generated/schema";

export type Prompt = components["schemas"]["PromptResponse"];
export type Version = components["schemas"]["VersionResponse"];
export type Tag = components["schemas"]["TagResponse"];
export type Feedback = components["schemas"]["FeedbackResponse"];
export type TestScenario = components["schemas"]["TestScenarioResponse"];
export type ImprovementSuggestion =
  components["schemas"]["ImprovementSuggestionResponse"];
export type ApiKey = components["schemas"]["ApiKeyResponse"];

export type CreatePromptRequest = components["schemas"]["CreatePromptRequest"];
export type CreatePromptResponse =
  components["schemas"]["CreatePromptResponse"];
export type UpdatePromptRequest = components["schemas"]["UpdatePromptRequest"];

export type CreateVersionRequest =
  components["schemas"]["CreateVersionRequest"];
export type CreateVersionResponse =
  components["schemas"]["CreateVersionResponse"];

export type RenderVersionRequest =
  components["schemas"]["RenderVersionRequest"];
export type RenderVersionResponse =
  components["schemas"]["RenderVersionResponse"];

export type TagVersionRequest = components["schemas"]["TagVersionRequest"];

export type SubmitFeedbackRequest =
  components["schemas"]["SubmitFeedbackRequest"];
export type SubmitFeedbackResponse =
  components["schemas"]["SubmitFeedbackResponse"];
export type UpdateFeedbackRequest =
  components["schemas"]["UpdateFeedbackRequest"];

export type CreateApiKeyRequest = components["schemas"]["CreateApiKeyRequest"];
export type CreateApiKeyResponse =
  components["schemas"]["CreateApiKeyResponse"];

export type CreateImprovementSuggestionRequest =
  components["schemas"]["CreateImprovementSuggestionRequest"];
export type CreateImprovementSuggestionResponse =
  components["schemas"]["CreateImprovementSuggestionResponse"];
export type AcceptImprovementSuggestionRequest =
  components["schemas"]["AcceptImprovementSuggestionRequest"];
export type AcceptImprovementSuggestionResponse =
  components["schemas"]["AcceptImprovementSuggestionResponse"];
export type DeclineImprovementSuggestionRequest =
  components["schemas"]["DeclineImprovementSuggestionRequest"];
export type AnalyzeFeedbackResponse =
  components["schemas"]["AnalyzeFeedbackResponse"];
