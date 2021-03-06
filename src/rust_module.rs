use crate::rust_component::{RustComponent, RustComponentTrait, Visibility};

/// Represents a module in Rust.
///
/// # Example
/// ```
/// use rmod_gen::{RustModule, RustStruct};
/// use rmod_gen::rust_component::{Field, RustComponentTrait};
///
/// // To create the following module:
/// /*
/// #[cfg(test)]
/// mod test_module {
///     use crate::other_module::Struct;
///
///     struct Time<'a, 'b, T> {
///         seconds: u64,
///         minutes: u64,
///         hours: u64,
///     }
/// }
/// */
///
/// let m = RustModule::new("test_module")
///             .with_import("use crate::other_module::Struct")
///             .with_cfg("#[cfg(test)]")
///             .with_component(
///                 RustStruct::new("Time")
///                     .with_lifetime("a")
///                     .with_lifetime("b")
///                     .with_template("T")
///                     .with_field(Field::private("seconds", "u64"))
///                     .with_field(Field::private("minutes", "u64"))
///                     .with_field(Field::private("hours", "u64"))
///                     .into(),
///             );
/// assert_eq!(
///     m.to_rust_string(0),
///     "#[cfg(test)]\nmod test_module {\n    use crate::other_module::Struct;\n\n    struct Time<'a, 'b, T> {\n        seconds: u64,\n        minutes: u64,\n        hours: u64,\n    }\n}\n"
/// );
/// ```
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct RustModule {
    name: String,
    visibility: Visibility,
    imports: Vec<String>,
    components: Vec<RustComponent>,
    cfg_options: String,
}

impl RustModule {
    /// Creates a new instance.
    pub fn new(name: &str) -> Self {
        return Self {
            name: name.to_string(),
            visibility: Visibility::Private,
            cfg_options: String::new(),
            imports: Vec::new(),
            components: Vec::new(),
        };
    }

    /// Specifies a configuration option. This must be the entire line.
    ///
    /// ```
    /// use rmod_gen::RustModule;
    /// use rmod_gen::rust_component::RustComponentTrait;
    /// let m = RustModule::new("my_module").with_cfg("#[cfg(test)]").to_rust_string(0);
    ///
    /// assert_eq!(m, "#[cfg(test)]\nmod my_module {\n}\n");
    /// ```
    pub fn with_cfg(mut self, option: &str) -> Self {
        self.set_cfg(option);

        return self;
    }

    /// Specifies the visibility of this module.
    pub fn with_visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = visibility;

        return self;
    }

    /// Appends an import to the list of imports.
    pub fn with_import(mut self, import: &str) -> Self {
        self.push_import(import);

        return self;
    }

    /// Sets the components.
    pub fn with_components(mut self, components: Vec<RustComponent>) -> Self {
        self.set_components(components);

        return self;
    }

    /// Appends a component.
    pub fn with_component(mut self, component: RustComponent) -> Self {
        self.push_component(component);

        return self;
    }

    /// Specifies a configuration option. This must be the entire line.
    ///
    /// ```
    /// use rmod_gen::RustModule;
    /// use rmod_gen::rust_component::RustComponentTrait;
    /// let mut m = RustModule::new("my_module");
    /// m.set_cfg("#[cfg(test)]");
    ///
    /// let m = m.to_rust_string(0);
    ///
    /// assert_eq!(m, "#[cfg(test)]\nmod my_module {\n}\n");
    /// ```
    pub fn set_cfg(&mut self, option: &str) {
        self.cfg_options = option.to_string();
    }

    /// Specifies the visibility of this module.
    pub fn set_visibility(&mut self, visibility: Visibility) {
        self.visibility = visibility;
    }

    /// Sets the components.
    pub fn set_components(&mut self, components: Vec<RustComponent>) {
        self.components = components;
    }

    /// Appends a component.
    pub fn push_component(&mut self, component: RustComponent) {
        self.components.push(component);
    }

    /// Appends an import to the list of imports.
    pub fn push_import(&mut self, import: &str) {
        self.imports.push(import.to_string());
    }
}

impl Into<RustComponent> for RustModule {
    fn into(self) -> RustComponent {
        return RustComponent::Module(self);
    }
}

impl RustComponentTrait for RustModule {
    fn to_rust_string(&self, indent_level: usize) -> String {
        let mut contents = Vec::new();
        let indent_string = crate::indent_string(indent_level);
        let import_indent_string = crate::indent_string(indent_level + 1);

        if !self.cfg_options.is_empty() {
            contents.push(format!("{}\n", self.cfg_options));
        }

        contents.push(match self.visibility {
            Visibility::Private => format!("{}mod {} {{\n", indent_string, self.name),
            Visibility::Public => format!("{}pub mod {} {{\n", indent_string, self.name),
            Visibility::CrateVisible => {
                format!("{}pub(crate) mod {} {{\n", indent_string, self.name)
            }
        });

        let imports: String = self
            .imports
            .iter()
            .map(|s| [import_indent_string.clone(), s.clone(), String::from(";\n")].join(""))
            .collect();

        if !imports.is_empty() {
            contents.push(imports);
            contents.push(String::from("\n"));
        }

        contents.extend(
            self.components
                .iter()
                .map(|s| s.to_rust_string(indent_level + 1)),
        );

        contents.push(format!("{}}}\n", indent_string));

        return contents.join("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rust_component::Field;
    use crate::RustStruct;

    #[test]
    fn test_struct_module() {
        let m = RustModule::new("test_module").with_component(
            RustStruct::new("Time")
                .with_lifetime("a")
                .with_lifetime("b")
                .with_template("T")
                .with_field(Field::private("seconds", "u64"))
                .with_field(Field::private("minutes", "u64"))
                .with_field(Field::private("hours", "u64"))
                .into(),
        );

        assert_eq!(
            m.to_rust_string(0),
            "mod test_module {\n    struct Time<'a, 'b, T> {\n        seconds: u64,\n        minutes: u64,\n        hours: u64,\n    }\n}\n"
        );
    }

    #[test]
    fn test_struct_module_imports() {
        let m = RustModule::new("test_module")
            .with_import("use crate::other_module::Struct")
            .with_component(
                RustStruct::new("Time")
                    .with_lifetime("a")
                    .with_lifetime("b")
                    .with_template("T")
                    .with_field(Field::private("seconds", "u64"))
                    .with_field(Field::private("minutes", "u64"))
                    .with_field(Field::private("hours", "u64"))
                    .into(),
            );

        assert_eq!(
            m.to_rust_string(0),
            "mod test_module {\n    use crate::other_module::Struct;\n\n    struct Time<'a, 'b, T> {\n        seconds: u64,\n        minutes: u64,\n        hours: u64,\n    }\n}\n"
        );
    }
}
