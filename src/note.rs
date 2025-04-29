use anyhow::Result;
use std::path::PathBuf;
use std::{collections::HashSet, str::FromStr};

use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

pub struct Note<'n> {
    pub frontmatter: FrontMatter<'n>,
    pub content: &'n str,
    pub path: PathBuf,
}

impl<'n> Note<'_> {
    fn generate_path_from_title(title: &str) -> String {
        let date = Local::now().date_naive();
        let clean_title = title.replace(" ", "-");
        format!("{}-{}.md", date, clean_title)
    }

    pub fn new<'a>(title: &'n str, content: &'n str) -> Note<'n> {
        // FIXME: allow to create file in subdirectory (create subdirectory) if they don't exists
        let path = Note::generate_path_from_title(title);
        Note {
            frontmatter: FrontMatter::new(title),
            content,
            path: PathBuf::from_str(&path).expect("Failed to parse path"),
        }
    }

    pub fn to_string(self) -> Result<String> {
        Ok(format!(
            "{}\n{}",
            self.frontmatter.to_toml_str()?,
            self.content
        ))
    }
}

#[derive(Serialize, Deserialize)]
pub struct FrontMatter<'fm> {
    pub title: &'fm str,
    pub created: NaiveDateTime,
    pub tags: HashSet<&'fm str>,
    pub links: HashSet<&'fm str>,
}

impl<'fm> FrontMatter<'_> {
    pub fn new<'a>(title: &'fm str) -> FrontMatter<'fm> {
        FrontMatter {
            title,
            created: Local::now().naive_local(),
            tags: HashSet::new(),
            links: HashSet::new(),
        }
    }

    pub fn to_toml_str(self) -> Result<String> {
        Ok(format!("+++\n{}+++", toml::to_string(&self)?))
    }
}
