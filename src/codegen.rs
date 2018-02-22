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
            let mut lines = vec![format!("\t.globl _{}", name), format!("_{}:", name)];
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
            lines.push(format!("\tret"));
            lines
        }
        _ => Vec::new(),
    }
}

fn generate_expr(expr: &AST) -> Vec<String> {
    match expr {
        &AST::IntConstant(n) => vec![format!("\tmovl ${}, %eax", n)],
        _ => Vec::new(),
    }
}
