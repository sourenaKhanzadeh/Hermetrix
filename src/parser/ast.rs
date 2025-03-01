use tree_sitter::Node;

/// Represents a Solidity AST Node
#[derive(Debug, Clone)]
pub struct ASTNode {
    pub kind: String,
    pub start_byte: usize,
    pub end_byte: usize,
    pub children: Vec<ASTNode>,
}

impl ASTNode {
    /// Converts a Tree-Sitter AST Node into our Rust representation
    pub fn from_tree_sitter(node: Node) -> Self {
        let mut children = Vec::new();
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                children.push(ASTNode::from_tree_sitter(child));
            }
        }

        ASTNode {
            kind: node.kind().to_string(),
            start_byte: node.start_byte(),
            end_byte: node.end_byte(),
            children,
        }
    }
}

/// Represents the full Solidity AST
#[derive(Debug)]
pub struct SolidityAST {
    pub root: ASTNode,
}

impl SolidityAST {
    /// Parses Solidity code and builds an AST structure
    pub fn new(source_code: &str) -> Self {
        let tree = crate::parser::parse_solidity_code(source_code);
        let root = ASTNode::from_tree_sitter(tree.root_node());

        SolidityAST { root }
    }

    /// Pretty-prints the AST
    pub fn print_ast(&self, depth: usize) {
        self.print_node(&self.root, depth);
    }

    fn print_node(&self, node: &ASTNode, depth: usize) {
        println!("{}- {}", " ".repeat(depth * 2), node.kind);
        for child in &node.children {
            self.print_node(child, depth + 1);
        }
    }
}
