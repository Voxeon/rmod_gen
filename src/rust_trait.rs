use crate::rust_component::{RustComponent, RustComponentTrait, RustTemplateUsage, Visibility};

/// Represents a trait in Rust.
///
/// # Example
/// ```
///
/// // Creates the following trait
/// /*
/// pub trait Explosive<'a, T>: std::fmt::Debug {
///     fn my_method();
/// }
/// */
///
/// use rmod_gen::{RustTrait, RustMethod};
/// use rmod_gen::rust_component::{Visibility, RustComponentTrait};
/// let r_trait = RustTrait::new("Explosive")
///                 .with_visibility(Visibility::Public)
///                 .with_bound("std::fmt::Debug")
///                 .with_lifetime("a")
///                 .with_template("T")
///                 .with_component(
///                         RustMethod::new("my_method")
///                         .without_body()
///                         .into()
///                 );
///
/// assert_eq!(r_trait.to_rust_string(0), "pub trait Explosive<'a, T>: std::fmt::Debug {\n    fn my_method();\n\n}\n")
/// ```
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct RustTrait {
    name: String,
    visibility: Visibility,
    bounds: Vec<String>,
    components: Vec<RustComponent>,
    lifetimes: Vec<String>,
    templates: Vec<String>,
    cfg: String,
    extra: String,
}

impl RustTrait {
    /// Creates a new empty instance.
    pub fn new(name: &str) -> Self {
        return Self {
            name: name.to_string(),
            visibility: Visibility::Private,
            bounds: Vec::new(),
            components: Vec::new(),
            lifetimes: Vec::new(),
            templates: Vec::new(),
            cfg: String::new(),
            extra: String::new(),
        };
    }

    /// Sets the visibility for this trait.
    pub fn with_visibility(mut self, visibility: Visibility) -> Self {
        self.set_visibility(visibility);

        return self;
    }

    /// Appends a trait bound.
    ///
    /// ```
    /// use rmod_gen::RustTrait;
    /// use rmod_gen::rust_component::RustComponentTrait;
    ///
    /// let rust_trait = RustTrait::new("MyTrait").with_bound("Debug").with_bound("Display");
    ///
    /// assert_eq!(rust_trait.to_rust_string(0), "trait MyTrait: Debug + Display {\n}\n");
    /// ```
    pub fn with_bound(mut self, bound: &str) -> Self {
        self.push_bound(bound);

        return self;
    }

    /// Appends a component to the trait.
    ///
    /// <b>NOTE:</b> Whilst this method accepts any RustComponent the ownership is on the user of this
    /// method to ensure that only components that are valid in this situation are added,
    /// e.g. methods or variables but not structs.
    pub fn with_component(mut self, component: RustComponent) -> Self {
        self.push_component(component);

        return self;
    }

    /// Appends a lifetime. The lifetime should be only the identifier. i.e. to create a lifetime " 'a "
    ///
    /// ```
    /// use rmod_gen::RustTrait;
    /// use rmod_gen::rust_component::RustComponentTrait;
    ///
    /// let rust_trait = RustTrait::new("MyTrait").with_lifetime("a").with_lifetime("b");
    ///
    /// assert_eq!(rust_trait.to_rust_string(0), "trait MyTrait<'a, 'b> {\n}\n");
    /// ```
    pub fn with_lifetime(mut self, lifetime: &str) -> Self {
        self.push_lifetime(lifetime);

        return self;
    }

    /// Appends a template.
    pub fn with_template(mut self, template: &str) -> Self {
        self.push_template(template);

        return self;
    }

    /// Sets some information that should go before the method.
    ///
    /// ```
    /// use rmod_gen::RustTrait;
    /// use rmod_gen::rust_component::RustComponentTrait;
    ///
    /// let rust_trait = RustTrait::new("MyTrait").with_cfg("#[my_cfg]");
    ///
    /// assert_eq!(rust_trait.to_rust_string(0), "#[my_cfg]\ntrait MyTrait {\n}\n");
    /// ```
    pub fn with_cfg(mut self, cfg: &str) -> Self {
        self.set_cfg(cfg);

        return self;
    }

    /// Extra information that is inserted right before the opening curly brace.
    ///
    /// ```
    /// use rmod_gen::RustTrait;
    /// use rmod_gen::rust_component::RustComponentTrait;
    ///
    /// let rust_trait = RustTrait::new("MyTrait").with_extra("where T: Debug");
    ///
    /// assert_eq!(rust_trait.to_rust_string(0), "trait MyTrait where T: Debug {\n}\n");
    /// ```
    pub fn with_extra(mut self, extra: &str) -> Self {
        self.set_extra(extra);

        return self;
    }

