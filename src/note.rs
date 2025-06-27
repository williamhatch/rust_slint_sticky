use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// RGB color representation that can be serialized
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SerializableColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl From<slint::Color> for SerializableColor {
    fn from(color: slint::Color) -> Self {
        Self {
            red: color.red(),
            green: color.green(),
            blue: color.blue(),
        }
    }
}

impl From<SerializableColor> for slint::Color {
    fn from(color: SerializableColor) -> Self {
        slint::Color::from_rgb_u8(color.red, color.green, color.blue)
    }
}

/// Represents a sticky note with all its properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickyNote {
    pub id: String,
    pub title: String,
    pub content: String,
    pub color: SerializableColor,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub created_at: String,
    pub updated_at: String,
}

impl StickyNote {
    /// Create a new sticky note with default values
    pub fn new(title: String, content: String) -> Self {
        let now = Utc::now().format("%Y-%m-%d %H:%M").to_string();
        
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            content,
            color: SerializableColor { red: 255, green: 235, blue: 59 }, // Yellow
            x: 10.0,
            y: 10.0,
            width: 220.0,
            height: 200.0,
            created_at: now.clone(),
            updated_at: now,
        }
    }
    
    /// Update the note's content and timestamp
    pub fn update_content(&mut self, title: String, content: String) {
        self.title = title;
        self.content = content;
        self.updated_at = Utc::now().format("%Y-%m-%d %H:%M").to_string();
    }
    
    /// Set the note's position
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
        self.updated_at = Utc::now().format("%Y-%m-%d %H:%M").to_string();
    }
    
    /// Set the note's size
    pub fn set_size(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        self.updated_at = Utc::now().format("%Y-%m-%d %H:%M").to_string();
    }
    
    /// Set the note's color
    pub fn set_color(&mut self, color: SerializableColor) {
        self.color = color;
        self.updated_at = Utc::now().format("%Y-%m-%d %H:%M").to_string();
    }
    
    /// Get the slint color representation
    pub fn slint_color(&self) -> slint::Color {
        self.color.into()
    }
}

impl Default for StickyNote {
    fn default() -> Self {
        Self::new("New Note".to_string(), "".to_string())
    }
}

// Convert between our StickyNote and Slint's StickyNote struct
impl StickyNote {
    /// Convert to Slint's StickyNote representation
    pub fn to_slint_note(&self) -> crate::StickyNote {
        crate::StickyNote {
            id: self.id.clone().into(),
            title: self.title.clone().into(),
            content: self.content.clone().into(),
            color: self.color.into(),
            x: self.x.into(),
            y: self.y.into(),
            width: self.width.into(),
            height: self.height.into(),
            created_at: self.created_at.clone().into(),
            updated_at: self.updated_at.clone().into(),
        }
    }
    
    /// Create from Slint's StickyNote representation
    pub fn from_slint_note(note: &crate::StickyNote) -> Self {
        Self {
            id: note.id.to_string(),
            title: note.title.to_string(),
            content: note.content.to_string(),
            color: SerializableColor::from(note.color),
            x: note.x.into(),
            y: note.y.into(),
            width: note.width.into(),
            height: note.height.into(),
            created_at: note.created_at.to_string(),
            updated_at: note.updated_at.to_string(),
        }
    }
} 