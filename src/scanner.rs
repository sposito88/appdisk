use anyhow::Result;
use rayon::prelude::*;
use serde::Serialize;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug, Serialize, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub size: u64,
    pub is_dir: bool,
    pub children: Vec<FileInfo>,
}

pub struct Scanner {
    root: PathBuf,
    max_depth: Option<usize>,
}

impl Scanner {
    pub fn new(path: &str, max_depth: Option<usize>) -> Self {
        Scanner {
            root: PathBuf::from(path),
            max_depth,
        }
    }

    pub fn scan(&self) -> Result<FileInfo> {
        let walker = WalkDir::new(&self.root)
            .min_depth(0)
            .max_depth(self.max_depth.unwrap_or(std::usize::MAX));

        let mut root_info = FileInfo {
            path: self.root.clone(),
            size: 0,
            is_dir: true,
            children: Vec::new(),
        };

        // Usa paralelismo para processar os arquivos
        let entries: Vec<_> = walker
            .into_iter()
            .filter_map(Result::ok)
            .collect::<Vec<_>>()
            .par_iter()
            .map(|entry| {
                let path = entry.path().to_path_buf();
                let metadata = entry.metadata().ok()?;
                let size = if metadata.is_file() {
                    metadata.len()
                } else {
                    0
                };

                Some(FileInfo {
                    path,
                    size,
                    is_dir: metadata.is_dir(),
                    children: Vec::new(),
                })
            })
            .collect();

        // Organiza a estrutura de árvore
        self.build_tree(&mut root_info, &entries);

        Ok(root_info)
    }

    fn build_tree(&self, parent: &mut FileInfo, entries: &[Option<FileInfo>]) {
        let parent_path = &parent.path;
        
        // Filtra entradas que são filhos diretos do pai atual
        let children: Vec<FileInfo> = entries
            .iter()
            .filter_map(|entry| entry.as_ref())
            .filter(|entry| {
                entry.path.parent().map_or(false, |p| p == parent_path)
            })
            .cloned()
            .collect();

        // Atualiza as informações do pai
        parent.children = children;
        parent.size = parent.children.iter().map(|child| child.size).sum();

        // Recursivamente constrói a árvore para cada filho
        for child in &mut parent.children {
            if child.is_dir {
                self.build_tree(child, entries);
            }
        }
    }

    pub fn root_path(&self) -> &PathBuf {
        &self.root
    }
} 