use crate::error::{MkunitError, Result};
use dialoguer::{Confirm, Input, Select};
use std::io::IsTerminal;

/// Check if we're running in an interactive terminal
#[must_use]
pub fn is_interactive() -> bool {
    std::io::stdin().is_terminal() && std::io::stdout().is_terminal()
}

/// Prompt for a required string value
pub fn prompt_required(prompt: &str, no_interactive: bool) -> Result<String> {
    if no_interactive {
        return Err(MkunitError::InteractiveModeDisabled);
    }
    if !is_interactive() {
        return Err(MkunitError::invalid_argument(format!(
            "Required value '{prompt}' not provided and not running interactively"
        )));
    }

    Input::new()
        .with_prompt(prompt)
        .interact_text()
        .map_err(|e| MkunitError::Other(format!("Failed to read input: {e}")))
}

/// Prompt for an optional string value with a default
#[allow(dead_code)]
pub fn prompt_optional(prompt: &str, default: &str, no_interactive: bool) -> Result<String> {
    if no_interactive {
        return Ok(default.to_string());
    }
    if !is_interactive() {
        return Ok(default.to_string());
    }

    Input::new()
        .with_prompt(prompt)
        .default(default.to_string())
        .interact_text()
        .map_err(|e| MkunitError::Other(format!("Failed to read input: {e}")))
}

/// Prompt for an optional string value (empty allowed)
pub fn prompt_optional_empty(prompt: &str, no_interactive: bool) -> Result<Option<String>> {
    if no_interactive || !is_interactive() {
        return Ok(None);
    }

    let value: String = Input::new()
        .with_prompt(prompt)
        .allow_empty(true)
        .interact_text()
        .map_err(|e| MkunitError::Other(format!("Failed to read input: {e}")))?;

    if value.is_empty() {
        Ok(None)
    } else {
        Ok(Some(value))
    }
}

/// Prompt for selection from a list of options
#[allow(dead_code)]
pub fn prompt_select(
    prompt: &str,
    options: &[&str],
    default: usize,
    no_interactive: bool,
) -> Result<usize> {
    if no_interactive {
        return Ok(default);
    }
    if !is_interactive() {
        return Ok(default);
    }

    Select::new()
        .with_prompt(prompt)
        .items(options)
        .default(default)
        .interact()
        .map_err(|e| MkunitError::Other(format!("Failed to read selection: {e}")))
}

/// Prompt for confirmation
pub fn prompt_confirm(prompt: &str, default: bool, no_interactive: bool) -> Result<bool> {
    if no_interactive {
        return Ok(default);
    }
    if !is_interactive() {
        return Ok(default);
    }

    Confirm::new()
        .with_prompt(prompt)
        .default(default)
        .interact()
        .map_err(|e| MkunitError::Other(format!("Failed to read confirmation: {e}")))
}

/// Prompt for yes/no with option to abort
pub fn prompt_confirm_or_abort(prompt: &str, default: bool, no_interactive: bool) -> Result<bool> {
    if no_interactive {
        return Ok(default);
    }
    if !is_interactive() {
        return Err(MkunitError::invalid_argument(
            "Confirmation required but not running interactively",
        ));
    }

    Confirm::new()
        .with_prompt(prompt)
        .default(default)
        .interact()
        .map_err(|e| MkunitError::Other(format!("Failed to read confirmation: {e}")))
}

/// Prompt for an integer value
#[allow(dead_code)]
pub fn prompt_number(prompt: &str, default: u32, no_interactive: bool) -> Result<u32> {
    if no_interactive {
        return Ok(default);
    }
    if !is_interactive() {
        return Ok(default);
    }

    Input::new()
        .with_prompt(prompt)
        .default(default)
        .interact_text()
        .map_err(|e| MkunitError::Other(format!("Failed to read input: {e}")))
}

/// Convenience struct for building interactive prompts
pub struct PromptBuilder {
    no_interactive: bool,
}

#[allow(dead_code)]
impl PromptBuilder {
    #[must_use]
    pub fn new(no_interactive: bool) -> Self {
        Self { no_interactive }
    }

    pub fn required(&self, prompt: &str) -> Result<String> {
        prompt_required(prompt, self.no_interactive)
    }

    pub fn optional(&self, prompt: &str, default: &str) -> Result<String> {
        prompt_optional(prompt, default, self.no_interactive)
    }

    pub fn optional_empty(&self, prompt: &str) -> Result<Option<String>> {
        prompt_optional_empty(prompt, self.no_interactive)
    }

    pub fn select(&self, prompt: &str, options: &[&str], default: usize) -> Result<usize> {
        prompt_select(prompt, options, default, self.no_interactive)
    }

    pub fn confirm(&self, prompt: &str, default: bool) -> Result<bool> {
        prompt_confirm(prompt, default, self.no_interactive)
    }

    pub fn confirm_or_abort(&self, prompt: &str, default: bool) -> Result<bool> {
        prompt_confirm_or_abort(prompt, default, self.no_interactive)
    }

    pub fn number(&self, prompt: &str, default: u32) -> Result<u32> {
        prompt_number(prompt, default, self.no_interactive)
    }
}
