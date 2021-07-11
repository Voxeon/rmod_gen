use crate::rust_component::RustComponent;

pub struct RustFile {
    root_components: Vec<RustComponent>,
    imports: Vec<String>,
    file_docstring: String,
    top_misc: String,
    bottom_misc: String,
}

/// Represents a file of rust code
///
/// # Construction
/// Create a new instance using the builder syntax
/// ```
/// use rmod_gen::{RustFile, RustStruct};
///
/// let f: String = RustFile::new()
///                     .with_import("use std::fs::File")
///                     .with_import("use std::io::Write")
///                     .with_file_docstring("Documentation for this file!\nWith Multiple line support!")
///                     .with_component(RustStruct::new("MyStruct").into())
///                     .into_rust_code();
/// ```
impl RustFile {
    pub fn new() -> Self {
        return Self {
            root_components: Vec::new(),
            imports: Vec::new(),
            file_docstring: String::new(),
            top_misc: String::new(),
            bottom_misc: String::new(),
        };
    }

    pub fn with_components(mut self, root_components: Vec<RustComponent>) -> Self {
        self.root_components = root_components;

        return self;
    }

    pub fn with_component(mut self, component: RustComponent) -> Self {
        self.push_component(component);

        return self;
    }

    pub fn with_imports(mut self, imports: Vec<String>) -> Self {
        self.imports = imports;

        return self;
    }

    pub fn with_import(mut self, import: &str) -> Self {
        self.push_import(import.to_string());

        return self;
    }

    pub fn with_file_docstring(mut self, s: &str) -> Self {
        self.file_docstring = s
            .split_inclusive("\n")
            .map(|l| format!("//!{}", l))
            .collect();

        return self;
    }

    pub fn with_top_string(mut self, s: &str) -> Self {
        self.top_misc = s.to_string();

        return self;
    }

    pub fn with_bottom_string(mut self, s: &str) -> Self {
        self.bottom_misc = s.to_string();

        return self;
    }

    pub fn push_component(&mut self, component: RustComponent) {
        self.root_components.push(component);
    }

    pub fn push_import(&mut self, import: String) {
        self.imports.push(import);
    }

    pub fn into_rust_code(self) -> String {
        let mut lines = Vec::new();

        if !self.file_docstring.is_empty() {
            lines.push(self.file_docstring);
            lines.push(String::new()); // Empty line
        }

        if !self.imports.is_empty() {
            lines.extend(
                self.imports
                    .iter()
                    .map(|s| [s.clone(), String::from(";")].join("")),
            );

            lines.push(String::new()); // Empty line
        }

        if !self.top_misc.is_empty() {
            lines.push(self.top_misc);
            lines.push(String::new()); // Empty line
        }

        let has_components = self.root_components.is_empty();

        for component in self.root_components {
            lines.push(component.to_rust_string(0));
        }

        if !has_components {
            lines.push(String::new()); // Empty line
        }

        if !self.bottom_misc.is_empty() {
            lines.push(self.bottom_misc);
            lines.push(String::new()); // Empty line
        }

        return lines.join("\n");
    }

    pub fn to_rust_code(&self) -> String {
        let mut lines = vec![self.file_docstring.clone()];

        lines.push(String::new()); // Empty line

        lines.extend(
            self.imports
                .iter()
                .map(|s| [s.clone(), String::from(";")].join("")),
        );

        lines.push(String::new()); // Empty line

        if !self.top_misc.is_empty() {
            lines.push(self.top_misc.clone());
            lines.push(String::new()); // Empty line
        }

        for component in &self.root_components {
            lines.push(component.to_rust_string(0));
            lines.push(String::new()); // Empty line
        }

        if !self.bottom_misc.is_empty() {
            lines.push(self.bottom_misc.clone());
            lines.push(String::new()); // Empty line
        }

        return lines.join("\n");
    }
}
