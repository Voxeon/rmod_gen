use crate::rust_component::{RustComponent, RustComponentTrait, Visibility};

/// Represents a variable in Rust.
///
/// # Example
/// ```
/// use rmod_gen::RustVariable;
/// use rmod_gen::rust_component::RustComponentTrait;
///
/// // Create the following variable:
/// // let var: &str = "carton";
///
///
/// let variable = RustVariable::new_let("var").with_value("\"carton\"").with_type("&str").to_rust_string(0);
/// assert_eq!(variable, "let var: &str = \"carton\";");
/// ```
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct RustVariable {
    visibility: Visibility,
    name: String,
    value: String,
    tp: String,
    variable_type: VariableType,
    is_mut: bool,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq)]
enum VariableType {
    Static,
    Const,
    Regular,
}

impl RustVariable {
    /// Create a new let variable.
    pub fn new_let(name: &str) -> Self {
        return Self {
            visibility: Visibility::Private,
            name: name.to_string(),
            value: String::new(),
            tp: String::new(),
            variable_type: VariableType::Regular,
            is_mut: false,
        };
    }

    /// Create a new constant variable.
    pub fn new_const(name: &str) -> Self {
        return Self {
            visibility: Visibility::Private,
            name: name.to_string(),
            value: String::new(),
            tp: String::new(),
            variable_type: VariableType::Const,
            is_mut: false,
        };
    }

    /// Create a new static variable.
    pub fn new_static(name: &str) -> Self {
        return Self {
            visibility: Visibility::Private,
            name: name.to_string(),
            value: String::new(),
            tp: String::new(),
            variable_type: VariableType::Static,
            is_mut: false,
        };
    }

    /// Sets the value for the variable.
    ///
    /// ```
    /// use rmod_gen::RustVariable;
    /// use rmod_gen::rust_component::RustComponentTrait;
    ///
    /// let v = RustVariable::new_let("people").with_value("5");
    ///
    /// assert_eq!(v.to_rust_string(0), "let people = 5;")
    /// ```
    pub fn with_value(mut self, value: &str) -> Self {
        self.set_value(value);

        return self;
    }

    /// Sets an explicit type for the variable.
    ///
    /// ```
    /// use rmod_gen::RustVariable;
    /// use rmod_gen::rust_component::RustComponentTrait;
    ///
    /// let v = RustVariable::new_let("people").with_type("u64");
    ///
    /// assert_eq!(v.to_rust_string(0), "let people: u64;")
    /// ```
    pub fn with_type(mut self, tp: &str) -> Self {
        self.set_type(tp);

        return self;
    }

    /// Marks the variable as mutable
    ///
    /// ```
    /// use rmod_gen::RustVariable;
    /// use rmod_gen::rust_component::RustComponentTrait;
    ///
    /// let v = RustVariable::new_let("people").with_mut();
    ///
    /// assert_eq!(v.to_rust_string(0), "let mut people;")
    /// ```
    pub fn with_mut(mut self) -> Self {
        self.set_mut(true);

        return self;
    }

    /// Set the variable's visibility
    ///
    /// ```
    /// use rmod_gen::RustVariable;
    /// use rmod_gen::rust_component::{RustComponentTrait, Visibility};
    ///
    /// let v = RustVariable::new_const("people").with_visibility(Visibility::Public);
    ///
    /// assert_eq!(v.to_rust_string(0), "pub const people;")
    /// ```
    pub fn with_visibility(mut self, visibility: Visibility) -> Self {
        self.set_visibility(visibility);

        return self;
    }

    /// Sets the value for the variable.
    ///
    /// ```
    /// use rmod_gen::RustVariable;
    /// use rmod_gen::rust_component::RustComponentTrait;
    ///
    /// let mut v = RustVariable::new_let("people");
    /// v.set_value("5");
    ///
    /// assert_eq!(v.to_rust_string(0), "let people = 5;")
    /// ```
    pub fn set_value(&mut self, value: &str) {
        self.value = value.to_string();
    }

