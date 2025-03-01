mod parser;

fn main() {
    let solidity_code = r#"
    contract Test {
        function add(uint a, uint b) public pure returns (uint) {
            return a + b;
        }
    }
    "#;

    let ast = parser::SolidityAST::new(solidity_code);
    println!("Parsed Solidity AST:");
    ast.print_ast(0);
}
