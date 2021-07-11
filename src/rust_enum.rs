use crate::rust_component::{
    Field, RustComponent, RustComponentTrait, RustTemplateUsage, Visibility,
};

/// Represents an enum in rust.
///
/// # Example
/// ```
/// use rmod_gen::{RustEnum, EnumVariant};
/// use rmod_gen::rust_component::{Visibility, RustComponentTrait};
///
/// // Creates a new rust enum titled Animals, with a template parameter and a variant with a field.
///
/// // The generated code would appear like this when formatted correctly:
/// /*
/// pub(crate) enum Animals<T> {
///     Cow {
///         age: u64,
///     },
///     Dog {
///         age: u64,
///         weight: u64,
///     },
/// }
/// */
///
/// let e = RustEnum::new("Animals")
///     .with_visibility(Visibility::CrateVisible)
///     .with_template("T")
///     .with_variant(EnumVariant::build("Cow").with_field("age", "u64").build())
///     .with_variant(
///         EnumVariant::build("Dog")
///             .with_field("age", "u64")
///             .with_field("weight", "u64")
///             .build(),
///     );
///
///  assert_eq!(
///     e.to_rust_string(0),
///     "pub(crate) enum Animals<T> {\n    Cow {\n        age: u64,\n    },\n    Dog {\n        age: u64,\n        weight: u64,\n    },\n}\n".to_string()
///  );
/// ```
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct RustEnum {
    name: String,
    visibility: Visibility,
    variants: Vec<EnumVariant>,
    templates: Vec<String>,
    lifetimes: Vec<String>,
    extra: String,
    cfg: String,
}

/// Represents an enum variant in Rust. It supports Struct, Value and Empty variants.
#[derive(Clone, Debug, Hash, PartialEq)]
pub enum EnumVariant {
    /// Represents an enum variant that is a struct.
    /// ```
    /// use rmod_gen::EnumVariant;
    ///
    /// // To represent the following enum
    /// /*
    /// enum MyEnum {
    ///     MyVariant {
    ///         field: String,
    ///     }
    /// }
    /// */
    ///
    /// let my_variant = EnumVariant::build("MyVariant").with_field("field", "String").build();
    /// ```
    StructVariant {
        name: String,
        /// Represents the fields for this struct variant. Field visibility is ignored when generating enums.
        fields: Vec<Field>,
    },
    /// Represents an enum variant that is a value.
    /// ```
    /// use rmod_gen::EnumVariant;
    ///
    /// // To represent the following enum
    /// /*
    /// enum MyEnum {
    ///     MyVariant(String),
    /// }
    /// */
    ///
    /// let my_variant = EnumVariant::build("MyVariant").with_value("String").build();
    /// ```
    ValueVariant { name: String, types: Vec<String> },
    /// Represents an enum variant that is simply a variant.
    /// ```
    /// use rmod_gen::EnumVariant;
    ///
    /// // To represent the following enum
    /// /*
    /// enum MyEnum {
    ///     MyVariant,
    /// }
    /// */
    ///
    /// let my_variant = EnumVariant::build("MyVariant").build();
    /// ```
    EmptyVariant { name: String },
}

/// Used to build enum variants where there are multiple fields or values. It is most useful when
/// you need to dynamically generate an enum.
#[derive(Debug)]
pub struct EnumVariantBuilder {
    name: String,
    struct_variant: bool,
    fields: Vec<(String, String)>,
}

impl RustEnum {
    /// Creates a new empty instance.
    pub fn new(name: &str) -> Self {
        return Self {
            name: name.to_string(),
            visibility: Visibility::Private,
            variants: Vec::new(),
            templates: Vec::new(),
            lifetimes: Vec::new(),
            extra: String::new(),
            cfg: String::new(),
        };
    }

    /// Appends a new variant to the enum.
    pub fn with_variant(mut self, variant: EnumVariant) -> Self {
        self.push_variant(variant);

        return self;
    }