    /// Sets an explicit type for the variable.
    ///
    /// ```
    /// use rmod_gen::RustVariable;
    /// use rmod_gen::rust_component::RustComponentTrait;
    ///
    /// let mut v = RustVariable::new_let("people");
    /// v.set_type("u64");
    ///
    /// assert_eq!(v.to_rust_string(0), "let people: u64;")
    /// ```
    pub fn set_type(&mut self, tp: &str) {
        self.tp = tp.to_string();
    }

    /// Marks the variable as mutable or immutable
    ///
    /// ```
    /// use rmod_gen::RustVariable;
    /// use rmod_gen::rust_component::RustComponentTrait;
    ///
    /// let mut v = RustVariable::new_let("people");
    /// v.set_mut(true);
    ///
    /// assert_eq!(v.to_rust_string(0), "let mut people;")
    /// ```
    pub fn set_mut(&mut self, mutable: bool) {
        self.is_mut = mutable;
    }

    /// Set the variable's visibility
    ///
    /// ```
    /// use rmod_gen::RustVariable;
    /// use rmod_gen::rust_component::{RustComponentTrait, Visibility};
    ///
    /// let mut v = RustVariable::new_const("people");
    /// v.set_visibility(Visibility::Public);
    ///
    /// assert_eq!(v.to_rust_string(0), "pub const people;")
    /// ```
    pub fn set_visibility(&mut self, visibility: Visibility) {
        self.visibility = visibility;
    }
}

impl Into<RustComponent> for RustVariable {
    fn into(self) -> RustComponent {
        return RustComponent::Variable(self);
    }
}

impl RustComponentTrait for RustVariable {
    fn to_rust_string(&self, indent_level: usize) -> String {
        let mut components = vec![crate::indent_string(indent_level)];

        if self.visibility != Visibility::Private {
            components.push(self.visibility.to_string());
            components.push(String::from(' '));
        }

        components.push(match self.variable_type {
            VariableType::Static => String::from("static "),
            VariableType::Const => String::from("const "),
            VariableType::Regular => String::from("let "),
        });

        if self.is_mut {
            components.push(String::from("mut "));
        }

        components.push(self.name.clone());

        if !self.tp.is_empty() {
            components.push(String::from(": "));
            components.push(self.tp.clone());
        }

        if !self.value.is_empty() {
            components.push(String::from(" = "));
            components.push(self.value.clone());
        }

        components.push(String::from(";"));

        return components.join("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_basic_variable() {
        let variable = RustVariable::new_let("people");

        assert_eq!(variable.to_rust_string(0), "let people;");
    }

    #[test]
    pub fn test_basic_variable_value() {
        let variable = RustVariable::new_let("people").with_value("52");

        assert_eq!(variable.to_rust_string(0), "let people = 52;");
    }

    #[test]
    pub fn test_basic_variable_type() {
        let variable = RustVariable::new_let("people")
            .with_value("52")
            .with_type("u64");

        assert_eq!(variable.to_rust_string(0), "let people: u64 = 52;");
    }

    #[test]
    pub fn test_static_variable() {
        let variable = RustVariable::new_static("people");

        assert_eq!(variable.to_rust_string(0), "static people;");
    }

    #[test]
    pub fn test_const_variable() {
        let variable = RustVariable::new_const("people");

        assert_eq!(variable.to_rust_string(0), "const people;");
    }

    #[test]
    pub fn test_variable_public_visibility() {
        let variable = RustVariable::new_const("people").with_visibility(Visibility::Public);

        assert_eq!(variable.to_rust_string(0), "pub const people;");
    }

    #[test]
    pub fn test_variable_crate_visibility() {
        let variable = RustVariable::new_const("people").with_visibility(Visibility::CrateVisible);

        assert_eq!(variable.to_rust_string(0), "pub(crate) const people;");
    }
}
