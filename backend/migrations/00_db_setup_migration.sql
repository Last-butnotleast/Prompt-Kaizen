-- Prompts table
CREATE TABLE prompts (
                         id UUID PRIMARY KEY,
                         user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
                         name VARCHAR(255) NOT NULL,
                         description TEXT,
                         prompt_type VARCHAR(20) NOT NULL DEFAULT 'system',
                         created_at TIMESTAMPTZ NOT NULL,
                         updated_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_prompts_user_id ON prompts(user_id);

-- Versions table
CREATE TABLE versions (
                          id UUID PRIMARY KEY,
                          prompt_id UUID NOT NULL REFERENCES prompts(id) ON DELETE CASCADE,
                          version VARCHAR(50) NOT NULL,
                          digest VARCHAR(255) NOT NULL,
                          content TEXT NOT NULL,
                          content_type VARCHAR(20) NOT NULL DEFAULT 'static',
                          variables JSONB,
                          changelog TEXT,
                          created_at TIMESTAMPTZ NOT NULL,
                          UNIQUE(prompt_id, version)
);

CREATE INDEX idx_versions_prompt_id ON versions(prompt_id);

-- Tags table
CREATE TABLE tags (
                      id UUID PRIMARY KEY,
                      prompt_id UUID NOT NULL REFERENCES prompts(id) ON DELETE CASCADE,
                      version_id UUID NOT NULL REFERENCES versions(id) ON DELETE CASCADE,
                      name VARCHAR(100) NOT NULL,
                      updated_at TIMESTAMPTZ NOT NULL,
                      UNIQUE(prompt_id, name)
);

CREATE INDEX idx_tags_prompt_id ON tags(prompt_id);
CREATE INDEX idx_tags_name ON tags(name);

-- Feedbacks table
CREATE TABLE feedbacks (
                           id UUID PRIMARY KEY,
                           version_id UUID NOT NULL REFERENCES versions(id) ON DELETE CASCADE,
                           rating SMALLINT NOT NULL CHECK (rating >= 1 AND rating <= 5),
                           comment TEXT,
                           test_input TEXT,
                           test_actual_output TEXT,
                           test_expected_output TEXT,
                           created_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_feedbacks_version_id ON feedbacks(version_id);

-- API Keys table
CREATE TABLE api_keys (
                          id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                          user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
                          name VARCHAR(255) NOT NULL,
                          key_hash VARCHAR(255) NOT NULL UNIQUE,
                          key_prefix VARCHAR(50) NOT NULL,
                          last_used_at TIMESTAMPTZ,
                          created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                          is_active BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE INDEX idx_api_keys_key_hash ON api_keys(key_hash);
CREATE INDEX idx_api_keys_user_id ON api_keys(user_id);

ALTER TABLE api_keys ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Users can view own API keys"
    ON api_keys FOR SELECT USING (auth.uid() = user_id);

CREATE POLICY "Users can create own API keys"
    ON api_keys FOR INSERT WITH CHECK (auth.uid() = user_id);

CREATE POLICY "Users can delete own API keys"
    ON api_keys FOR DELETE USING (auth.uid() = user_id);

CREATE POLICY "Users can update own API keys"
    ON api_keys FOR UPDATE USING (auth.uid() = user_id);

-- Improvement Suggestions table
CREATE TABLE improvement_suggestions (
                                         id UUID PRIMARY KEY,
                                         source_version_id UUID NOT NULL REFERENCES versions(id) ON DELETE CASCADE,
                                         suggested_content TEXT NOT NULL,
                                         ai_rationale TEXT NOT NULL,
                                         status VARCHAR(20) NOT NULL CHECK (status IN ('pending', 'accepted', 'declined')),
                                         decline_reason TEXT,
                                         created_at TIMESTAMPTZ NOT NULL,
                                         resolved_at TIMESTAMPTZ,
                                         resulting_version_id UUID REFERENCES versions(id) ON DELETE SET NULL
);

CREATE INDEX idx_improvement_suggestions_source_version ON improvement_suggestions(source_version_id);
CREATE INDEX idx_improvement_suggestions_status ON improvement_suggestions(status);
CREATE INDEX idx_improvement_suggestions_resulting_version ON improvement_suggestions(resulting_version_id);