use parser::{BinaryOperator, UnaryOperator, AST};

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
        &AST::UnaryOp(ref operator, ref operand) => {
            let mut lines = generate_expr(operand);
            lines.append(&mut generate_unary_op(operator));
            lines
        }
        &AST::BinaryOp(ref operator, ref expr1, ref expr2) => {
            let mut lines = generate_expr(expr1);
            lines.push(indent("pushq %rax"));
            lines.append(&mut generate_expr(expr2));
            lines.push(indent("popq %rcx"));
            lines.append(&mut generate_binary_op(operator));
            lines
        }
        _ => Vec::new(),
    }
}

fn generate_unary_op(operator: &UnaryOperator) -> Vec<String> {
    match *operator {
        UnaryOperator::Minus => vec![indent("neg %eax")],
        UnaryOperator::Tilde => vec![indent("not %eax")],
        UnaryOperator::Bang => vec![
            indent("cmpl $0, %eax"),
            indent("movl $0, %eax"),
            indent("sete %al"),
        ],
    }
}

fn generate_binary_op(operator: &BinaryOperator) -> Vec<String> {
    match *operator {
        BinaryOperator::Plus => vec![indent("addl %ecx, %eax")],
        BinaryOperator::Minus => vec![
            indent("subl %eax, %ecx"),
            // subl will store the result in %ecx, but we need it in %eax.
            indent("movl %ecx, %eax"),
        ],
        BinaryOperator::Times => vec![indent("imul %ecx, %eax")],
        BinaryOperator::Divide => vec![
            // Zero out %edx, as idivl dst computes [%edx:%eax] / dst.
            indent("movl $0, %edx"),
            // Store expr1 in %eax, expr2 in %ebx.
            indent("movl %eax, %ebx"),
            indent("movl %ecx, %eax"),
            // The quotient of idivl is written to %eax.
            indent("idivl %ebx"),
        ],
        _ => Vec::new(),
    }
}

fn indent(line: &str) -> String {
    "\t".to_owned() + line
}
