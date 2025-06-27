use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashSet;

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

/// Note priority levels (for future use)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

/// Workflow status for task automation (for future use)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkflowStatus {
    Idea,          // 初始想法
    Todo,          // 待办任务
    InProgress,    // 进行中
    Review,        // 待审查
    Done,          // 已完成
    Archived,      // 已归档
}

/// Relationship types between notes (for knowledge graph)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RelationType {
    RelatedTo,     // 相关
    DependsOn,     // 依赖
    Blocks,        // 阻塞
    PartOf,        // 属于
    SubtaskOf,     // 子任务
    References,    // 引用
    Conflicts,     // 冲突
    Extends,       // 扩展
}

/// A relationship between two notes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteRelation {
    pub id: String,
    pub from_note_id: String,
    pub to_note_id: String,
    pub relation_type: RelationType,
    pub strength: f32,  // 关联强度 0.0-1.0
    pub created_at: String,
    pub description: Option<String>,
}

/// Internal note representation with enhanced features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppNote {
    pub id: String,
    pub title: String,
    pub content: String,
    pub color: SerializableColor,
    pub text_color: Option<SerializableColor>,  // None means auto-contrast
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub created_at: String,
    pub updated_at: String,
    
    // 扩展字段
    pub tags: HashSet<String>,
    pub keywords: HashSet<String>,
    pub workflow_status: WorkflowStatus,
    pub sentiment: Option<f32>,
    pub priority: Priority,
    pub due_date: Option<String>,
    pub estimated_time: Option<u32>,
    pub completion_percentage: f32,
    pub actual_time: Option<u32>,
}

impl AppNote {
    /// Create a new sticky note with default values
    pub fn new(title: String, content: String) -> Self {
        let now = Utc::now().format("%Y-%m-%d %H:%M").to_string();
        
        let mut note = Self {
            id: Uuid::new_v4().to_string(),
            title,
            content,
            color: SerializableColor { red: 255, green: 235, blue: 59 }, // Yellow
            text_color: None, // Auto-contrast by default
            x: 10.0,
            y: 10.0,
            width: 220.0,
            height: 200.0,
            created_at: now.clone(),
            updated_at: now,
            
            // 扩展字段
            tags: HashSet::new(),
            keywords: HashSet::new(),
            workflow_status: WorkflowStatus::Idea,
            sentiment: None,
            priority: Priority::Medium,
            due_date: None,
            estimated_time: None,
            completion_percentage: 0.0,
            actual_time: None,
        };
        
        // 自动分析内容
        note.extract_keywords();
        note.analyze_sentiment();
        
        note
    }
    
    /// Update the note's content and timestamp
    pub fn update_content(&mut self, title: String, content: String) {
        self.title = title;
        self.content = content;
        self.updated_at = Utc::now().format("%Y-%m-%d %H:%M").to_string();
        
        // 重新分析内容
        self.extract_keywords();
        self.analyze_sentiment();
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
    
    /// Add a tag
    pub fn add_tag(&mut self, tag: String) {
        self.tags.insert(tag.to_lowercase());
        self.updated_at = Utc::now().format("%Y-%m-%d %H:%M").to_string();
    }
    
    /// Remove a tag
    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.remove(&tag.to_lowercase());
        self.updated_at = Utc::now().format("%Y-%m-%d %H:%M").to_string();
    }
    
    /// Set workflow status
    pub fn set_workflow_status(&mut self, status: WorkflowStatus) {
        self.workflow_status = status;
        self.updated_at = Utc::now().format("%Y-%m-%d %H:%M").to_string();
    }
    
    /// Extract keywords from content (simplified implementation)
    pub fn extract_keywords(&mut self) {
        let content_lower = format!("{} {}", self.title, self.content).to_lowercase();
        let words: Vec<&str> = content_lower.split_whitespace().collect();
        
        // 简单的关键词提取
        let keywords: HashSet<String> = words.iter()
            .filter(|word| word.len() > 3)  // 过滤短词
            .filter(|word| !is_stop_word(word))  // 过滤停用词
            .map(|word| word.to_string())
            .collect();
        
        self.keywords = keywords;
    }
    
