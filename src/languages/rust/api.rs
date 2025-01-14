use crate::error::LaibraryError;
use crate::types::Namespace;
use std::path::Path;
use tree_sitter::Parser;

use super::namespace_construction::construct_namespaces;
use super::symbol_collection::collect_symbols;
use super::symbol_resolution::resolve_symbols;

pub fn build_public_api(
    entry_point: &Path,
    crate_name: &str,
    parser: &mut Parser,
) -> Result<Vec<Namespace>, LaibraryError> {
    let raw_namespaces = collect_symbols(entry_point, parser)?;
    let resolution = resolve_symbols(&raw_namespaces)?;
    let namespaces = construct_namespaces(resolution, crate_name);
    Ok(namespaces)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::languages::rust::test_helpers::setup_parser;
    use crate::test_helpers::{create_file, create_temp_dir};

    const STUB_CRATE_NAME: &str = "test_crate";

    #[test]
    fn test_nonexistent_file() {
        let mut parser = setup_parser();
        let path = std::path::PathBuf::from("nonexistent.rs");

        let result = build_public_api(&path, STUB_CRATE_NAME, &mut parser);

        assert!(matches!(result, Err(LaibraryError::Parse(_))));
    }

    #[test]
    fn test_integration() {
        let temp_dir = create_temp_dir();
        let lib_rs = temp_dir.path().join("src").join("lib.rs");
        let module_rs = temp_dir.path().join("src").join("module.rs");

        create_file(
            &lib_rs,
            r#"
pub mod module;
pub use module::Format;

pub fn process(format: Format) -> String {
    "processed".to_string()
}
"#,
        );
        create_file(
            &module_rs,
            r#"
pub enum Format {
    Text,
    Binary,
}
"#,
        );

        let mut parser = setup_parser();
        let namespaces = build_public_api(&lib_rs, STUB_CRATE_NAME, &mut parser).unwrap();

        assert_eq!(namespaces.len(), 2);
        let root = namespaces
            .iter()
            .find(|n| n.name == STUB_CRATE_NAME)
            .unwrap();
        assert_eq!(root.symbols.len(), 2);
        assert!(root.symbols.iter().any(|s| s.name == "process"));
        assert!(root.symbols.iter().any(|s| s.name == "Format"));

        let module = namespaces
            .iter()
            .find(|n| n.name == format!("{}::module", STUB_CRATE_NAME))
            .unwrap();
        assert_eq!(module.symbols.len(), 1);
        assert!(module.symbols.iter().any(|s| s.name == "Format"));
    }
}
