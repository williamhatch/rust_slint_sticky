mod note;
mod storage;

use slint::{ComponentHandle, ModelRc, VecModel, Model};
use note::{AppNote, SerializableColor, KnowledgeGraph, WorkflowStatus as AppWorkflowStatus, Priority};
use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

slint::include_modules!();

// Conversion functions between AppNote and Slint-generated StickyNote
fn app_note_to_slint_note(note: &AppNote) -> StickyNote {
    StickyNote {
        id: note.id.clone().into(),
        title: note.title.clone().into(),
        content: note.content.clone().into(),
        color: slint::Color::from_rgb_u8(
            note.color.red,
            note.color.green,
            note.color.blue,
        ),
        text_color: match &note.text_color {
            Some(tc) => slint::Color::from_rgb_u8(
                tc.red,
                tc.green,
                tc.blue,
            ),
            None => slint::Color::from_argb_u8(0, 0, 0, 0), // Transparent for auto-contrast
        },
        x: note.x.into(),
        y: note.y.into(),
        width: note.width.into(),
        height: note.height.into(),
        tags: slint::ModelRc::new(slint::VecModel::from(note.tags.iter().map(|tag| tag.clone().into()).collect::<Vec<slint::SharedString>>())),
        workflow_status: match note.workflow_status {
            AppWorkflowStatus::Idea => "Idea".into(),
            AppWorkflowStatus::Todo => "Todo".into(),
            AppWorkflowStatus::InProgress => "In Progress".into(),
            AppWorkflowStatus::Review => "Review".into(),
            AppWorkflowStatus::Done => "Done".into(),
            AppWorkflowStatus::Archived => "Archived".into(),
        },
        priority: match note.priority {
            Priority::Low => "Low".into(),
            Priority::Medium => "Medium".into(),
            Priority::High => "High".into(),
            Priority::Urgent => "Urgent".into(),
        },
        due_date: note.due_date.clone().unwrap_or_default().into(),
        estimated_time: note.estimated_time.unwrap_or(0) as i32,
        completion_percentage: note.completion_percentage as f32,
        updated_at: note.updated_at.clone().into(),
    }
}