    /// Sets the visibility for the enum.
    pub fn with_visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = visibility;

        return self;
    }

    /// Appends a template value for the enum.
    pub fn with_template(mut self, template_identifier: &str) -> Self {
        self.push_template(template_identifier);

        return self;
    }

    /// Adds a lifetime. The lifetime should be only the identifier. i.e. to create a lifetime " 'a "
    ///
    /// ```
    /// use rmod_gen::RustEnum;
    ///
    /// // enum enum_name<'a>
    /// let mut rust_enum = RustEnum::new("enum_name").with_lifetime("a"); // Creates new lifetime 'a
    /// ```
    pub fn with_lifetime(mut self, lifetime_identifier: &str) -> Self {
        self.push_lifetime(lifetime_identifier);

        return self;
    }

    /// Adds some extra text after the enum's name but before the opening brace
    ///
    /// ```
    /// use rmod_gen::RustEnum;
    /// use rmod_gen::rust_component::RustComponentTrait;
    ///
    /// let rust_enum = RustEnum::new("n").with_extra("where T: Debug").to_rust_string(0);
    ///
    /// assert_eq!(rust_enum, "enum n where T: Debug {\n}\n");
    /// ```
    pub fn with_extra(mut self, extra: &str) -> Self {
        self.set_extra(extra);

        return self;
    }

    /// Adds some extra text after the enum's name but before the opening brace
    ///
    /// ```
    /// use rmod_gen::RustEnum;
    /// use rmod_gen::rust_component::RustComponentTrait;
    ///
    /// let rust_enum = RustEnum::new("n").with_cfg("#[derive(Clone)]").to_rust_string(0);
    ///
    /// assert_eq!(rust_enum, "#[derive(Clone)]\nenum n {\n}\n");
    /// ```
    pub fn with_cfg(mut self, cfg: &str) -> Self {
        self.set_cfg(cfg);

        return self;
    }

    /// Appends a new enum variant.
    pub fn push_variant(&mut self, variant: EnumVariant) {
        self.variants.push(variant);
    }

    /// Sets the visibility of this enum.
    pub fn set_visibility(&mut self, visibility: Visibility) {
        self.visibility = visibility;
    }

    /// Appends a template parameter.
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

    /// Adds some extra text after the enum's name but before the opening brace
    ///
    /// ```
    /// use rmod_gen::RustEnum;
    /// use rmod_gen::rust_component::RustComponentTrait;
    ///
    /// let mut rust_enum = RustEnum::new("n");
    /// rust_enum.set_extra("where T: Debug");
    ///
    /// let rust_enum = rust_enum.to_rust_string(0);
    ///
    /// assert_eq!(rust_enum, "enum n where T: Debug {\n}\n");
    /// ```
    pub fn set_extra(&mut self, extra: &str) {
        self.extra = extra.to_string();
    }

    /// Adds some extra text after the enum's name but before the opening brace
    ///
    /// ```
    /// use rmod_gen::RustEnum;
    /// use rmod_gen::rust_component::RustComponentTrait;
    ///
    /// let mut rust_enum = RustEnum::new("n");
    /// rust_enum.set_cfg("#[derive(Clone)]");
    /// let rust_enum = rust_enum.to_rust_string(0);
    ///
    /// assert_eq!(rust_enum, "#[derive(Clone)]\nenum n {\n}\n");
    /// ```
    pub fn set_cfg(&mut self, cfg: &str) {
        self.cfg = cfg.to_string();
    }
}

impl EnumVariant {
    /// Creates a new struct enum variant.
    pub fn new_struct(name: &str, fields: Vec<Field>) -> Self {
        return Self::StructVariant {
            name: name.to_string(),
            fields,
        };
    }

    /// Creates a new value enum variant.
    pub fn new_value(name: &str, types: Vec<String>) -> Self {
        return Self::ValueVariant {
            name: name.to_string(),
            types,
        };
    }

