use crate::scanner::{FileInfo, Scanner};
use anyhow::Result;
use std::path::PathBuf;
use std::fs;

#[derive(PartialEq)]
pub enum SortOrder {
    Size,
    Name,
    Modified,
}

#[derive(PartialEq)]
pub enum PopupState {
    Hidden,
    DeleteConfirmation,
}

pub struct App {
    pub scanner: Scanner,
    pub current_path: PathBuf,
    pub entries: Option<FileInfo>,
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub sort_order: SortOrder,
    pub show_hidden: bool,
    pub history: Vec<(PathBuf, FileInfo)>,
    pub popup_state: PopupState,
    pub selected_for_deletion: Option<usize>,
}

impl App {
    pub fn new(scanner: Scanner) -> Self {
        let current_path = scanner.root_path().to_path_buf();
        Self {
            scanner,
            current_path,
            entries: None,
            selected_index: 0,
            scroll_offset: 0,
            sort_order: SortOrder::Size,
            show_hidden: false,
            history: Vec::new(),
            popup_state: PopupState::Hidden,
            selected_for_deletion: None,
        }
    }

    pub fn scan(&mut self) -> Result<()> {
        self.entries = Some(self.scanner.scan()?);
        self.sort_entries();
        Ok(())
    }

    pub fn sort_entries(&mut self) {
        if let Some(entries) = &mut self.entries {
            Self::sort_fileinfo(entries, &self.sort_order);
        }
    }

    fn sort_fileinfo(file_info: &mut FileInfo, sort_order: &SortOrder) {
        file_info.children.sort_by(|a, b| {
            match sort_order {
                SortOrder::Size => b.size.cmp(&a.size),
                SortOrder::Name => a.path.file_name().cmp(&b.path.file_name()),
                SortOrder::Modified => {
                    let a_modified = a.path.metadata().and_then(|m| m.modified()).ok();
                    let b_modified = b.path.metadata().and_then(|m| m.modified()).ok();
                    b_modified.cmp(&a_modified)
                }
            }
        });

        for child in &mut file_info.children {
            Self::sort_fileinfo(child, sort_order);
        }
    }

    pub fn toggle_sort_order(&mut self) {
        self.sort_order = match self.sort_order {
            SortOrder::Size => SortOrder::Name,
            SortOrder::Name => SortOrder::Modified,
            SortOrder::Modified => SortOrder::Size,
        };
        self.sort_entries();
    }

    pub fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
        self.scan().unwrap_or_default();
    }

    pub fn enter_directory(&mut self) -> Result<()> {
        let selected_path = if let Some(entries) = &self.entries {
            if self.selected_index < entries.children.len() {
                let selected = &entries.children[self.selected_index];
                if selected.is_dir {
                    Some(selected.path.clone())
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        if let Some(new_path) = selected_path {
            if let Some(current_entries) = self.entries.take() {
                self.history.push((self.current_path.clone(), current_entries));
            }

            self.current_path = new_path;
            self.scanner = Scanner::new(
                self.current_path.to_str().unwrap_or("."),
                None,
            );
            self.selected_index = 0;
            self.scroll_offset = 0;
            self.scan()?;
        }

        Ok(())
    }

    pub fn go_back(&mut self) -> Result<()> {
        if let Some((parent_path, parent_entries)) = self.history.pop() {
            self.current_path = parent_path;
            self.entries = Some(parent_entries);
            self.selected_index = 0;
            self.scroll_offset = 0;
        }
        Ok(())
    }

    pub fn update_scroll(&mut self, height: usize) {
        if let Some(entries) = &self.entries {
            let total_entries = entries.children.len();
            
            if self.selected_index >= self.scroll_offset + height {
                self.scroll_offset = self.selected_index - height + 1;
            } else if self.selected_index < self.scroll_offset {
                self.scroll_offset = self.selected_index;
            }

            self.scroll_offset = self.scroll_offset.min(total_entries.saturating_sub(height));
        }
    }

    pub fn show_delete_confirmation(&mut self) {
        self.popup_state = PopupState::DeleteConfirmation;
        self.selected_for_deletion = Some(self.selected_index);
    }

    pub fn hide_popup(&mut self) {
        self.popup_state = PopupState::Hidden;
        self.selected_for_deletion = None;
    }

    pub fn confirm_delete(&mut self) -> Result<()> {
        if let Some(index) = self.selected_for_deletion {
            if let Some(entries) = &self.entries {
                if index < entries.children.len() {
                    let selected = &entries.children[index];
                    let path = &selected.path;

                    if selected.is_dir {
                        fs::remove_dir_all(path)?;
                    } else {
                        fs::remove_file(path)?;
                    }
                    self.scan()?;
                }
            }
        }
        self.hide_popup();
        Ok(())
    }
} 