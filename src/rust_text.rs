use crate::rust_component::{RustComponent, RustComponentTrait};

#[derive(Clone, Debug, PartialEq, Hash)]
pub struct RustText {
    text: String,
}

impl RustText {
    pub fn new(text: &str) -> Self {
        return Self {
            text: text.to_string(),
        };
    }
}

impl Into<RustComponent> for RustText {
    fn into(self) -> RustComponent {
        return RustComponent::Text(self);
    }
}

impl RustComponentTrait for RustText {
    fn to_rust_string(&self, indent_level: usize) -> String {
        return format!("{}{}", crate::indent_string(indent_level), &self.text);
    }
}
