use crate::rust_component::{RustComponent, RustComponentTrait, RustTemplateUsage};

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct RustImplementation {
    name: String,
    components: Vec<RustComponent>,
    impl_lifetimes: Vec<String>,
    target_lifetimes: Vec<String>,
    impl_templates: Vec<String>,
    target_templates: Vec<String>,
    extra: String,
}

impl RustImplementation {
    pub fn new(name: &str) -> Self {
        return Self {
            name: name.to_string(),
            components: Vec::new(),
            impl_lifetimes: Vec::new(),
            target_lifetimes: Vec::new(),
            impl_templates: Vec::new(),
            target_templates: Vec::new(),
            extra: String::new(),
        };
    }

    pub fn new_for(lhs: &str, rhs: &str) -> Self {
        return Self {
            name: format!("{} for {}", lhs, rhs),
            components: Vec::new(),
            impl_lifetimes: Vec::new(),
            target_lifetimes: Vec::new(),
            impl_templates: Vec::new(),
            target_templates: Vec::new(),
            extra: String::new(),
        };
    }

    pub fn with_component(mut self, component: RustComponent) -> Self {
        self.push_component(component);

        return self;
    }

    pub fn with_lifetime(mut self, lifetime: &str) -> Self {
        self.push_lifetime(lifetime);

        return self;
    }

    pub fn with_impl_lifetime(mut self, lifetime: &str) -> Self {
        self.push_impl_lifetime(lifetime);

        return self;
    }

    pub fn with_target_lifetime(mut self, lifetime: &str) -> Self {
        self.push_target_lifetime(lifetime);

        return self;
    }

    pub fn with_template(mut self, template: &str) -> Self {
        self.push_template(template);

        return self;
    }

    pub fn with_impl_template(mut self, template: &str) -> Self {
        self.push_impl_template(template);

        return self;
    }

    pub fn with_target_template(mut self, template: &str) -> Self {
        self.push_target_template(template);

        return self;
    }

    pub fn with_extra(mut self, extra: &str) -> Self {
        self.set_extra(extra);

        return self;
    }

    pub fn push_component(&mut self, component: RustComponent) {
        self.components.push(component);
    }

    pub fn push_lifetime(&mut self, lifetime: &str) {
        self.push_impl_lifetime(lifetime);
        self.push_target_lifetime(lifetime);
    }

    pub fn push_impl_lifetime(&mut self, template: &str) {
        self.impl_lifetimes.push(template.to_string());
    }

    pub fn push_target_lifetime(&mut self, template: &str) {
        self.target_lifetimes.push(template.to_string());
    }

    pub fn push_template(&mut self, template: &str) {
        self.push_impl_template(template);
        self.push_target_template(template);
    }

    pub fn push_impl_template(&mut self, template: &str) {
        self.impl_templates.push(template.to_string());
    }

    pub fn push_target_template(&mut self, template: &str) {
        self.target_templates.push(template.to_string());
    }

    pub fn set_extra(&mut self, extra: &str) {
        self.extra = extra.to_string();
    }
}

impl Into<RustComponent> for RustImplementation {
    fn into(self) -> RustComponent {
        return RustComponent::Implementation(self);
    }
}

impl RustTemplateUsage for RustImplementation {}

impl RustComponentTrait for RustImplementation {
    fn to_rust_string(&self, indent_level: usize) -> String {
        let mut lines = Vec::new();

        let base_indent_level = crate::indent_string(indent_level);

        let definition_line = format!(
            "impl{} {}{}",
            Self::create_template_string(&self.impl_templates, &self.impl_lifetimes),
            self.name,
            Self::create_template_string(&self.target_templates, &self.target_lifetimes)
        );

        if self.extra.is_empty() {
            lines.push(format!("{}{} {{", &base_indent_level, definition_line));
        } else {
            lines.push(format!(
                "{}{} {} {{",
                &base_indent_level, &self.extra, definition_line
            ));
        }

        for component in &self.components {
            lines.push(component.to_rust_string(indent_level + 1));
            lines.push(String::new());
        }

        lines.push(format!("{}}}\n", base_indent_level));

        return lines.join("\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impl_empty() {
        let s = RustImplementation::new("Carton").to_rust_string(0);
        let cmp = "impl Carton {\n}\n".to_string();

        assert_eq!(s, cmp);
    }

    #[test]
    fn test_impl_for_empty() {
        let s = RustImplementation::new_for("Container", "Carton").to_rust_string(0);
        let cmp = "impl Container for Carton {\n}\n".to_string();

        assert_eq!(s, cmp);
    }

    #[test]
    fn test_impl_for_template() {
        let s = RustImplementation::new_for("Container", "Carton")
            .with_template("B")
            .to_rust_string(0);
        let cmp = "impl<B> Container for Carton<B> {\n}\n".to_string();

        assert_eq!(s, cmp);
    }

    #[test]
    fn test_impl_for_impl_template() {
        let s = RustImplementation::new_for("Container", "Carton")
            .with_impl_template("B")
            .to_rust_string(0);
        let cmp = "impl<B> Container for Carton {\n}\n".to_string();

        assert_eq!(s, cmp);
    }

    #[test]
    fn test_impl_for_target_template() {
        let s = RustImplementation::new_for("Container", "Carton")
            .with_target_template("B")
            .to_rust_string(0);
        let cmp = "impl Container for Carton<B> {\n}\n".to_string();

        assert_eq!(s, cmp);
    }
}
