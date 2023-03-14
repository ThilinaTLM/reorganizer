#![allow(dead_code)]

mod utils;
mod rules;

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use clap::Parser;

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
    #[arg(short, long)]
    shell: Option<String>,

    #[arg(short, long, default_value_t = utils::get_cwd_string())]
    cwd: String,

    #[arg(long, default_value_t = false)]
    create_dirs: bool,
}


fn main() {
    let args = Args::parse();

    let cwd = Path::new(&args.cwd);
    let entries = cwd.read_dir().unwrap()
        .filter(|entry| entry.as_ref().unwrap().file_type().unwrap().is_file())
        .map(|entry| OrganizeEntry::new(entry.unwrap().path().to_path_buf()));

    if args.create_dirs {
        let mut dirs = HashSet::new();
        for entry in entries {
            let category = entry.get_category_from_ext();
            if let Some(category) = category {
                let dir = cwd.join(category);
                dirs.insert(dir);
            }
        }
        for dir in dirs {
            println!("Category Directory: {}", dir.to_str().unwrap());
            // std::fs::create_dir_all(dir).unwrap();
        }
    }

    // if args.shell.is_some() {
    //     let shell = args.shell.unwrap();
    // }
}
