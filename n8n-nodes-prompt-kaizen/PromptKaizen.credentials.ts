import type { ICredentialType, INodeProperties } from "n8n-workflow";

export class PromptKaizenApi implements ICredentialType {
  name = "promptKaizenApi";
  displayName = "Prompt Kaizen API";
  documentationUrl = "https://github.com/yourusername/prompt-kaizen";
  properties: INodeProperties[] = [
    {
      displayName: "API Key",
      name: "apiKey",
      type: "string",
      typeOptions: { password: true },
      default: "",
      required: true,
    },
    {
      displayName: "Base URL",
      name: "baseUrl",
      type: "string",
      default: "http://localhost:3000",
      required: true,
    },
  ];
}