    /// Analyze sentiment (simplified implementation)
    pub fn analyze_sentiment(&mut self) {
        let content = format!("{} {}", self.title, self.content).to_lowercase();
        
        // 简单的情感分析
        let positive_words = ["good", "great", "excellent", "awesome", "happy", "love", "amazing"];
        let negative_words = ["bad", "terrible", "awful", "hate", "sad", "angry", "frustrated"];
        
        let mut score = 0;
        for word in positive_words.iter() {
            if content.contains(word) {
                score += 1;
            }
        }
        for word in negative_words.iter() {
            if content.contains(word) {
                score -= 1;
            }
        }
        
        self.sentiment = Some((score as f32).max(-1.0).min(1.0));
    }
    
    /// Calculate similarity with another note (for knowledge graph)
    pub fn calculate_similarity(&self, other: &AppNote) -> f32 {
        let mut similarity = 0.0;
        
        // 标签相似度
        let tag_intersection: HashSet<_> = self.tags.intersection(&other.tags).collect();
        let tag_union: HashSet<_> = self.tags.union(&other.tags).collect();
        if !tag_union.is_empty() {
            similarity += 0.3 * (tag_intersection.len() as f32 / tag_union.len() as f32);
        }
        
        // 关键词相似度
        let keyword_intersection: HashSet<_> = self.keywords.intersection(&other.keywords).collect();
        let keyword_union: HashSet<_> = self.keywords.union(&other.keywords).collect();
        if !keyword_union.is_empty() {
            similarity += 0.5 * (keyword_intersection.len() as f32 / keyword_union.len() as f32);
        }
        
        // 情感相似度
        if let (Some(sentiment1), Some(sentiment2)) = (self.sentiment, other.sentiment) {
            similarity += 0.2 * (1.0 - (sentiment1 - sentiment2).abs());
        }
        
        similarity.min(1.0)
    }
    
    /// Get the slint color representation
    pub fn slint_color(&self) -> slint::Color {
        self.color.into()
    }
}

impl Default for AppNote {
    fn default() -> Self {
        Self::new("New Note".to_string(), "".to_string())
    }
}

// Conversion methods moved to main.rs to access Slint-generated structures

/// Knowledge graph manager for handling note relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGraph {
    pub relations: Vec<NoteRelation>,
    pub auto_relation_threshold: f32,  // 自动建立关联的相似度阈值
}

impl KnowledgeGraph {
    pub fn new() -> Self {
        Self {
            relations: Vec::new(),
            auto_relation_threshold: 0.3,
        }
    }
    
    /// Add a relation between two notes
    pub fn add_relation(&mut self, from_id: String, to_id: String, relation_type: RelationType, strength: f32) {
        let relation = NoteRelation {
            id: Uuid::new_v4().to_string(),
            from_note_id: from_id,
            to_note_id: to_id,
            relation_type,
            strength,
            created_at: Utc::now().format("%Y-%m-%d %H:%M").to_string(),
            description: None,
        };
        self.relations.push(relation);
    }
    
    /// Find related notes for a given note
    pub fn find_related_notes(&self, note_id: &str) -> Vec<&NoteRelation> {
        self.relations.iter()
            .filter(|r| r.from_note_id == note_id || r.to_note_id == note_id)
            .collect()
    }
    
    /// Auto-discover relationships between notes based on similarity
    pub fn auto_discover_relations(&mut self, notes: &[AppNote]) {
        for i in 0..notes.len() {
            for j in (i + 1)..notes.len() {
                let similarity = notes[i].calculate_similarity(&notes[j]);
                
                if similarity >= self.auto_relation_threshold {
                    // 检查关系是否已存在
                    let exists = self.relations.iter().any(|r| 
                        (r.from_note_id == notes[i].id && r.to_note_id == notes[j].id) ||
                        (r.from_note_id == notes[j].id && r.to_note_id == notes[i].id)
                    );
                    
                    if !exists {
                        self.add_relation(
                            notes[i].id.clone(),
                            notes[j].id.clone(),
                            RelationType::RelatedTo,
                            similarity
                        );
                    }
                }
            }
        }
    }
}

/// Helper function to check if a word is a stop word
fn is_stop_word(word: &str) -> bool {
    let stop_words = ["the", "and", "or", "but", "in", "on", "at", "to", "for", "of", "with", "by", "is", "are", "was", "were", "be", "been", "have", "has", "had", "do", "does", "did", "will", "would", "could", "should"];
    stop_words.contains(&word)
}

// Removed type alias to avoid conflict with Slint-generated StickyNote struct 