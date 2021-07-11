use crate::rust_component::{RustComponent, RustComponentTrait, RustTemplateUsage, Visibility};

// TODO: Add extra and add cfg options.

/// Represents a function or method in Rust.
///
/// # Example
/// ```
/// use rmod_gen::RustMethod;
/// use rmod_gen::rust_component::{Visibility, RustComponentTrait};
///
/// // Creates the following method:
/// /*
/// pub unsafe fn create_cow<'a, T>(name: &str, age: u64) -> Cow {
///     let cow = Cow::new();
///
///     return cow;
/// }
/// */
///
/// let method = RustMethod::new("create_cow")
///             .with_lifetime("a")
///             .with_template("T")
///             .with_body("let cow = Cow::new();\nreturn cow;\n")
///             .with_fn_type("unsafe")
///             .with_visibility(Visibility::Public)
///             .with_return_type("Cow")
///             .with_argument("name: &str")
///             .with_argument("age: u64");
///
/// assert_eq!(method.to_rust_string(0), "pub unsafe fn create_cow<'a, T>(name: &str, age: u64) -> Cow {\n    let cow = Cow::new();\n    return cow;\n}\n");
///```
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct RustMethod {
    name: String,
    fn_type: String,
    visibility: Visibility,
    arguments: Vec<String>,
    return_type: String,
    body: String,
    templates: Vec<String>,
    lifetimes: Vec<String>,
}

impl RustMethod {
    /// Creates a new instance.
    pub fn new(name: &str) -> Self {
        return Self {
            name: name.to_string(),
            fn_type: String::new(),
            visibility: Visibility::Private,
            arguments: Vec::new(),
            return_type: String::new(),
            body: String::new(),
            templates: Vec::new(),
            lifetimes: Vec::new(),
        };
    }

    /// Set the function type, for example 'unsafe' or 'const'
    ///
    /// ```
    /// use rmod_gen::RustMethod;
    /// use rmod_gen::rust_component::RustComponentTrait;
    ///
    /// let method = RustMethod::new("my_method").with_fn_type("unsafe").to_rust_string(0);
    ///
    /// assert_eq!(method, "unsafe fn my_method() {\n}\n");
    /// ```
    pub fn with_fn_type(mut self, tp: &str) -> Self {
        self.set_fn_type(tp);

        return self;
    }

    /// Sets the visibility for this method.
    pub fn with_visibility(mut self, visibility: Visibility) -> Self {
        self.set_visibility(visibility);

        return self;
    }

    /// Appends an argument for this method.
    pub fn with_argument(mut self, arg: &str) -> Self {
        self.push_argument(arg);

        return self;
    }

    /// Sets the return type.
    pub fn with_return_type(mut self, return_type: &str) -> Self {
        self.set_return_type(return_type);

        return self;
    }

    /// Sets the body of the method. When the code is generated the lines will be split and an
    /// appropriate indent added.
    pub fn with_body(mut self, body: &str) -> Self {
        self.set_body(body);

        return self;
    }

    /// Appends a template.
    pub fn with_template(mut self, template: &str) -> Self {
        self.push_template(template);

        return self;
    }

    /// Appends a lifetime. The lifetime should be only the identifier. i.e. to create a lifetime " 'a "
    ///
    /// ```
    /// use rmod_gen::RustMethod;
    ///
    /// // fn method_name<'a>()
    /// let rust_enum = RustMethod::new("method_name").with_lifetime("a"); // Creates new lifetime 'a
    /// ```
    pub fn with_lifetime(mut self, lifetime: &str) -> Self {
        self.push_lifetime(lifetime);

        return self;
    }

    /// Set the function type, for example 'unsafe' or 'const'
    ///
    /// ```
    /// use rmod_gen::RustMethod;
    /// use rmod_gen::rust_component::RustComponentTrait;
    ///
    /// let mut method = RustMethod::new("my_method");
    /// method.set_fn_type("unsafe");
    /// let method = method.to_rust_string(0);
    ///
    /// assert_eq!(method, "unsafe fn my_method() {\n}\n");
    /// ```
    pub fn set_fn_type(&mut self, tp: &str) {
        self.fn_type = tp.to_string();
    }

    /// Sets the visibility for this method.
    pub fn set_visibility(&mut self, visibility: Visibility) {
        self.visibility = visibility;
    }

    /// Appends an argument for this method.
    pub fn push_argument(&mut self, argument: &str) {
        self.arguments.push(argument.to_string());
    }

    /// Sets the return type.
    pub fn set_return_type(&mut self, return_type: &str) {
        self.return_type = return_type.to_string();
    }

    /// Sets the body of the method. When the code is generated the lines will be split and an
    /// appropriate indent added.
    pub fn set_body(&mut self, body: &str) {
        self.body = body.to_string();
    }

