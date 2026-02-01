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