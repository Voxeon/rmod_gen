use crate::rust_component::{RustComponent, RustComponentTrait};

/// Represents some text that can't be represented by any of the other components.
///
/// # Example
/// ```
/// use rmod_gen::RustText;
/// use rmod_gen::rust_component::RustComponentTrait;
///
/// let component = RustText::new("#[cfg(test)]").to_rust_string(0);
/// assert_eq!(component, "#[cfg(test)]".to_string());
/// ```
#[derive(Clone, Debug, PartialEq, Hash)]
pub struct RustText {
    text: String,
}

impl RustText {
    /// Creates a new instance
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
