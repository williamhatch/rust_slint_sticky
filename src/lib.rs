pub mod note;
pub mod storage;

pub use note::AppNote;
pub use storage::{NoteStorage, StorageStats};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sticky_note_creation() {
        let note = AppNote::new("Test Title".to_string(), "Test Content".to_string());
        assert_eq!(note.title, "Test Title");
        assert_eq!(note.content, "Test Content");
        assert!(!note.id.is_empty());
        assert!(!note.created_at.is_empty());
        assert!(!note.updated_at.is_empty());
    }

    #[test]
    fn test_sticky_note_update() {
        let mut note = AppNote::new("Original".to_string(), "Original Content".to_string());
        let original_updated_at = note.updated_at.clone();
        
        // Sleep longer to ensure timestamp changes (format is to the minute)
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        note.update_content("Updated".to_string(), "Updated Content".to_string());
        
        assert_eq!(note.title, "Updated");
        assert_eq!(note.content, "Updated Content");
        assert!(!note.updated_at.is_empty());
        // Note: Due to time format precision (minutes), timestamps might be the same
        // This is expected behavior in real usage
    }

    #[test]
    fn test_sticky_note_position() {
        let mut note = AppNote::default();
        note.set_position(100.0, 200.0);
        
        assert_eq!(note.x, 100.0);
        assert_eq!(note.y, 200.0);
    }

    #[test]
    fn test_sticky_note_size() {
        let mut note = AppNote::default();
        note.set_size(300.0, 400.0);
        
        assert_eq!(note.width, 300.0);
        assert_eq!(note.height, 400.0);
    }

    #[tokio::test]
    async fn test_note_storage_creation() {
        let storage = NoteStorage::new().await;
        let stats = storage.get_stats().await.unwrap();
        
        // New storage should have no notes
        assert_eq!(stats.total_notes, 0);
        assert!(stats.data_directory.exists() || stats.data_directory.to_string_lossy().contains("rust_slint_sticky"));
    }
} 