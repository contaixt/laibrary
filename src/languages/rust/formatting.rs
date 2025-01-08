use crate::types::Namespace;

pub fn format_module(module: &Namespace) -> String {
    let module_doc = module
        .symbols
        .iter()
        .map(|symbol| symbol.source_code.as_str())
        .collect::<Vec<_>>()
        .join("\n\n");
    module_doc
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Symbol;

    #[test]
    fn test_format_module() {
        let module = Namespace {
            name: "test".to_string(),
            symbols: vec![
                Symbol {
                    name: "test".to_string(),
                    source_code: "pub fn test() -> () {}".to_string(),
                },
                Symbol {
                    name: "TestEnum".to_string(),
                    source_code: "pub enum TestEnum { A, B }".to_string(),
                },
            ],
        };

        let formatted = format_module(&module);
        assert_eq!(
            formatted, "pub fn test() -> () {}\n\npub enum TestEnum { A, B }",
            "Module should format all symbols with double newlines between them"
        );
    }
}