fn slint_note_to_app_note(note: &StickyNote) -> AppNote {
    let mut new_note = AppNote {
        id: note.id.to_string(),
        title: note.title.to_string(),
        content: note.content.to_string(),
        color: SerializableColor::from(note.color),
        text_color: if note.text_color.alpha() > 0 { Some(SerializableColor::from(note.text_color)) } else { None },
        x: note.x.into(),
        y: note.y.into(),
        width: note.width.into(),
        height: note.height.into(),
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: note.updated_at.to_string(),
        
        // 扩展字段
        tags: HashSet::new(),
        keywords: HashSet::new(),
        workflow_status: AppWorkflowStatus::Idea,
        sentiment: None,
        priority: Priority::Medium,
        due_date: None,
        estimated_time: None,
        completion_percentage: 0.0,
        actual_time: None,
    };
    
    // 自动分析内容
    new_note.extract_keywords();
    new_note.analyze_sentiment();
    
    new_note
}

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;
    
    // 创建便签和关联关系的数据模型
    let notes_model = ModelRc::new(VecModel::<StickyNote>::default());
    let relations_model = ModelRc::new(VecModel::<NoteRelation>::default());
    
    ui.set_notes(notes_model.clone());
    ui.set_relations(relations_model.clone());
    
    // 知识图谱管理器
    let mut knowledge_graph = KnowledgeGraph::new();
    let mut app_notes: Vec<AppNote> = Vec::new();
    
    // 设置快速添加便签回调
    let ui_weak = ui.as_weak();
    let notes_model_clone = notes_model.clone();
    ui.on_quick_add_note(move |text| {
        let ui = ui_weak.unwrap();
        let notes = notes_model_clone.as_any().downcast_ref::<VecModel<StickyNote>>().unwrap();
        
        let mut new_note = AppNote::new(
            if text.len() > 30 { format!("{}...", &text[..27]) } else { text.to_string() },
            text.to_string()
        );
        
        // Set position to avoid overlap
        new_note.set_position(
            50.0 + (notes.row_count() as f32 * 20.0),
            50.0 + (notes.row_count() as f32 * 20.0)
        );
        
        // Auto-extract keywords and analyze sentiment
        new_note.extract_keywords();
        new_note.analyze_sentiment();
        
        notes.push(app_note_to_slint_note(&new_note));
        println!("✨ Quick added note: {}", text);
        
        // 同步更新filtered_notes显示
        let current_filter = ui.get_filter_status();
        let current_search = ui.get_search_text();
        
        if !current_search.is_empty() {
            // 如果当前有搜索条件，重新执行搜索
            ui.invoke_search_notes(current_search);
        } else if current_filter != "All" {
            // 如果当前有过滤条件，重新执行过滤  
            ui.invoke_filter_notes_by_status(current_filter);
        } else {
            // 如果没有过滤/搜索条件，显示所有笔记
            ui.set_filtered_notes(notes_model_clone.clone().into());
        }
    });

    // 设置添加便签回调
    let ui_weak = ui.as_weak();
    ui.on_add_note(move || {
        let ui = ui_weak.unwrap();
        ui.set_show_editor(true);
        ui.set_editor_title("".into());
        ui.set_editor_content("".into());
        ui.set_editor_color(slint::Color::from_rgb_u8(255, 235, 59)); // Yellow
        ui.set_editing_note_id("".into());
    });
    
    // 设置保存便签回调（增强版）
    let ui_weak = ui.as_weak();
    let notes_model_clone = notes_model.clone();
    ui.on_save_note(move |title, content, color, text_color, tags_text, workflow_status, priority, due_date, estimated_time| {
        let ui = ui_weak.unwrap();
        let editing_id = ui.get_editing_note_id();
        
        // 创建新便签
        let mut note = AppNote::new(title.to_string(), content.to_string());
        let serializable_color = SerializableColor {
            red: color.red(),
            green: color.green(),
            blue: color.blue(),
        };
        note.set_color(serializable_color);
        
        // Set text color (None if transparent/auto-contrast)
        if text_color.alpha() > 0 {
            note.text_color = Some(SerializableColor {
                red: text_color.red(),
                green: text_color.green(),
                blue: text_color.blue(),
            });
        } else {
            note.text_color = None; // Auto-contrast
        }
        
        // 解析并设置标签
        let tags: Vec<&str> = tags_text.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
        for tag in tags {
            note.add_tag(tag.to_string());
        }
        
        // 设置工作流状态
        let status = match workflow_status.as_str() {
            "Todo" => AppWorkflowStatus::Todo,
            "Progress" => AppWorkflowStatus::InProgress,
            "Review" => AppWorkflowStatus::Review,
            "Done" => AppWorkflowStatus::Done,
            "Archived" => AppWorkflowStatus::Archived,
            _ => AppWorkflowStatus::Idea,
        };
        note.set_workflow_status(status);
        
        // 设置优先级
        note.priority = match priority.as_str() {
            "Low" => Priority::Low,
            "High" => Priority::High,
            "Urgent" => Priority::Urgent,
            _ => Priority::Medium,
        };
        
        // 设置截止日期和预估时间
        if !due_date.is_empty() {
            note.due_date = Some(due_date.to_string());
        }
        if estimated_time > 0 {
            note.estimated_time = Some(estimated_time as u32);
        }
        
        // 如果编辑现有便签，找到合适的位置
        if !editing_id.is_empty() {
            // 在编辑现有便签时保持原位置
            let vec_model = notes_model_clone.as_any().downcast_ref::<VecModel<StickyNote>>().unwrap();
            for i in 0..vec_model.row_count() {
                if let Some(existing_note) = vec_model.row_data(i) {
                    if existing_note.id == editing_id {
                        note.set_position(existing_note.x.into(), existing_note.y.into());
                        note.set_size(existing_note.width.into(), existing_note.height.into());
                        break;
                    }
                }
            }
        } else {
            // 为新便签设置随机位置，避免重叠
            // 使用便签ID生成伪随机位置
            let mut hasher = DefaultHasher::new();
            note.id.hash(&mut hasher);
            let hash_value = hasher.finish();
            
            let random_x = ((hash_value % 500) as f32) + 50.0;
            let random_y = (((hash_value >> 16) % 300) as f32) + 100.0;
            note.set_position(random_x, random_y);
        }
        
        let slint_note = app_note_to_slint_note(&note);
        
        // 更新模型
        let vec_model = notes_model_clone.as_any().downcast_ref::<VecModel<StickyNote>>().unwrap();
        if editing_id.is_empty() {
            // 添加新便签
            vec_model.push(slint_note);
        } else {
            // 更新现有便签
            for i in 0..vec_model.row_count() {
                if let Some(existing_note) = vec_model.row_data(i) {
                    if existing_note.id == editing_id {
                        let mut updated_note = slint_note;
                        updated_note.id = editing_id.clone();
                        vec_model.set_row_data(i, updated_note);
                        break;
                    }
                }
            }
        }
        
        // 关闭编辑器
        ui.set_show_editor(false);
        ui.set_editor_title("".into());
        ui.set_editor_content("".into());
        ui.set_editing_note_id("".into());
        
        // 同步更新filtered_notes显示
        let current_filter = ui.get_filter_status();
        let current_search = ui.get_search_text();
        
        if !current_search.is_empty() {
            // 如果当前有搜索条件，重新执行搜索
            ui.invoke_search_notes(current_search);
        } else if current_filter != "All" {
            // 如果当前有过滤条件，重新执行过滤
            ui.invoke_filter_notes_by_status(current_filter);
        } else {
            // 如果没有过滤/搜索条件，显示所有笔记
            ui.set_filtered_notes(notes_model_clone.clone().into());
        }
    });
    
    // 设置编辑便签回调
    let ui_weak = ui.as_weak();
    ui.on_edit_note(move |note| {
        let ui = ui_weak.unwrap();
        ui.set_show_editor(true);
        ui.set_editor_title(note.title.clone());
        ui.set_editor_content(note.content.clone());
        ui.set_editor_color(note.color);
        ui.set_editing_note_id(note.id.clone());
    });
    
    // 设置删除便签回调
    let notes_model_clone = notes_model.clone();
    let ui_weak = ui.as_weak();
    ui.on_delete_note(move |note_id| {
        let ui = ui_weak.unwrap();
        let vec_model = notes_model_clone.as_any().downcast_ref::<VecModel<StickyNote>>().unwrap();
        
        // 找到并删除便签
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
            println!("🗑️ Deleted note: {}", note_id);
            
            // 同步更新filtered_notes显示
            let current_filter = ui.get_filter_status();
            let current_search = ui.get_search_text();
            
            if !current_search.is_empty() {
                // 如果当前有搜索条件，重新执行搜索
                ui.invoke_search_notes(current_search);
            } else if current_filter != "All" {
                // 如果当前有过滤条件，重新执行过滤
                ui.invoke_filter_notes_by_status(current_filter);
            } else {
                // 如果没有过滤/搜索条件，显示所有剩余笔记
                ui.set_filtered_notes(notes_model_clone.clone().into());
            }
        }
    });
    
    // New feature: drag position update callback
    let notes_model_clone = notes_model.clone();
    ui.on_position_changed(move |note_id, x, y| {
        let vec_model = notes_model_clone.as_any().downcast_ref::<VecModel<StickyNote>>().unwrap();
        
        // Update note position
        for i in 0..vec_model.row_count() {
            if let Some(mut note) = vec_model.row_data(i) {
                if note.id == note_id {
                    note.x = x;
                    note.y = y;
                    vec_model.set_row_data(i, note);
                    break;
                }
            }
        }
    });
    
    // 🔥 修复工作流状态更改回调 - 真正更新数据模型
    let notes_model_clone = notes_model.clone();
    let ui_weak = ui.as_weak();
    ui.on_workflow_status_changed(move |note_id, status| {
        let ui = ui_weak.unwrap();
        let vec_model = notes_model_clone.as_any().downcast_ref::<VecModel<StickyNote>>().unwrap();
        
        // Update note's workflow_status in the data model
        for i in 0..vec_model.row_count() {
            if let Some(mut note) = vec_model.row_data(i) {
                if note.id == note_id {
                    // Update status
                    note.workflow_status = status.clone();
                    // Update data in model
                    vec_model.set_row_data(i, note);
                    println!("✅ Note {} status UPDATED to: {}", note_id, status);
                    
                    // Update filtered_notes to maintain consistency
                    // If currently showing all notes, update filtered_notes as well
                    if ui.get_filter_status() == "All" {
                        ui.set_filtered_notes(notes_model_clone.clone().into());
                    } else {
                        // If currently filtered, reapply the filter
                        let current_filter = ui.get_filter_status();
                        ui.invoke_filter_notes_by_status(current_filter);
                    }
                    break;
                }
            }
        }
    });
    
    // New feature: knowledge graph toggle callback
    let ui_weak = ui.as_weak();
    ui.on_toggle_knowledge_graph(move || {
        let ui = ui_weak.unwrap();
        println!("Knowledge graph display status: {}", ui.get_show_knowledge_graph());
    });
    
    // New feature: auto discover relations callback
    let notes_model_clone = notes_model.clone();
    let relations_model_clone = relations_model.clone();
    ui.on_auto_discover_relations(move || {
        println!("Starting auto discovery of note relations...");
        
        // Get all notes from UI model
        let vec_model = notes_model_clone.as_any().downcast_ref::<VecModel<StickyNote>>().unwrap();
        let mut app_notes_temp: Vec<AppNote> = Vec::new();
        
        for i in 0..vec_model.row_count() {
            if let Some(slint_note) = vec_model.row_data(i) {
                let app_note = slint_note_to_app_note(&slint_note);
                app_notes_temp.push(app_note);
            }
        }
        
        // Create temporary knowledge graph and discover relations
        let mut temp_graph = KnowledgeGraph::new();
        temp_graph.auto_discover_relations(&app_notes_temp);
        
        // Update relation model
        let relations_vec_model = relations_model_clone.as_any().downcast_ref::<VecModel<NoteRelation>>().unwrap();
        // Clear existing relations
        while relations_vec_model.row_count() > 0 {
            relations_vec_model.remove(0);
        }
        
        let relations_count = temp_graph.relations.len();
        for relation in temp_graph.relations {
            let slint_relation = NoteRelation {
                from_note_id: relation.from_note_id.into(),
                to_note_id: relation.to_note_id.into(),
                relation_type: format!("{:?}", relation.relation_type).into(),
                strength: relation.strength,
            };
            relations_vec_model.push(slint_relation);
        }
        
        println!("Discovered {} relations", relations_count);
    });
    
    // Search notes functionality will be implemented after filter functionality
    
    // Add some smart sample notes
    let vec_model = notes_model.as_any().downcast_ref::<VecModel<StickyNote>>().unwrap();
    
    // Sample note 1: Project planning
    let mut sample_note1 = AppNote::new(
        "AI Project Launch".to_string(), 
        "Need to develop an intelligent sticky notes system\n- Implement knowledge graph\n- Add workflow automation\n- Support drag functionality\nDeadline: End of this month".to_string()
    );
    sample_note1.add_tag("AI".to_string());
    sample_note1.add_tag("project".to_string());
    sample_note1.add_tag("important".to_string());
    sample_note1.set_workflow_status(AppWorkflowStatus::InProgress);
    sample_note1.priority = Priority::High;
    sample_note1.set_position(100.0, 150.0);
    sample_note1.estimated_time = Some(240); // 4 hours
    let slint_note1 = app_note_to_slint_note(&sample_note1);
    vec_model.push(slint_note1);
    
    // Sample note 2: Technical research
    let mut sample_note2 = AppNote::new(
        "Rust & Slint Research".to_string(),
        "Deep dive into Rust programming language and Slint UI framework\n- Master ownership concepts\n- Understand UI component design\n- Practice cross-platform development\nThis tech stack is very promising!".to_string()
    );
    sample_note2.add_tag("Rust".to_string());
    sample_note2.add_tag("learning".to_string());
    sample_note2.add_tag("technology".to_string());
    sample_note2.set_workflow_status(AppWorkflowStatus::Todo);
    sample_note2.priority = Priority::Medium;
    sample_note2.set_position(350.0, 200.0);
    sample_note2.set_color(SerializableColor { red: 76, green: 175, blue: 80 });
    sample_note2.estimated_time = Some(180); // 3 hours
    let slint_note2 = app_note_to_slint_note(&sample_note2);
    vec_model.push(slint_note2);
    
    // Sample note 3: UI design
    let mut sample_note3 = AppNote::new(
        "UI/UX Design Thoughts".to_string(),
        "User interface should be intuitive and easy to use\n- Support drag operations\n- Display relationships\n- Smart recommendation features\nUser experience is key!".to_string()
    );
    sample_note3.add_tag("UI".to_string());
    sample_note3.add_tag("design".to_string());
    sample_note3.add_tag("UX".to_string());
    sample_note3.set_workflow_status(AppWorkflowStatus::Review);
    sample_note3.priority = Priority::Medium;
    sample_note3.set_position(150.0, 350.0);
    sample_note3.set_color(SerializableColor { red: 33, green: 150, blue: 243 });
    sample_note3.estimated_time = Some(120); // 2 hours
    let slint_note3 = app_note_to_slint_note(&sample_note3);
    vec_model.push(slint_note3);
    
    // Sample note 4: Completed task
    let mut sample_note4 = AppNote::new(
        "Data Structure Design".to_string(),
        "Completed the design of note data structure\n✓ Added tag system\n✓ Implemented relationships\n✓ Support workflow status\nNext step: Frontend integration".to_string()
    );
    sample_note4.add_tag("completed".to_string());
    sample_note4.add_tag("data-structure".to_string());
    sample_note4.set_workflow_status(AppWorkflowStatus::Done);
    sample_note4.priority = Priority::Low;
    sample_note4.set_position(400.0, 100.0);
    sample_note4.set_color(SerializableColor { red: 156, green: 39, blue: 176 });
    sample_note4.completion_percentage = 100.0;
    sample_note4.actual_time = Some(150); // Actually took 2.5 hours
    let slint_note4 = app_note_to_slint_note(&sample_note4);
    vec_model.push(slint_note4);
    
    // Save all sample notes to app_notes vector
    app_notes.push(sample_note1);
    app_notes.push(sample_note2);
    app_notes.push(sample_note3);
    app_notes.push(sample_note4);
    
    // Auto discover initial relations
    knowledge_graph.auto_discover_relations(&app_notes);
    
    // Add discovered relations to UI model
    let relations_vec_model = relations_model.as_any().downcast_ref::<VecModel<NoteRelation>>().unwrap();
    for relation in &knowledge_graph.relations {
        let slint_relation = NoteRelation {
            from_note_id: relation.from_note_id.clone().into(),
            to_note_id: relation.to_note_id.clone().into(),
            relation_type: format!("{:?}", relation.relation_type).into(),
            strength: relation.strength,
        };
        relations_vec_model.push(slint_relation);
    }
    
    // Implement filtering functionality
    let notes_model_clone = notes_model.clone();
    let ui_weak = ui.as_weak();
    ui.on_filter_notes_by_status(move |status| {
        let ui = ui_weak.unwrap();
        let vec_model = notes_model_clone.as_any().downcast_ref::<VecModel<StickyNote>>().unwrap();
        
        println!("Filtering notes by status: {}", status);
        
        if status == "All" {
            // Show all notes
            ui.set_filtered_notes(notes_model_clone.clone().into());
            println!("Showing all {} notes", vec_model.row_count());
        } else {
            // Filter notes matching the status
            let mut filtered_notes = Vec::new();
            let mut count = 0;
            
            for i in 0..vec_model.row_count() {
                if let Some(note) = vec_model.row_data(i) {
                    if note.workflow_status == status {
                        filtered_notes.push(note);
                        count += 1;
                    }
                }
            }
            
            let filtered_model = std::rc::Rc::new(VecModel::from(filtered_notes));
            ui.set_filtered_notes(filtered_model.into());
            println!("Found {} notes with status: {}", count, status);
        }
    });
    
    // Implement search notes functionality
    let notes_model_clone = notes_model.clone();
    let ui_weak = ui.as_weak();
    ui.on_search_notes(move |search_text| {
        let ui = ui_weak.unwrap();
        let vec_model = notes_model_clone.as_any().downcast_ref::<VecModel<StickyNote>>().unwrap();
        
        println!("🔍 Searching notes: '{}'", search_text);
        
        if search_text.is_empty() {
            // If search is empty, show all notes
            ui.set_filtered_notes(notes_model_clone.clone().into());
            println!("✅ Showing all {} notes", vec_model.row_count());
        } else {
            // Search for matching notes
            let mut filtered_notes = Vec::new();
            let search_lower = search_text.to_lowercase();
            
            for i in 0..vec_model.row_count() {
                if let Some(note) = vec_model.row_data(i) {
                    let title_match = note.title.to_lowercase().contains(&search_lower);
                    let content_match = note.content.to_lowercase().contains(&search_lower);
                    let tags_match = note.tags.iter().any(|tag| tag.to_lowercase().contains(&search_lower));
                    
                    if title_match || content_match || tags_match {
                        filtered_notes.push(note);
                    }
                }
            }
            
            let count = filtered_notes.len();
            let filtered_model = std::rc::Rc::new(VecModel::from(filtered_notes));
            ui.set_filtered_notes(filtered_model.into());
            println!("🎯 Found {} notes matching '{}'", count, search_text);
        }
    });
    
    // Set initial filter to show all notes
    ui.set_filtered_notes(notes_model.clone().into());
    
    println!("🚀 Smart sticky notes system launched successfully!");
    println!("📊 Loaded {} sample notes", app_notes.len());
    println!("🔗 Discovered {} relations", knowledge_graph.relations.len());
    println!("✨ Supports drag & drop, knowledge graph, and workflow automation features");
    
    ui.run()
} 