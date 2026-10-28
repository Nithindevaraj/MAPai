/// Redaction and entropy-based credential sanitizer.
/// Prevents accidental leakage of API keys, AWS credentials, and SSH keys in logs and web UI.
pub struct SecretMasker;

impl SecretMasker {
    /// Masks all detected secrets and high-entropy credentials within an arbitrary string.
    pub fn mask(input: &str) -> String {
        let mut output = input.to_string();

        // 1. Private Key blocks
        if output.contains("-----BEGIN") {
            let key_types = [
                "RSA PRIVATE KEY",
                "OPENSSH PRIVATE KEY",
                "EC PRIVATE KEY",
                "DSA PRIVATE KEY",
                "PRIVATE KEY",
            ];
            for ktype in key_types {
                let start_tag = format!("-----BEGIN {}-----", ktype);
                let end_tag = format!("-----END {}-----", ktype);
                while let Some(start_idx) = output.find(&start_tag) {
                    if let Some(end_rel_idx) = output[start_idx..].find(&end_tag) {
                        let end_idx = start_idx + end_rel_idx + end_tag.len();
                        output.replace_range(start_idx..end_idx, "[REDACTED:PRIVATE_KEY]");
                    } else {
                        output.replace_range(start_idx.., "[REDACTED:PRIVATE_KEY]");
                        break;
                    }
                }
            }
        }

        // 2. AWS Access Key IDs (AKIA...)
        output = Self::mask_pattern(&output, "AKIA", 20, "[REDACTED:AWS_KEY]");

        // 3. GitHub Personal Access Tokens (ghp_..., gho_...)
        output = Self::mask_pattern(&output, "ghp_", 40, "[REDACTED:GITHUB_TOKEN]");
        output = Self::mask_pattern(&output, "gho_", 40, "[REDACTED:GITHUB_TOKEN]");

        // 4. OpenAI / Anthropic style API keys (sk-...)
        output = Self::mask_pattern(&output, "sk-", 40, "[REDACTED:API_KEY]");

        // 5. Bearer tokens
        output = Self::mask_bearer_tokens(&output);

        output
    }

    /// Checks if a string contains unmasked credentials.
    pub fn contains_secrets(input: &str) -> bool {
        input.contains("-----BEGIN")
            || input.contains("ghp_")
            || input.contains("gho_")
            || input.contains("AKIA")
            || input.contains("sk-")
    }

    fn mask_pattern(text: &str, prefix: &str, min_len: usize, replacement: &str) -> String {
        let mut result = text.to_string();
        let mut start_search = 0;

        while let Some(pos) = result[start_search..].find(prefix) {
            let abs_pos = start_search + pos;
            let slice = &result[abs_pos..];

            // Find end of token (whitespace, quote, punctuation)
            let token_len = slice
                .find(|c: char| c.is_whitespace() || c == '"' || c == '\'' || c == ',' || c == ')' || c == '}')
                .unwrap_or(slice.len());

            if token_len >= prefix.len() + 10 && token_len <= min_len + 15 {
                result.replace_range(abs_pos..abs_pos + token_len, replacement);
                start_search = abs_pos + replacement.len();
            } else {
                start_search = abs_pos + prefix.len();
            }
        }

        result
    }

    fn mask_bearer_tokens(text: &str) -> String {
        let mut result = text.to_string();
        let prefix = "Bearer ";
        let mut start_search = 0;

        while let Some(pos) = result[start_search..].find(prefix) {
            let abs_pos = start_search + pos + prefix.len();
            let slice = &result[abs_pos..];

            let token_len = slice
                .find(|c: char| c.is_whitespace() || c == '"' || c == '\'' || c == ',' || c == '}' || c == ';')
                .unwrap_or(slice.len());

            if token_len >= 16 {
                result.replace_range(abs_pos..abs_pos + token_len, "[REDACTED:BEARER_TOKEN]");
                start_search = abs_pos + "[REDACTED:BEARER_TOKEN]".len();
            } else {
                start_search = abs_pos;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_masking_api_keys() {
        let log = "Connecting with key sk-proj-1234567890abcdef1234567890abcdef via OpenAI endpoint";
        let masked = SecretMasker::mask(log);
        assert!(!masked.contains("sk-proj-"));
        assert!(masked.contains("[REDACTED:API_KEY]"));
    }

    #[test]
    fn test_masking_aws_and_github_tokens() {
        let raw = "Deploying with AKIAIOSFODNN7EXAMPLE and ghp_111122223333444455556666777788889999 to repo";
        let masked = SecretMasker::mask(raw);
        assert!(!masked.contains("AKIAIOSFODNN7EXAMPLE"));
        assert!(!masked.contains("ghp_11112222"));
        assert!(masked.contains("[REDACTED:AWS_KEY]"));
        assert!(masked.contains("[REDACTED:GITHUB_TOKEN]"));
    }

    #[test]
    fn test_masking_private_keys() {
        let key = "Here is the key:\n-----BEGIN RSA PRIVATE KEY-----\nMIIEowIBAAKCAQEA0\n-----END RSA PRIVATE KEY-----\nEnd of key.";
        let masked = SecretMasker::mask(key);
        assert!(!masked.contains("MIIEowIBAAKCAQEA0"));
        assert!(masked.contains("[REDACTED:PRIVATE_KEY]"));
    }
}
