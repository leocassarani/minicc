use parser::AST;

pub fn generate(ast: AST) -> Vec<String> {
    generate_program(&ast)
}

fn generate_program(ast: &AST) -> Vec<String> {
    match ast {
        &AST::Program(ref func) => generate_function(&func),
        _ => Vec::new(),
    }
}

fn generate_function(func: &AST) -> Vec<String> {
    match func {
        &AST::Function(ref name, ref body) => {
            let label = format!("_{}", name);
            let mut lines = vec![indent(&format!(".globl {}", label)), format!("{}:", label)];
            lines.append(&mut generate_statement(&body));
            lines
        }
        _ => Vec::new(),
    }
}

fn generate_statement(stmt: &AST) -> Vec<String> {
    match stmt {
        &AST::Return(ref expr) => {
            let mut lines = generate_expr(&expr);
            lines.push(indent("ret"));
            lines
        }
        _ => Vec::new(),
    }
}

fn generate_expr(expr: &AST) -> Vec<String> {
    match expr {
        &AST::IntConstant(n) => vec![indent(&format!("movl ${}, %eax", n))],
        _ => Vec::new(),
    }
}

fn indent(line: &str) -> String {
    "\t".to_owned() + line
}
