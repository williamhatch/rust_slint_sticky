use crate::note::AppNote;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Storage manager for sticky notes
#[derive(Debug, Clone)]
pub struct NoteStorage {
    data_dir: PathBuf,
    notes_file: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
struct StorageData {
    notes: HashMap<String, AppNote>,
}

impl Default for StorageData {
    fn default() -> Self {
        Self {
            notes: HashMap::new(),
        }
    }
}

impl NoteStorage {
    /// Create a new note storage instance
    pub async fn new() -> Self {
        let data_dir = Self::get_data_directory();
        let notes_file = data_dir.join("notes.json");
        
        // Create data directory if it doesn't exist
        if let Err(e) = fs::create_dir_all(&data_dir).await {
            eprintln!("Warning: Failed to create data directory: {}", e);
        }
        
        Self {
            data_dir,
            notes_file,
        }
    }
    
    /// Get the appropriate data directory for the current platform
    fn get_data_directory() -> PathBuf {
        if let Some(data_dir) = dirs::data_dir() {
            data_dir.join("rust_slint_sticky")
        } else {
            // Fallback to current directory
            std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join("data")
        }
    }
    
    /// Load all notes from storage
    pub async fn load_notes(&self) -> Result<Vec<crate::AppNote>, Box<dyn std::error::Error + Send + Sync>> {
        if !self.notes_file.exists() {
            return Ok(Vec::new());
        }
        
        let mut file = fs::File::open(&self.notes_file).await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;
        
        if contents.trim().is_empty() {
            return Ok(Vec::new());
        }
        
        let storage_data: StorageData = serde_json::from_str(&contents)?;
        
        let notes: Vec<crate::AppNote> = storage_data
            .notes
            .into_values()
            .collect();
        
        Ok(notes)
    }
    
    /// Save a note to storage
    pub async fn save_note(&self, note: &AppNote) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut storage_data = self.load_storage_data().await?;
        storage_data.notes.insert(note.id.clone(), note.clone());
        self.save_storage_data(&storage_data).await?;
        Ok(())
    }
    
    /// Delete a note from storage
    pub async fn delete_note(&self, note_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut storage_data = self.load_storage_data().await?;
        storage_data.notes.remove(note_id);
        self.save_storage_data(&storage_data).await?;
        Ok(())
    }
    
    /// Update an existing note in storage
    pub async fn update_note(&self, note: &AppNote) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Same as save_note since we use HashMap
        self.save_note(note).await
    }
    
    /// Get a specific note by ID
    pub async fn get_note(&self, note_id: &str) -> Result<Option<AppNote>, Box<dyn std::error::Error + Send + Sync>> {
        let storage_data = self.load_storage_data().await?;
        Ok(storage_data.notes.get(note_id).cloned())
    }
    
    /// Clear all notes from storage
    pub async fn clear_all_notes(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let storage_data = StorageData::default();
        self.save_storage_data(&storage_data).await?;
        Ok(())
    }
    
    /// Export notes to a backup file
    pub async fn export_notes(&self, backup_path: &PathBuf) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let storage_data = self.load_storage_data().await?;
        let json_data = serde_json::to_string_pretty(&storage_data)?;
        
        let mut file = fs::File::create(backup_path).await?;
        file.write_all(json_data.as_bytes()).await?;
        file.sync_all().await?;
        
        Ok(())
    }
    
    /// Import notes from a backup file
    pub async fn import_notes(&self, backup_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        let mut file = fs::File::open(backup_path).await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;
        
        let imported_data: StorageData = serde_json::from_str(&contents)?;
        let count = imported_data.notes.len();
        
        // Merge with existing notes
        let mut storage_data = self.load_storage_data().await?;
        for (id, note) in imported_data.notes {
            storage_data.notes.insert(id, note);
        }
        
        self.save_storage_data(&storage_data).await?;
        Ok(count)
    }
    
    /// Get storage statistics
    pub async fn get_stats(&self) -> Result<StorageStats, Box<dyn std::error::Error + Send + Sync>> {
        let storage_data = self.load_storage_data().await?;
        let file_size = if self.notes_file.exists() {
            fs::metadata(&self.notes_file).await?.len()
        } else {
            0
        };
        
        Ok(StorageStats {
            total_notes: storage_data.notes.len(),
            file_size_bytes: file_size,
            data_directory: self.data_dir.clone(),
        })
    }
    
    // Private helper methods
    
    async fn load_storage_data(&self) -> Result<StorageData, Box<dyn std::error::Error + Send + Sync>> {
        if !self.notes_file.exists() {
            return Ok(StorageData::default());
        }
        
        let mut file = fs::File::open(&self.notes_file).await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;
        
        if contents.trim().is_empty() {
            return Ok(StorageData::default());
        }
        
        let storage_data: StorageData = serde_json::from_str(&contents)?;
        Ok(storage_data)
    }
    
    async fn save_storage_data(&self, data: &StorageData) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let json_data = serde_json::to_string_pretty(data)?;
        
        // Write to a temporary file first, then rename for atomic operation
        let temp_file = self.notes_file.with_extension("tmp");
        
        let mut file = fs::File::create(&temp_file).await?;
        file.write_all(json_data.as_bytes()).await?;
        file.sync_all().await?;
        drop(file);
        
        // Atomic rename
        fs::rename(&temp_file, &self.notes_file).await?;
        
        Ok(())
    }
}

/// Storage statistics
#[derive(Debug)]
pub struct StorageStats {
    pub total_notes: usize,
    pub file_size_bytes: u64,
    pub data_directory: PathBuf,
}

impl StorageStats {
    pub fn file_size_human_readable(&self) -> String {
        let bytes = self.file_size_bytes as f64;
        if bytes < 1024.0 {
            format!("{} B", bytes)
        } else if bytes < 1024.0 * 1024.0 {
            format!("{:.1} KB", bytes / 1024.0)
        } else if bytes < 1024.0 * 1024.0 * 1024.0 {
            format!("{:.1} MB", bytes / (1024.0 * 1024.0))
        } else {
            format!("{:.1} GB", bytes / (1024.0 * 1024.0 * 1024.0))
        }
    }
} 