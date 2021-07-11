use rmod_gen::rust_component::Field;
use rmod_gen::{RustFile, RustModule, RustStruct};

#[test]
fn test_single_file() {
    let my_file = RustFile::new()
        .with_import("use std::fmt")
        .with_component(
            RustStruct::new("Logger")
                .with_template("T")
                .with_extra("where T: Write")
                .with_field(Field::private("sink", "T"))
                .into(),
        )
        .with_component(RustModule::new("tests").with_cfg("#[cfg(test)]").into());

    let comp = "use std::fmt;\n\nstruct Logger<T> where T: Write {\n    sink: T,\n}\n\n#[cfg(test)]\nmod tests {\n}\n\n";

    assert_eq!(comp.to_string(), my_file.into_rust_code());
}
