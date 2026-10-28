use anyhow::{bail, Context, Result};
use std::fs;
use std::path::PathBuf;

/// Represents an autonomously generated agent skill compatible with agentskills.io.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynthesizedSkill {
    pub name: String,
    pub description: String,
    pub triggers: Vec<String>,
    pub instructions: String,
    pub verification_steps: Vec<String>,
}

/// Linter and manager for autonomous skill creation and continuous learning.
pub struct SkillSynthesizer {
    skills_root: PathBuf,
}

impl SkillSynthesizer {
    pub fn new(skills_root: impl Into<PathBuf>) -> Self {
        Self {
            skills_root: skills_root.into(),
        }
    }

    /// Validates that a skill satisfies all format, clarity, and structural constraints.
    pub fn lint_skill(skill: &SynthesizedSkill) -> Result<()> {
        if skill.name.trim().is_empty() {
            bail!("Skill name cannot be empty");
        }
        if skill.description.trim().is_empty() {
            bail!("Skill description cannot be empty");
        }
        if skill.instructions.trim().len() < 50 {
            bail!("Skill instructions must provide actionable depth (minimum 50 characters)");
        }
        if skill.triggers.is_empty() {
            bail!("Skill must declare at least one activation trigger");
        }
        Ok(())
    }

    /// Renders the skill into standard YAML frontmatter + markdown specification.
    pub fn render_markdown(skill: &SynthesizedSkill) -> String {
        let mut out = String::new();
        out.push_str("---\n");
        out.push_str(&format!("name: \"{}\"\n", skill.name));
        out.push_str(&format!("description: \"{}\"\n", skill.description));
        out.push_str("triggers:\n");
        for trig in &skill.triggers {
            out.push_str(&format!("  - \"{}\"\n", trig));
        }
        out.push_str("---\n\n");
        out.push_str(&format!("# Skill: {}\n\n", skill.name));
        out.push_str(&format!("## Overview\n{}\n\n", skill.description));
        out.push_str(&format!("## Instructions\n{}\n\n", skill.instructions));

        if !skill.verification_steps.is_empty() {
            out.push_str("## Verification Gates\n");
            for step in &skill.verification_steps {
                out.push_str(&format!("- [ ] {}\n", step));
            }
            out.push('\n');
        }

        out
    }

    /// Lints, renders, and saves the synthesized skill directly to the persistent directory.
    pub fn persist_skill(&self, skill: &SynthesizedSkill) -> Result<PathBuf> {
        Self::lint_skill(skill)?;
        let skill_dir = self.skills_root.join(&skill.name);
        fs::create_dir_all(&skill_dir).context("Failed to create skill directory")?;

        let skill_file = skill_dir.join("SKILL.md");
        let content = Self::render_markdown(skill);
        fs::write(&skill_file, content).context("Failed to write SKILL.md file")?;

        Ok(skill_file)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_linter_and_rendering() {
        let temp_dir = std::env::temp_dir().join(format!("mapai_skills_{}", chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)));
        let synthesizer = SkillSynthesizer::new(&temp_dir);

        let skill = SynthesizedSkill {
            name: "conpty-windows-debug".into(),
            description: "Diagnoses and mounts native Windows ConPTY handles for interactive CLI sessions".into(),
            triggers: vec!["conpty".into(), "windows terminal hang".into(), "interactive pty".into()],
            instructions: "1. Load node-pty precompiled ConPTY 1.25 DLLs.\n2. Bind pseudo-terminal to PowerShell child process.\n3. Multiplex stdin/stdout via async ring-buffer.".into(),
            verification_steps: vec!["Verify exit code 0 on EOF".into(), "Ensure 0 zombie daemons".into()],
        };

        // 1. Lint check
        assert!(SkillSynthesizer::lint_skill(&skill).is_ok());

        // 2. Render check
        let md = SkillSynthesizer::render_markdown(&skill);
        assert!(md.contains("name: \"conpty-windows-debug\""));
        assert!(md.contains("## Instructions"));

        // 3. Persist check
        let path = synthesizer.persist_skill(&skill).unwrap();
        assert!(path.exists());

        // Clean up
        let _ = fs::remove_dir_all(temp_dir);
    }
}
