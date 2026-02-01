````markdown
# n8n-nodes-prompt-kaizen

n8n node for [Prompt Kaizen](https://last-butnotleast.com) - retrieve and use versioned AI prompts with feedback capabilities.

## Configuration

1. Get your API key from Prompt Kaizen dashboard
2. Add **Prompt Kaizen API** credentials in n8n:
   - **API Key**: Your Prompt Kaizen API key
   - **Base URL**: `http://localhost:3000` (default)

## Features

### Get Prompt

- Retrieve prompts by latest version, specific version, or tag
- Auto-renders templates with context variables
- Returns rendered content + metadata

### Submit Feedback

- Rate prompts (1-5 stars)
- Add comments and test scenarios
- Track prompt performance

## Usage Example

**Get a prompt with variables:**

```
Operation: Get Prompt
Prompt: My AI Assistant
Version Selection: By Tag → "production"
Context Variables: {{ { "user_name": "Alice", "topic": "sales" } }}
```

**Submit feedback:**

```
Operation: Submit Feedback
Version: v1.2.0
Rating: 5
Comment: "Works perfectly for customer support"
```

## Output

**Get Prompt returns:**

```json
{
  "content": "Hello Alice, let's discuss sales...",
  "raw_content": "Hello {{user_name}}, let's discuss {{topic}}...",
  "version": "1.2.0",
  "prompt_type": "system",
  "content_type": "template",
  "variables": ["user_name", "topic"]
}
```

## Links

- [Prompt Kaizen](https://last-butnotleast.com)
- [API Documentation](https://github.com/Last-butnotleast/Prompt-Kaizen)
- [Report Issues](https://github.com/Last-butnotleast/Prompt-Kaizen)
````
