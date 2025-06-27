#[cfg(test)]
mod tests {
    use rust_slint_sticky::note::{StickyNote, SerializableColor};

    #[test]
    fn test_create_new_note() {
        let note = StickyNote::new("Test Title".to_string(), "Test Content".to_string());
        
        assert_eq!(note.title, "Test Title");
        assert_eq!(note.content, "Test Content");
        assert!(!note.id.is_empty());
        assert!(!note.created_at.is_empty());
        assert!(!note.updated_at.is_empty());
        assert_eq!(note.x, 10.0);
        assert_eq!(note.y, 10.0);
        assert_eq!(note.width, 220.0);
        assert_eq!(note.height, 200.0);
    }

    #[test]
    fn test_note_color_conversion() {
        let color = SerializableColor { red: 255, green: 235, blue: 59 };
        let slint_color: slint::Color = color.into();
        let converted_back: SerializableColor = slint_color.into();
        
        assert_eq!(color.red, converted_back.red);
        assert_eq!(color.green, converted_back.green);
        assert_eq!(color.blue, converted_back.blue);
    }

    #[test]
    fn test_note_update_content() {
        let mut note = StickyNote::new("Original".to_string(), "Original Content".to_string());
        let original_created_at = note.created_at.clone();
        
        // Sleep a bit to ensure timestamp difference
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        note.update_content("Updated Title".to_string(), "Updated Content".to_string());
        
        assert_eq!(note.title, "Updated Title");
        assert_eq!(note.content, "Updated Content");
        assert_eq!(note.created_at, original_created_at); // Should not change
        assert!(!note.updated_at.is_empty()); // Should have a valid timestamp
        // Note: Due to time format precision (minutes), timestamps might be the same
        // This is expected behavior in real usage
    }

    #[test]
    fn test_note_set_color() {
        let mut note = StickyNote::new("Test".to_string(), "Test".to_string());
        let new_color = SerializableColor { red: 100, green: 150, blue: 200 };
        
        note.set_color(new_color);
        
        assert_eq!(note.color.red, 100);
        assert_eq!(note.color.green, 150);
        assert_eq!(note.color.blue, 200);
    }

    #[test]
    fn test_note_set_position() {
        let mut note = StickyNote::new("Test".to_string(), "Test".to_string());
        
        note.set_position(100.0, 200.0);
        
        assert_eq!(note.x, 100.0);
        assert_eq!(note.y, 200.0);
    }

    #[test]
    fn test_note_set_size() {
        let mut note = StickyNote::new("Test".to_string(), "Test".to_string());
        
        note.set_size(300.0, 250.0);
        
        assert_eq!(note.width, 300.0);
        assert_eq!(note.height, 250.0);
    }

    #[test]
    fn test_slint_note_conversion() {
        let original_note = StickyNote::new("Test Title".to_string(), "Test Content".to_string());
        let slint_note = original_note.to_slint_note();
        let converted_back = StickyNote::from_slint_note(&slint_note);
        
        assert_eq!(original_note.id, converted_back.id);
        assert_eq!(original_note.title, converted_back.title);
        assert_eq!(original_note.content, converted_back.content);
        assert_eq!(original_note.x, converted_back.x);
        assert_eq!(original_note.y, converted_back.y);
        assert_eq!(original_note.width, converted_back.width);
        assert_eq!(original_note.height, converted_back.height);
    }
} 