    /// Sets the visibility for this trait.
    pub fn set_visibility(&mut self, visibility: Visibility) {
        self.visibility = visibility;
    }

    /// Appends a trait bound.
    ///
    /// ```
    /// use rmod_gen::RustTrait;
    /// use rmod_gen::rust_component::RustComponentTrait;
    ///
    /// let mut rust_trait = RustTrait::new("MyTrait");
    /// rust_trait.push_bound("Debug");
    /// rust_trait.push_bound("Display");
    ///
    /// assert_eq!(rust_trait.to_rust_string(0), "trait MyTrait: Debug + Display {\n}\n");
    /// ```
    pub fn push_bound(&mut self, bound: &str) {
        self.bounds.push(bound.to_string());
    }

    /// Appends a component to the trait.
    ///
    /// <b>NOTE:</b> Whilst this method accepts any RustComponent the ownership is on the user of this
    /// method to ensure that only components that are valid in this situation are added,
    /// e.g. methods or variables but not structs.
    pub fn push_component(&mut self, component: RustComponent) {
        self.components.push(component);
    }

    /// Appends a lifetime. The lifetime should be only the identifier. i.e. to create a lifetime " 'a "
    ///
    /// ```
    /// use rmod_gen::RustTrait;
    /// use rmod_gen::rust_component::RustComponentTrait;
    ///
    /// let mut rust_trait = RustTrait::new("MyTrait");
    /// rust_trait.push_lifetime("a");
    /// rust_trait.push_lifetime("b");
    ///
    /// assert_eq!(rust_trait.to_rust_string(0), "trait MyTrait<'a, 'b> {\n}\n");
    /// ```
    pub fn push_lifetime(&mut self, lifetime: &str) {
        self.lifetimes.push(lifetime.to_string());
    }

    /// Appends a template.
    pub fn push_template(&mut self, template: &str) {
        self.templates.push(template.to_string());
    }

    /// Sets some information that should go before the method.
    ///
    /// ```
    /// use rmod_gen::RustTrait;
    /// use rmod_gen::rust_component::RustComponentTrait;
    ///
    /// let mut rust_trait = RustTrait::new("MyTrait");
    /// rust_trait.set_cfg("#[my_cfg]");
    ///
    /// assert_eq!(rust_trait.to_rust_string(0), "#[my_cfg]\ntrait MyTrait {\n}\n");
    /// ```
    pub fn set_cfg(&mut self, cfg: &str) {
        self.cfg = cfg.to_string();
    }

    /// Extra information that is inserted right before the opening curly brace.
    ///
    /// ```
    /// use rmod_gen::RustTrait;
    /// use rmod_gen::rust_component::RustComponentTrait;
    ///
    /// let mut rust_trait = RustTrait::new("MyTrait");
    /// rust_trait.set_extra("where T: Debug");
    ///
    /// assert_eq!(rust_trait.to_rust_string(0), "trait MyTrait where T: Debug {\n}\n");
    /// ```
    pub fn set_extra(&mut self, extra: &str) {
        self.extra = extra.to_string();
    }
}

impl Into<RustComponent> for RustTrait {
    fn into(self) -> RustComponent {
        return RustComponent::Trait(self);
    }
}

impl RustTemplateUsage for RustTrait {}

impl RustComponentTrait for RustTrait {
    fn to_rust_string(&self, indent_level: usize) -> String {
        let base_indent_string = crate::indent_string(indent_level);
        let mut components = vec![base_indent_string.clone()];

        if !self.cfg.is_empty() {
            components.push(format!("{}\n", self.cfg));
            components.push(base_indent_string.clone());
        }

        if self.visibility != Visibility::Private {
            components.push(format!("{} ", self.visibility));
        }

        components.push(format!("trait {}", self.name));
        components.push(Self::create_template_string(&self.templates, &self.lifetimes));

        if !self.bounds.is_empty() {
            components.push(format!(": {}", self.bounds.join(" + ")));
        }

        if !self.extra.is_empty() {
            components.push(format!(" {}", self.extra));
        }

        components.push(" {\n".to_string());

        for comp in &self.components {
            components.push(comp.to_rust_string(indent_level + 1));
            components.push("\n".to_string());
        }

        components.push(format!("{}}}\n", base_indent_string));

        return components.join("");
    }
}
