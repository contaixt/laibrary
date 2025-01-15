mod analysers;
pub mod error;
mod formatting;
mod languages;
mod parsing;
mod types;

#[cfg(test)]
mod test_helpers;
#[cfg(test)]
mod treesitter_test_helpers;

use crate::error::LaibraryError;
use crate::formatting::format_library_context;
use crate::languages::get_analyser;
use crate::parsing::get_parser;
use std::path::Path;

/// Generate API documentation for a library in the specified language.
///
/// # Arguments
///
/// * `language` - The programming language of the library
/// * `path` - Path to the library's root directory
///
/// # Returns
///
/// Returns a Result containing the generated documentation as a string, or an error if something went wrong.
pub fn generate_documentation(language: &str, path: &Path) -> Result<String, LaibraryError> {
    let analyser = get_analyser(language)?;

    let metadata = analyser.get_package_metadata(path)?;
    let mut parser = get_parser(&analyser.get_parser_language())?;
    let namespaces = analyser.extract_public_api(&metadata, &mut parser)?;

    format_library_context(&metadata, &namespaces, language)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_unsupported_language() {
        let result = generate_documentation("unsupported", &PathBuf::new());
        assert!(matches!(result, Err(LaibraryError::UnsupportedLanguage(_))));
    }
}
