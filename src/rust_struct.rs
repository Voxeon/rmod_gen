use crate::rust_component::{
    Field, RustComponent, RustComponentTrait, RustTemplateUsage, Visibility,
};

use std::fmt;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct RustStruct {
    name: String,
    fields: Vec<Field>,
    visibility: Visibility,
    templates: Vec<String>,
    lifetimes: Vec<String>,
    extra: String,
}

impl RustStruct {
    pub fn new(name: &str) -> Self {
        return Self {
            name: name.to_string(),
            fields: Vec::new(),
            visibility: Visibility::Private,
            templates: Vec::new(),
            lifetimes: Vec::new(),
            extra: String::new(),
        };
    }

    pub fn with_field(mut self, field: Field) -> Self {
        self.push_field(field);

        return self;
    }

    pub fn with_visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = visibility;

        return self;
    }

    pub fn with_template(mut self, template_identifier: &str) -> Self {
        self.push_template(template_identifier);

        return self;
    }

    /// Adds a lifetime. The lifetime should be only the identifier. i.e. to create a lifetime " 'a "
    ///
    /// ```
    /// use rmod_gen::RustStruct;
    ///
    /// let mut rust_struct = RustStruct::new("struct_name").with_lifetime("a"); // Creates new lifetime 'a
    /// ```
    pub fn with_lifetime(mut self, lifetime_identifier: &str) -> Self {
        self.push_lifetime(lifetime_identifier);

        return self;
    }

    pub fn with_extra(mut self, extra: &str) -> Self {
        self.set_extra(extra);

        return self;
    }

    pub fn push_field(&mut self, field: Field) {
        self.fields.push(field);
    }

    pub fn set_visibility(&mut self, visibility: Visibility) {
        self.visibility = visibility;
    }

    pub fn push_template(&mut self, template_identifier: &str) {
        self.templates.push(template_identifier.to_string());
    }

    /// Adds a lifetime. The lifetime should be only the identifier. i.e. to create a lifetime " 'a "
    ///
    /// ```
    /// use rmod_gen::RustStruct;
    ///
    /// let mut rust_struct = RustStruct::new("struct_name");
    /// rust_struct.push_lifetime("a"); // Creates new lifetime 'a
    /// ```
    pub fn push_lifetime(&mut self, lifetime_identifier: &str) {
        self.lifetimes.push(lifetime_identifier.to_string());
    }

    pub fn set_extra(&mut self, extra: &str) {
        self.extra = extra.to_string();
    }
}

impl Into<RustComponent> for RustStruct {
    fn into(self) -> RustComponent {
        return RustComponent::Struct(self);
    }
}

impl RustTemplateUsage for RustStruct {}

impl RustComponentTrait for RustStruct {
    fn to_rust_string(&self, indent_level: usize) -> String {
        let mut lines = Vec::new();

        let crate_line = match self.visibility {
            Visibility::Private => format!(
                "struct {}{}",
                self.name,
                Self::create_template_string(&self.templates, &self.lifetimes)
            ),
            _ => format!(
                "{} struct {}{}",
                self.visibility,
                self.name,
                Self::create_template_string(&self.templates, &self.lifetimes)
            ),
        };

        if self.extra.is_empty() {
            lines.push(format!("{} {{", crate_line));
        } else {
            lines.push(format!("{} {} {{", crate_line, &self.extra));
        }

        let indent_str = crate::indent_string(1);

        for field in &self.fields {
            lines.push([indent_str.clone(), field.to_string(), ",".to_string()].join(""));
        }

        lines.push(String::from("}"));

        let indent_str = crate::indent_string(indent_level);

        return lines
            .into_iter()
            .map(|l| [indent_str.clone(), l, String::from("\n")].join(""))
            .collect();
    }
}

impl fmt::Display for RustStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return f.write_str(&self.to_rust_string(0));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let s = RustStruct::new("Time")
            .with_field(Field::private("seconds", "u64"))
            .with_field(Field::private("minutes", "u64"))
            .with_field(Field::private("hours", "u64"));

        assert_eq!(
            s.to_rust_string(0),
            "struct Time {\n    seconds: u64,\n    minutes: u64,\n    hours: u64,\n}\n"
        );
    }

    #[test]
    fn lifetimes_test() {
        let s = RustStruct::new("Time")
            .with_lifetime("a")
            .with_lifetime("b")
            .with_field(Field::private("seconds", "&'a u64"))
            .with_field(Field::private("minutes", "u64"))
            .with_field(Field::private("hours", "u64"));

        assert_eq!(
            s.to_rust_string(0),
            "struct Time<'a, 'b> {\n    seconds: &'a u64,\n    minutes: u64,\n    hours: u64,\n}\n"
        );
    }

    #[test]
    fn template_test() {
        let s = RustStruct::new("Time")
            .with_template("T")
            .with_template("P")
            .with_field(Field::private("seconds", "T"))
            .with_field(Field::private("minutes", "P"))
            .with_field(Field::private("hours", "u64"));

        assert_eq!(
            s.to_rust_string(0),
            "struct Time<T, P> {\n    seconds: T,\n    minutes: P,\n    hours: u64,\n}\n"
        );
    }

    #[test]
    fn mixed_test() {
        let s = RustStruct::new("Time")
            .with_lifetime("a")
            .with_lifetime("b")
            .with_template("T")
            .with_field(Field::private("seconds", "u64"))
            .with_field(Field::private("minutes", "u64"))
            .with_field(Field::private("hours", "u64"));

        assert_eq!(
            s.to_rust_string(0),
            "struct Time<'a, 'b, T> {\n    seconds: u64,\n    minutes: u64,\n    hours: u64,\n}\n"
        );
    }
}
