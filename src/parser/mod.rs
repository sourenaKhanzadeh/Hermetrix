// Declare submodules
pub mod ast;
pub mod solidity;

// Re-export useful functions
pub use ast::{ASTNode, SolidityAST};
pub use solidity::parse_solidity_code;
