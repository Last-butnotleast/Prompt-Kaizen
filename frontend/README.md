# Prompt Kaizen

**Continuous Improvement Platform for AI Prompts**

## Overview

Prompt Kaizen applies continuous improvement methodology to prompt engineering. Version, test, and systematically improve your AI prompts based on real user feedback and AI-driven insights.

## The Problem

Teams struggle to maintain prompt quality over time. Changes are ad-hoc, there's no visibility into what works, and improvements rely on guesswork rather than data.

## Core Features

### 1. **Prompt Version Control**

- Docker-style versioning (e.g., `0.0.1`, `0.1.2`)
- Tag support (e.g., `latest`, `production`, `experimental`)
- Unique digest generated for each version
- Full version history with changelog

### 2. **Feedback Collection**

- Capture user ratings (1-5) on AI responses
- Link feedback directly to prompt versions and digests
- Optional qualitative comments
- Aggregate feedback per version

### 3. **AI-Powered Improvement Engine**

- Analyze aggregated feedback patterns
- Generate data-driven improvement suggestions
- Explain reasoning behind each recommendation

### 4. **Decision Records**

- Document why each version was created
- Track rationale for changes
- Reference feedback that triggered improvements

## Integration Methods

- **REST API** - Programmatic access to prompts and feedback submission
- **MCP (Model Context Protocol)** - Direct integration with AI systems
- **n8n** - Workflow automation and feedback pipelines

## Architecture

### Domain Model

**Prompt** (Aggregate Root)

- Manages versions, tags, and feedback
- Versions have unique digests (SHA256 of content)
- Tags point to specific versions
- Feedback linked to versions

### API Layer

- **OpenAPI Spec** - Single source of truth for API contracts
- **Type Generation** - Auto-generated TypeScript types via `openapi-typescript`
- **Type-Safe Client** - `openapi-fetch` + `openapi-react-query` for fully typed React hooks
- **API Hooks** - Custom hooks per domain (prompts, versions, tags, feedback)

### File Structure

```
api/
  generated/
    schema.d.ts          # Auto-generated from OpenAPI spec
  hooks/                 # React Query hooks per domain
    prompts.ts
    versions.ts
    tags.ts
    feedback.ts
  client.ts              # Configured openapi-fetch client
components/
  layout/
  pages/
  sidebar/
routes/                  # TanStack Router file-based routes
types/
  index.ts               # App-specific types
```

### State & Navigation

**Server State** (React Query):

- All domain data managed via openapi-react-query hooks
- Automatic caching, refetching, invalidation

**Navigation** (TanStack Router):

- URL-driven, type-safe routing
- File-based route definitions

### Data Flow

```
Component → API Hook → openapi-react-query → Backend API
Component → useNavigate → TanStack Router → URL
```

## Development

```bash
pnpm install
pnpm run dev
```

Backend API expected at `http://localhost:3000`

## Tech Stack

- **React** + **TypeScript**
- **TanStack Router** - Type-safe routing
- **TanStack Query** (via openapi-react-query) - Server state
- **Tailwind CSS** + **shadcn/ui**
- **openapi-fetch** - Type-safe API client
