-- Prompts table
CREATE TABLE prompts (
                         id UUID PRIMARY KEY,
                         user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
                         name VARCHAR(255) NOT NULL,
                         description TEXT,
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
                           created_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_feedbacks_version_id ON feedbacks(version_id);