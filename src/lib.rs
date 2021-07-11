mod file;
pub mod rust_component;
mod rust_enum;
mod rust_impl;
mod rust_method;
mod rust_module;
mod rust_struct;
mod rust_text;
mod rust_variable;

pub use file::RustFile;
pub use rust_enum::{EnumVariant, EnumVariantBuilder, RustEnum};
pub use rust_impl::RustImplementation;
pub use rust_method::RustMethod;
pub use rust_module::RustModule;
pub use rust_struct::RustStruct;
pub use rust_text::RustText;
pub use rust_variable::RustVariable;

const TAB_SIZE: usize = 4;

#[cfg(feature = "indent_tabs")]
fn indent_string(indent_level: usize) -> String {
    return "\t".repeat(indent_level);
}

#[cfg(not(feature = "indent_tabs"))]
fn indent_string(indent_level: usize) -> String {
    return " ".repeat(TAB_SIZE).repeat(indent_level);
}
