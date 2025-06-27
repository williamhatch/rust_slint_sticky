mod note;
mod storage;

use slint::{ComponentHandle, ModelRc, VecModel, Model};
use note::{StickyNote as AppStickyNote, SerializableColor};
use storage::NoteStorage;

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;
    
    // Create initial empty notes model
    let notes_model = ModelRc::new(VecModel::<StickyNote>::default());
    ui.set_notes(notes_model.clone());
    
    // Set up add note callback
    let ui_weak = ui.as_weak();
    ui.on_add_note(move || {
        let ui = ui_weak.unwrap();
        ui.set_show_editor(true);
        ui.set_editor_title("".into());
        ui.set_editor_content("".into());
        ui.set_editor_color(slint::Color::from_rgb_u8(255, 235, 59)); // Yellow
        ui.set_editing_note_id("".into());
    });
    
    // Set up save note callback
    let ui_weak = ui.as_weak();
    let notes_model_clone = notes_model.clone();
    ui.on_save_note(move |title, content, color| {
        let ui = ui_weak.unwrap();
        let editing_id = ui.get_editing_note_id();
        
        // Create new note
        let mut note = AppStickyNote::new(title.to_string(), content.to_string());
        let serializable_color = SerializableColor {
            red: color.red(),
            green: color.green(),
            blue: color.blue(),
        };
        note.set_color(serializable_color);
        let slint_note = note.to_slint_note();
        
        // Update the model
        let vec_model = notes_model_clone.as_any().downcast_ref::<VecModel<StickyNote>>().unwrap();
        if editing_id.is_empty() {
            // Add new note
            vec_model.push(slint_note);
        } else {
            // Update existing note
            for i in 0..vec_model.row_count() {
                if let Some(existing_note) = vec_model.row_data(i) {
                    if existing_note.id == editing_id {
                        // Update the existing note with new data but keep the same ID
                        let mut updated_note = slint_note;
                        updated_note.id = editing_id.clone();
                        vec_model.set_row_data(i, updated_note);
                        break;
                    }
                }
            }
        }
        
        // Close editor
        ui.set_show_editor(false);
        ui.set_editor_title("".into());
        ui.set_editor_content("".into());
        ui.set_editing_note_id("".into());
    });
    
    // Set up edit note callback
    let ui_weak = ui.as_weak();
    ui.on_edit_note(move |note| {
        let ui = ui_weak.unwrap();
        ui.set_show_editor(true);
        ui.set_editor_title(note.title.clone());
        ui.set_editor_content(note.content.clone());
        ui.set_editor_color(note.color);
        ui.set_editing_note_id(note.id.clone());
    });
    
    // Set up delete note callback
    let notes_model_clone = notes_model.clone();
    ui.on_delete_note(move |note_id| {
        let vec_model = notes_model_clone.as_any().downcast_ref::<VecModel<StickyNote>>().unwrap();
        
        // Find and remove the note
        let mut index_to_remove = None;
        for i in 0..vec_model.row_count() {
            if let Some(note) = vec_model.row_data(i) {
                if note.id == note_id {
                    index_to_remove = Some(i);
                    break;
                }
            }
        }
        
        if let Some(index) = index_to_remove {
            vec_model.remove(index);
        }
    });
    
    // Add some sample notes for testing
    let vec_model = notes_model.as_any().downcast_ref::<VecModel<StickyNote>>().unwrap();
    
    // Sample note 1
    let sample_note1 = StickyNote {
        id: "sample-1".into(),
        title: "Welcome to Sticky Notes!".into(),
        content: "This is your first sticky note. Click to edit or use the 'Add New Note' button to create more.".into(),
        color: slint::Color::from_rgb_u8(255, 235, 59),
        x: 220.0.into(),
        y: 200.0.into(),
        width: 220.0.into(),
        height: 200.0.into(),
        created_at: "2025-01-17 10:00".into(),
        updated_at: "2025-01-17 10:00".into(),
    };
    vec_model.push(sample_note1);
    
    // Sample note 2
    let sample_note2 = StickyNote {
        id: "sample-2".into(),
        title: "Features".into(),
        content: "• Create and edit notes\n• Choose colors\n• Persistent storage\n• Cross-platform".into(),
        color: slint::Color::from_rgb_u8(76, 175, 80),
        x: 220.0.into(),
        y: 200.0.into(),
        width: 220.0.into(),
        height: 200.0.into(),
        created_at: "2025-01-17 10:01".into(),
        updated_at: "2025-01-17 10:01".into(),
    };
    vec_model.push(sample_note2);
    
    ui.run()
} 