use crate::application::AIService;
use crate::domain::prompt::{Feedback, ContentType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub struct OpenAIService {
    api_key: String,
    client: reqwest::Client,
}

impl OpenAIService {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    response_format: ResponseFormat,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ResponseFormat {
    #[serde(rename = "type")]
    format_type: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

#[derive(Deserialize)]
struct AIAnalysisResult {
    suggested_content: String,
    rationale: String,
}

#[async_trait]
impl AIService for OpenAIService {
    async fn analyze_feedback_and_suggest(
        &self,
        prompt_content: &str,
        content_type: ContentType,
        feedbacks: &[Feedback],
    ) -> Result<(String, String), String> {
        let avg_rating = if !feedbacks.is_empty() {
            feedbacks.iter().map(|f| f.rating() as f64).sum::<f64>() / feedbacks.len() as f64
        } else {
            0.0
        };

        let content_type_str = match content_type {
            ContentType::Static => "static",
            ContentType::Template => "template (Handlebars)",
        };

        let mut feedback_details = String::new();
        for (i, fb) in feedbacks.iter().enumerate() {
            feedback_details.push_str(&format!("\n{}. Rating: {}/5", i + 1, fb.rating()));
            if let Some(comment) = fb.comment() {
                feedback_details.push_str(&format!("\n   Comment: {}", comment));
            }
            if let Some(ts) = fb.test_scenario() {
                feedback_details.push_str(&format!(
                    "\n   Test Case:\n     Input: {}\n     Actual Output: {}",
                    ts.input(), ts.actual_output()
                ));
                if let Some(expected) = ts.expected_output() {
                    feedback_details.push_str(&format!("\n     Expected Output: {}", expected));
                }
            }
        }

        let system_prompt = "You are a prompt engineering expert. Analyze user feedback on prompts and suggest concrete improvements. Output ONLY valid JSON with this exact structure: {\"suggested_content\": \"improved prompt text\", \"rationale\": \"explanation of changes\"}";

        let user_prompt = format!(
            "Current Prompt:\n---\n{}\n---\nType: {}\n\nFeedback Summary:\n- Average Rating: {:.1}/5\n- Total Feedback: {}\n\nDetailed Feedback:{}\n\nAnalyze the feedback and suggest an improved version of the prompt that addresses the issues. Focus on:\n1. Low ratings and specific complaints\n2. Test case failures if present\n3. Clarity and effectiveness\n\nProvide the improved prompt and explain your changes.",
            prompt_content,
            content_type_str,
            avg_rating,
            feedbacks.len(),
            feedback_details
        );

        let request = ChatRequest {
            model: "gpt-4o-mini".to_string(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: user_prompt,
                },
            ],
            temperature: 0.7,
            response_format: ResponseFormat {
                format_type: "json_object".to_string(),
            },
        };

        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("OpenAI API request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!("OpenAI API error {}: {}", status, body));
        }

        let chat_response: ChatResponse = response.json().await
            .map_err(|e| format!("Failed to parse OpenAI response: {}", e))?;

        let content = chat_response.choices
            .first()
            .ok_or("No choices in OpenAI response")?
            .message
            .content
            .clone();

        let result: AIAnalysisResult = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse AI analysis result: {}", e))?;

        Ok((result.suggested_content, result.rationale))
    }
}