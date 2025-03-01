use tree_sitter::{Parser, Tree};
use tree_sitter_solidity as sol;

/// Parses Solidity code into an AST
pub fn parse_solidity_code(code: &str) -> Tree {
    let mut parser = Parser::new();
    parser
        .set_language(&sol::LANGUAGE.into())
        .expect("Failed to load Solidity grammar");

    parser
        .parse(code, None)
        .expect("Failed to parse Solidity code")
}
