use crate::error::PuppyCraftError;
use crate::models::SkillSrc;
use std::fs;
use std::path::{Path, PathBuf};

pub static SKILLS: &[SkillSrc] = &[
    SkillSrc {
        name: "puppycraft-advisor",
        description: "Dog acquisition advisor, breed matcher, training, legal, vaccination, and travel expert for PuppyCraft.",
        skill_md: include_str!("../../../skills/puppycraft-advisor/SKILL.md"),
    },
    SkillSrc {
        name: "puppycraft-workflow",
        description: "How to use PuppyCraft tools for breed matching, diary, sitter calendar, vaccines, and status.",
        skill_md: include_str!("../../../skills/puppycraft-workflow/SKILL.md"),
    },
];

pub fn skills_list() -> &'static [SkillSrc] {
    SKILLS
}

pub fn skill_read(name: &str) -> Result<&'static SkillSrc, PuppyCraftError> {
    SKILLS
        .iter()
        .find(|s| s.name == name)
        .ok_or_else(|| PuppyCraftError::NotFound(format!("skill not found: {name}")))
}

pub fn skill_by_uri(uri: &str) -> Result<&'static SkillSrc, PuppyCraftError> {
    let trimmed = uri.trim();
    let rest = trimmed
        .strip_prefix("skill://")
        .ok_or_else(|| PuppyCraftError::NotFound(format!("unknown resource uri: {uri}")))?;
    let name = rest
        .strip_suffix("/SKILL.md")
        .ok_or_else(|| PuppyCraftError::NotFound(format!("unknown resource uri: {uri}")))?;
    skill_read(name)
}

pub fn export_skills(dest_dir: &Path, force: bool) -> Result<Vec<PathBuf>, PuppyCraftError> {
    let mut written = Vec::new();
    for s in SKILLS {
        let dir = dest_dir.join(s.name);
        fs::create_dir_all(&dir).map_err(|e| PuppyCraftError::Store(e.to_string()))?;
        let target = dir.join("SKILL.md");
        if target.exists() && !force {
            return Err(PuppyCraftError::Invalid(format!(
                "refusing to overwrite existing file without --force: {}",
                target.display()
            )));
        }
        fs::write(&target, s.skill_md).map_err(|e| PuppyCraftError::Store(e.to_string()))?;
        written.push(target);
    }
    Ok(written)
}