    /// Creates a new empty enum variant.
    pub fn new_empty(name: &str) -> Self {
        return Self::EmptyVariant {
            name: name.to_string(),
        };
    }

    /// Creates a new builder with the specified name.
    pub fn build(name: &str) -> EnumVariantBuilder {
        return EnumVariantBuilder::new(name);
    }
}

impl EnumVariantBuilder {
    pub(crate) fn new(name: &str) -> Self {
        return Self {
            name: name.to_string(),
            struct_variant: false,
            fields: Vec::new(),
        };
    }

    /// Finish building and generate the corresponding enum variant based on the input supplied.
    pub fn build(self) -> EnumVariant {
        let name = self.name.clone();

        if self.struct_variant {
            return EnumVariant::StructVariant {
                name,
                fields: self.fields(),
            };
        }

        if self.fields.is_empty() {
            return EnumVariant::EmptyVariant { name };
        }

        return EnumVariant::ValueVariant {
            name,
            types: self.types(),
        };
    }

    /// Add a new field.
    pub fn with_field(mut self, name: &str, tp: &str) -> Self {
        self.push_field(name, tp);

        return self;
    }

    /// Add a new value.
    pub fn with_value(mut self, tp: &str) -> Self {
        self.push_value(tp);

        return self;
    }

    /// Add a new field.
    pub fn push_field(&mut self, name: &str, tp: &str) {
        self.struct_variant = true;

        self.fields.push((name.to_string(), tp.to_string()));
    }

    /// Add a new value.
    pub fn push_value(&mut self, tp: &str) {
        self.fields
            .push((self.fields.len().to_string(), tp.to_string()));
    }

    fn fields(self) -> Vec<Field> {
        return self
            .fields
            .into_iter()
            .map(|(name, tp)| Field::private_fast(name, tp))
            .collect();
    }

    fn types(self) -> Vec<String> {
        return self.fields.into_iter().map(|(_name, tp)| tp).collect();
    }
}

impl Into<RustComponent> for RustEnum {
    fn into(self) -> RustComponent {
        return RustComponent::Enum(self);
    }
}

impl RustTemplateUsage for RustEnum {}

