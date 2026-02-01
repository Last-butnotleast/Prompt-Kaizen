#[derive(Debug, Clone)]
pub struct TestScenario {
    input: String,
    actual_output: String,
    expected_output: Option<String>,
}

impl TestScenario {
    pub fn new(
        input: String,
        actual_output: String,
        expected_output: Option<String>,
    ) -> Result<Self, String> {
        if input.trim().is_empty() {
            return Err("Input cannot be empty".to_string());
        }
        if actual_output.trim().is_empty() {
            return Err("Actual output cannot be empty".to_string());
        }

        Ok(Self {
            input,
            actual_output,
            expected_output,
        })
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn actual_output(&self) -> &str {
        &self.actual_output
    }

    pub fn expected_output(&self) -> Option<&str> {
        self.expected_output.as_deref()
    }
}