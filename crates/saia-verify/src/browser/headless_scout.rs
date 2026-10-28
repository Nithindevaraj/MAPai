use serde::{Deserialize, Serialize};
use std::process::Command;
use regex::Regex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadlessBrowseResult {
    pub url: String,
    pub title: String,
    pub markdown: String,
    pub links: Vec<String>,
    pub requires_auth: bool,
    pub has_captcha: bool,
    pub status_code: u16,
    pub byte_size: usize,
}

pub struct HeadlessScoutEngine;

impl HeadlessScoutEngine {
    /// Fetches a web page silently in memory, converts HTML to clean Markdown,
    /// and detects if escalation to a real browser is required.
    pub fn browse(url: &str) -> anyhow::Result<HeadlessBrowseResult> {
        // Execute curl with silent flags, follow redirects, capture output and headers
        let output = Command::new("curl")
            .args([
                "-sL",
                "-w", "\n%{http_code}\n%{url_effective}",
                "-H", "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36",
                "-H", "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
                "--max-time", "15",
                url,
            ])
            .output()?;

        let raw_stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let lines: Vec<&str> = raw_stdout.lines().collect();

        if lines.len() < 2 {
            anyhow::bail!("Failed to retrieve content from {}", url);
        }

        let effective_url = lines.last().unwrap_or(&url).trim().to_string();
        let status_code = lines[lines.len() - 2].trim().parse::<u16>().unwrap_or(200);

        // Body content without the trailing metrics
        let body_lines = if lines.len() > 2 { &lines[0..lines.len() - 2] } else { &[] };
        let html_body = body_lines.join("\n");
        let byte_size = html_body.len();

        // 1. Detect if authentication or login redirection occurred
        let lower_url = effective_url.to_lowercase();
        let lower_body = html_body.to_lowercase();

        let requires_auth = status_code == 401 
            || status_code == 403
            || lower_url.contains("/login")
            || lower_url.contains("/signin")
            || lower_url.contains("accounts.google.com")
            || lower_url.contains("login.microsoftonline.com")
            || lower_url.contains("github.com/login");

        // 2. Detect if bot-check, Cloudflare Turnstile, or CAPTCHA is present
        let has_captcha = lower_body.contains("cf-turnstile")
            || lower_body.contains("g-recaptcha")
            || lower_body.contains("hcaptcha")
            || lower_body.contains("challenge-platform")
            || lower_body.contains("please enable javascript")
            || lower_body.contains("just a moment...");

        // 3. Extract title
        let title_re = Regex::new(r"(?i)<title[^>]*>(.*?)</title>").unwrap();
        let title = title_re.captures(&html_body)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or_else(|| "Web Page".to_string());

        // 4. Extract links
        let href_re = Regex::new(r#"href\s*=\s*["'](https?://[^"']+)["']"#).unwrap();
        let mut links = Vec::new();
        for cap in href_re.captures_iter(&html_body) {
            if let Some(m) = cap.get(1) {
                let link = m.as_str().to_string();
                if !links.contains(&link) {
                    links.push(link);
                }
            }
        }

        // 5. Semantic HTML to Markdown conversion (token-dense)
        let markdown = Self::html_to_markdown(&html_body);

        Ok(HeadlessBrowseResult {
            url: effective_url,
            title,
            markdown,
            links,
            requires_auth,
            has_captcha,
            status_code,
            byte_size,
        })
    }

    /// Converts raw HTML to clean, token-dense Markdown
    fn html_to_markdown(html: &str) -> String {
        // Strip script, style, SVG, and noscript elements
        let script_re = Regex::new(r"(?is)<script[^>]*>.*?</script>").unwrap();
        let style_re = Regex::new(r"(?is)<style[^>]*>.*?</style>").unwrap();
        let svg_re = Regex::new(r"(?is)<svg[^>]*>.*?</svg>").unwrap();
        let noscript_re = Regex::new(r"(?is)<noscript[^>]*>.*?</noscript>").unwrap();

        let s1 = script_re.replace_all(html, "");
        let s2 = style_re.replace_all(&s1, "");
        let s3 = svg_re.replace_all(&s2, "");
        let stripped = noscript_re.replace_all(&s3, "");

        // Convert common markdown elements
        let h1_re = Regex::new(r"(?i)<h1[^>]*>(.*?)</h1>").unwrap();
        let h2_re = Regex::new(r"(?i)<h2[^>]*>(.*?)</h2>").unwrap();
        let h3_re = Regex::new(r"(?i)<h3[^>]*>(.*?)</h3>").unwrap();
        let p_re = Regex::new(r"(?i)<p[^>]*>(.*?)</p>").unwrap();
        let code_re = Regex::new(r"(?is)<pre[^>]*><code[^>]*>(.*?)</code></pre>").unwrap();
        let li_re = Regex::new(r"(?i)<li[^>]*>(.*?)</li>").unwrap();
        let tag_re = Regex::new(r"<[^>]+>").unwrap();

        let step1 = h1_re.replace_all(&stripped, "\n# $1\n");
        let step2 = h2_re.replace_all(&step1, "\n## $1\n");
        let step3 = h3_re.replace_all(&step2, "\n### $1\n");
        let step4 = code_re.replace_all(&step3, "\n```\n$1\n```\n");
        let step5 = li_re.replace_all(&step4, "\n- $1");
        let step6 = p_re.replace_all(&step5, "\n$1\n");

        // Strip remaining HTML tags
        let clean = tag_re.replace_all(&step6, "");

        // Decode common HTML entities
        let decoded = clean
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&#39;", "'")
            .replace("&nbsp;", " ");

        // Clean up excessive blank lines
        let mut result = Vec::new();
        let mut last_was_empty = false;
        for line in decoded.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                if !last_was_empty {
                    result.push("");
                    last_was_empty = true;
                }
            } else {
                result.push(trimmed);
                last_was_empty = false;
            }
        }

        result.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_to_markdown_distillation() {
        let sample_html = r#"
            <html>
                <head><title>Test Page</title></head>
                <body>
                    <h1>Rust Verification</h1>
                    <p>Independent verification and validation.</p>
                    <pre><code>fn main() {}</code></pre>
                    <ul>
                        <li>Zero hollow stubs</li>
                        <li>Full truth</li>
                    </ul>
                </body>
            </html>
        "#;

        let md = HeadlessScoutEngine::html_to_markdown(sample_html);
        assert!(md.contains("# Rust Verification"));
        assert!(md.contains("fn main() {}"));
        assert!(md.contains("- Zero hollow stubs"));
    }
}