impl RustComponentTrait for RustEnum {
    fn to_rust_string(&self, indent_level: usize) -> String {
        let mut lines;

        if self.cfg.is_empty() {
            lines = Vec::new();
        } else {
            lines = vec![self.cfg.clone()];
        }

        let crate_line = match self.visibility {
            Visibility::Private => format!(
                "enum {}{}",
                self.name,
                Self::create_template_string(&self.templates, &self.lifetimes)
            ),
            _ => format!(
                "{} enum {}{}",
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

        for variant in &self.variants {
            lines.push(variant.to_rust_string(indent_level + 1));
        }

        lines.push(String::from("}"));

        let indent_str = crate::indent_string(indent_level);

        return lines
            .into_iter()
            .map(|l| [indent_str.clone(), l, String::from("\n")].join(""))
            .collect();
    }
}

impl Into<RustComponent> for EnumVariant {
    fn into(self) -> RustComponent {
        return RustComponent::EnumVariant(self);
    }
}

impl RustComponentTrait for EnumVariant {
    fn to_rust_string(&self, indent_level: usize) -> String {
        let indent_string = crate::indent_string(indent_level);

        return match self {
            EnumVariant::StructVariant { name, fields } => {
                let nested_indent_string = crate::indent_string(indent_level + 1);
                let f_str: String = fields
                    .iter()
                    .map(|f| format!("{}{},\n", &nested_indent_string, f))
                    .collect();

                format!(
                    "{}{} {{\n{}{}}},",
                    &indent_string, name, f_str, &indent_string
                )
            }
            EnumVariant::ValueVariant { name, types } => {
                format!("{}{}({}),", indent_string, name, types.join(", "))
            }
            EnumVariant::EmptyVariant { name } => {
                format!("{}{},", indent_string, name)
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_enum_variant {
        use super::*;

        #[test]
        fn test_empty_variant() {
            let variant = EnumVariantBuilder::new("Carton").build();

            assert_eq!(variant.to_rust_string(0), "Carton,");
        }

        #[test]
        fn test_value_variant() {
            let variant = EnumVariantBuilder::new("Carton")
                .with_value("u64")
                .with_value("u64")
                .build();

            assert_eq!(variant.to_rust_string(0), "Carton(u64, u64),");
        }

        #[test]
        fn test_value_variant_2() {
            let variant = EnumVariantBuilder::new("Carton")
                .with_value("u64")
                .with_value("u64")
                .with_value("String")
                .build();

            assert_eq!(variant.to_rust_string(0), "Carton(u64, u64, String),");
        }

        #[test]
        fn test_struct_variant() {
            let variant = EnumVariantBuilder::new("Carton")
                .with_field("capacity", "u64")
                .with_field("brand", "String")
                .build();

            assert_eq!(
                variant.to_rust_string(0),
                "Carton {\n    capacity: u64,\n    brand: String,\n},"
            );
        }
    }

    mod test_enum {
        use super::*;

        #[test]
        fn test_empty_variant_enum() {
            let e = RustEnum::new("Animals")
                .with_variant(EnumVariant::new_empty("Cow"))
                .with_variant(EnumVariant::new_empty("Dog"));

            assert_eq!(
                e.to_rust_string(0),
                "enum Animals {\n    Cow,\n    Dog,\n}\n".to_string()
            );
        }

        #[test]
        fn test_value_variant_enum() {
            let e = RustEnum::new("Animals")
                .with_variant(EnumVariant::build("Cow").with_value("u64").build())
                .with_variant(
                    EnumVariant::build("Dog")
                        .with_value("u64")
                        .with_value("u64")
                        .build(),
                );

            assert_eq!(
                e.to_rust_string(0),
                "enum Animals {\n    Cow(u64),\n    Dog(u64, u64),\n}\n".to_string()
            );
        }

        #[test]
        fn test_struct_variant_enum() {
            let e = RustEnum::new("Animals")
                .with_variant(EnumVariant::build("Cow").with_field("age", "u64").build())
                .with_variant(
                    EnumVariant::build("Dog")
                        .with_field("age", "u64")
                        .with_field("weight", "u64")
                        .build(),
                );

            assert_eq!(
                e.to_rust_string(0),
                "enum Animals {\n    Cow {\n        age: u64,\n    },\n    Dog {\n        age: u64,\n        weight: u64,\n    },\n}\n".to_string()
            );
        }

        #[test]
        fn test_pub_enum() {
            let e = RustEnum::new("Animals")
                .with_visibility(Visibility::Public)
                .with_variant(EnumVariant::build("Cow").with_field("age", "u64").build())
                .with_variant(
                    EnumVariant::build("Dog")
                        .with_field("age", "u64")
                        .with_field("weight", "u64")
                        .build(),
                );

            assert_eq!(
                e.to_rust_string(0),
                "pub enum Animals {\n    Cow {\n        age: u64,\n    },\n    Dog {\n        age: u64,\n        weight: u64,\n    },\n}\n".to_string()
            );
        }

        #[test]
        fn test_template_enum() {
            let e = RustEnum::new("Animals")
                .with_visibility(Visibility::CrateVisible)
                .with_template("T")
                .with_variant(EnumVariant::build("Cow").with_field("age", "u64").build())
                .with_variant(
                    EnumVariant::build("Dog")
                        .with_field("age", "u64")
                        .with_field("weight", "u64")
                        .build(),
                );

            assert_eq!(
                e.to_rust_string(0),
                "pub(crate) enum Animals<T> {\n    Cow {\n        age: u64,\n    },\n    Dog {\n        age: u64,\n        weight: u64,\n    },\n}\n".to_string()
            );
        }
    }
}
