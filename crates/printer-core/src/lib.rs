//! Shared content model used by native virtual printer adapters.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkdownDocument { pub title: Option<String>, pub blocks: Vec<Block> }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block {
    Heading { level: u8, text: String },
    Paragraph(String),
    Code { language: Option<String>, text: String },
    Quote(String),
    List(Vec<String>),
    Rule,
}

impl MarkdownDocument {
    /// Serialize structured print content as Markdown.
    pub fn to_markdown(&self) -> String {
        let mut out = String::new();
        if let Some(title) = &self.title { out.push_str("# "); out.push_str(title.trim()); out.push_str("\n\n"); }
        for (index, block) in self.blocks.iter().enumerate() {
            match block {
                Block::Heading { level, text } => { out.push_str(&"#".repeat((*level).clamp(1, 6) as usize)); out.push(' '); out.push_str(text.trim()); }
                Block::Paragraph(text) => out.push_str(text.trim()),
                Block::Code { language, text } => { out.push_str("```"); if let Some(language) = language { out.push_str(language.trim()); } out.push('\n'); out.push_str(text.trim_end()); out.push_str("\n```"); }
                Block::Quote(text) => { for (i, line) in text.lines().enumerate() { if i > 0 { out.push('\n'); } out.push_str("> "); out.push_str(line); } }
                Block::List(items) => { for (i, item) in items.iter().enumerate() { if i > 0 { out.push('\n'); } out.push_str("- "); out.push_str(item.trim()); } }
                Block::Rule => out.push_str("---"),
            }
            if index + 1 < self.blocks.len() { out.push_str("\n\n"); }
        }
        out
    }
}
