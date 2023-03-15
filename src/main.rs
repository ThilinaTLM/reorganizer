#![allow(dead_code)]

extern crate core;

use std::path::{Path, PathBuf};

use clap::Parser;

mod rules;
mod org;

enum OrganizeStrategy {
    ByExt,
    ByDate,
}

struct OrganizeEntry {
    src: PathBuf,
    strategy: OrganizeStrategy,
}

impl OrganizeEntry {

    fn new(src: PathBuf) -> Self {
        Self {
            src,
            strategy: OrganizeStrategy::ByExt,
        }
    }

    fn get_src(&self) -> PathBuf {
        self.src.to_path_buf()
    }

    fn get_dst(&self) -> PathBuf {
        let path = self.src.parent().unwrap();
        let file_name = self.src.file_name().unwrap();
        let category = match self.strategy {
            OrganizeStrategy::ByExt => self.get_category_from_ext(),
            OrganizeStrategy::ByDate => None,
        };

        let dst = match category {
            Some(category) => path.join(category).join(file_name),
            None => path.join(file_name),
        };
        dst
    }

    fn get_category_from_ext(&self) -> Option<String> {
        let ext = match Path::new(&self.src).extension() {
            Some(ext) => ext.to_str().unwrap(),
            None => "",
        };
        match ext {
            "pdf" => Some("Documents".to_string()),
            "doc" | "docx" => Some("Documents".to_string()),
            "xls" | "xlsx" => Some("Documents".to_string()),
            "ppt" | "pptx" => Some("Documents".to_string()),
            "txt" => Some("Documents".to_string()),
            "jpg" | "jpeg" | "png" | "gif" => Some("Images".to_string()),
            "mp4" | "mkv" | "avi" => Some("Videos".to_string()),
            "mp3" | "wav" => Some("Music".to_string()),
            _ => None,
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = Args::get_default_shell_cmd())]
    shell_cmd: String,

    #[arg(short, long, default_value_t = Args::get_default_cwd())]
    cwd: String,

    #[arg(long, default_value_t = true)]
    create_dirs: bool,
}

impl Args {
    fn get_default_cwd() -> String {
        match std::env::current_dir() {
            Ok(path) => path.to_str().unwrap().to_string(),
            Err(_) => String::from(""),
        }
    }

    fn get_default_shell_cmd() -> String {
        String::from("mv {} {}")
    }
}


fn main() {
    // let args = Args::parse();
}