    /// Appends a template.
    pub fn push_template(&mut self, template: &str) {
        self.templates.push(template.to_string());
    }

    /// Appends a lifetime. The lifetime should be only the identifier. i.e. to create a lifetime " 'a "
    ///
    /// ```
    /// use rmod_gen::RustMethod;
    ///
    /// // fn method_name<'a>()
    /// let mut rust_enum = RustMethod::new("method_name");
    /// rust_enum.with_lifetime("a"); // Creates new lifetime 'a
    /// ```
    pub fn push_lifetime(&mut self, lifetime: &str) {
        self.lifetimes.push(lifetime.to_string());
    }
}

impl Into<RustComponent> for RustMethod {
    fn into(self) -> RustComponent {
        return RustComponent::Method(self);
    }
}

impl RustTemplateUsage for RustMethod {}

impl RustComponentTrait for RustMethod {
    fn to_rust_string(&self, indent_level: usize) -> String {
        let base_indent_string = crate::indent_string(indent_level);
        let next_level_indent_string = crate::indent_string(indent_level + 1);

        let mut components = vec![base_indent_string.clone()];

        if self.visibility != Visibility::Private {
            components.push(self.visibility.to_string());
            components.push(" ".to_string());
        }

        if !self.fn_type.is_empty() {
            components.push(self.fn_type.clone());
            components.push(" ".to_string());
        }

        components.push("fn ".to_string());

        components.push(self.name.clone());

        let templates_string = Self::create_template_string(&self.templates, &self.lifetimes);

        if !templates_string.is_empty() {
            components.push(templates_string);
        }

        components.push("(".to_string());
        components.push(self.arguments.join(", "));
        components.push(") ".to_string());

        if !self.return_type.is_empty() {
            components.push("-> ".to_string());
            components.push(self.return_type.clone());
            components.push(" ".to_string());
        }

        components.push("{\n".to_string());

        for line in self.body.lines() {
            components.push(next_level_indent_string.clone());
            components.push(line.to_string());
            components.push("\n".to_string());
        }

        components.push(base_indent_string);
        components.push("}\n".to_string());

        return components.join("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_method() {
        let method = RustMethod::new("create_cow");

        assert_eq!(method.to_rust_string(0), "fn create_cow() {\n}\n");
    }

    #[test]
    fn test_basic_method_return() {
        let method = RustMethod::new("create_cow").with_return_type("Cow");

        assert_eq!(method.to_rust_string(0), "fn create_cow() -> Cow {\n}\n");
    }

    #[test]
    fn test_basic_method_args() {
        let method = RustMethod::new("create_cow")
            .with_return_type("Cow")
            .with_argument("name: &str")
            .with_argument("age: u64");

        assert_eq!(
            method.to_rust_string(0),
            "fn create_cow(name: &str, age: u64) -> Cow {\n}\n"
        );
    }

    #[test]
    fn test_basic_public_method() {
        let method = RustMethod::new("create_cow")
            .with_visibility(Visibility::Public)
            .with_return_type("Cow")
            .with_argument("name: &str")
            .with_argument("age: u64");

        assert_eq!(
            method.to_rust_string(0),
            "pub fn create_cow(name: &str, age: u64) -> Cow {\n}\n"
        );
    }

    #[test]
    fn test_basic_unsafe_method() {
        let method = RustMethod::new("create_cow")
            .with_fn_type("unsafe")
            .with_visibility(Visibility::Public)
            .with_return_type("Cow")
            .with_argument("name: &str")
            .with_argument("age: u64");

        assert_eq!(
            method.to_rust_string(0),
            "pub unsafe fn create_cow(name: &str, age: u64) -> Cow {\n}\n"
        );
    }

    #[test]
    fn test_basic_unsafe_method_with_body() {
        let method = RustMethod::new("create_cow")
            .with_body("let cow = Cow::new();\nreturn cow;\n")
            .with_fn_type("unsafe")
            .with_visibility(Visibility::Public)
            .with_return_type("Cow")
            .with_argument("name: &str")
            .with_argument("age: u64");

        assert_eq!(method.to_rust_string(0), "pub unsafe fn create_cow(name: &str, age: u64) -> Cow {\n    let cow = Cow::new();\n    return cow;\n}\n");
    }

    #[test]
    fn test_basic_unsafe_method_with_templates() {
        let method = RustMethod::new("create_cow")
            .with_lifetime("a")
            .with_template("T")
            .with_body("let cow = Cow::new();\nreturn cow;\n")
            .with_fn_type("unsafe")
            .with_visibility(Visibility::Public)
            .with_return_type("Cow")
            .with_argument("name: &str")
            .with_argument("age: u64");

        assert_eq!(method.to_rust_string(0), "pub unsafe fn create_cow<'a, T>(name: &str, age: u64) -> Cow {\n    let cow = Cow::new();\n    return cow;\n}\n");
    }
}
