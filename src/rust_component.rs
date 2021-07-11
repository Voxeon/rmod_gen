use crate::{
    EnumVariant, RustEnum, RustImplementation, RustMethod, RustModule, RustStruct, RustVariable,
};

use crate::rust_text::RustText;
use std::fmt;
use std::fmt::Debug;

/// Any rust component should implement this trait, it can then be used as sub-components for
/// components which support it. It provides a method for converting a component into a string of Rust code.
pub trait RustComponentTrait: Into<RustComponent> {
    /// Represent this object as rust code indented to the desired level.
    fn to_rust_string(&self, indent_level: usize) -> String;
}

pub(crate) trait RustTemplateUsage {
    fn create_template_string(templates: &Vec<String>, lifetimes: &Vec<String>) -> String {
        let templates: String = templates.join(", ");
        let lifetimes = Self::create_lifetime_string(lifetimes);

        if lifetimes.is_empty() && templates.is_empty() {
            return String::new();
        } else if lifetimes.len() == 0 {
            return format!("<{}>", templates);
        } else if templates.len() == 0 {
            return format!("<{}>", lifetimes);
        } else {
            return format!("<{}, {}>", lifetimes, templates);
        }
    }

    fn create_lifetime_string(lifetimes: &Vec<String>) -> String {
        let mut res = String::new();

        for i in 0..lifetimes.len() {
            res.push('\'');
            res.push_str(&lifetimes[i]);

            if i != lifetimes.len() - 1 {
                res.push_str(", ");
            }
        }

        return res;
    }
}

/// Represents a field with a name, type and visibility level.
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Field {
    name: String,
    field_type: String,
    visibility: Visibility,
}

/// RustComponent is the base type that is used across the library. Every component must have a RustComponent variant.
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum RustComponent {
    Module(RustModule),
    Struct(RustStruct),
    Enum(RustEnum),
    EnumVariant(EnumVariant),
    Method(RustMethod),
    Implementation(RustImplementation),
    Variable(RustVariable),
    Text(RustText),
}

/// Represents the 3 levels of visibility in Rust.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Visibility {
    Private,
    Public,
    /// pub(crate)
    CrateVisible,
}

impl Field {
    /// Creates a new field with a specified visibility.
    pub fn new(name: &str, field_type: &str, visibility: Visibility) -> Self {
        return Self {
            name: name.to_string(),
            field_type: field_type.to_string(),
            visibility,
        };
    }

    /// Creates a new private field.
    pub fn private(name: &str, field_type: &str) -> Self {
        return Self::new(name, field_type, Visibility::Private);
    }

    /// A fast version that doesn't involve any additional allocations.
    pub(crate) fn private_fast(name: String, field_type: String) -> Self {
        return Self {
            name,
            field_type,
            visibility: Visibility::Private,
        };
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self.visibility {
            Visibility::Private => write!(f, "{}: {}", self.name, self.field_type),
            Visibility::Public => write!(f, "pub {}: {}", self.name, self.field_type),
            Visibility::CrateVisible => write!(f, "pub(crate) {}: {}", self.name, self.field_type),
        };
    }
}

impl RustComponent {
    pub fn to_rust_string(&self, indent_level: usize) -> String {
        return match self {
            RustComponent::Module(o) => o.to_rust_string(indent_level),
            RustComponent::Struct(o) => o.to_rust_string(indent_level),
            RustComponent::Enum(o) => o.to_rust_string(indent_level),
            RustComponent::EnumVariant(o) => o.to_rust_string(indent_level),
            RustComponent::Method(o) => o.to_rust_string(indent_level),
            RustComponent::Implementation(o) => o.to_rust_string(indent_level),
            RustComponent::Variable(o) => o.to_rust_string(indent_level),
            RustComponent::Text(o) => o.to_rust_string(indent_level),
        };
    }
}

impl std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "{}",
            match self {
                Visibility::Private => "",
                Visibility::Public => "pub",
                Visibility::CrateVisible => "pub(crate)",
            }
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_rust_template_usage {
        use super::*;

        struct Tester;

        impl RustTemplateUsage for Tester {}

        #[test]
        fn test_create_lifetime_string() {
            let lifetimes = vec![String::from("a"), String::from("b")];

            assert_eq!(
                Tester::create_lifetime_string(&lifetimes),
                String::from("'a, 'b")
            );
        }

        #[test]
        fn test_create_lifetime_string_2() {
            let lifetimes = vec![];

            assert_eq!(Tester::create_lifetime_string(&lifetimes), String::new());
        }

        #[test]
        fn test_create_template_string() {
            let lifetimes = vec![String::from("a"), String::from("b")];
            let templates = vec![String::from("T")];

            assert_eq!(
                Tester::create_template_string(&templates, &lifetimes),
                String::from("<'a, 'b, T>")
            );
        }

        #[test]
        fn test_create_template_string_2() {
            let lifetimes = vec![String::from("a"), String::from("b")];
            let templates = vec![];

            assert_eq!(
                Tester::create_template_string(&templates, &lifetimes),
                String::from("<'a, 'b>")
            );
        }

        #[test]
        fn test_create_template_string_3() {
            let lifetimes = vec![];
            let templates = vec![String::from("T")];

            assert_eq!(
                Tester::create_template_string(&templates, &lifetimes),
                String::from("<T>")
            );
        }

        #[test]
        fn test_create_template_string_4() {
            let lifetimes = Vec::new();
            let templates = Vec::new();

            assert_eq!(
                Tester::create_template_string(&templates, &lifetimes),
                String::new()
            );
        }
    }
}
