mod expr;
mod parse;
mod tokenize;

use expr::{AssignmentImpl, BinaryOpImpl, Expr, FunCallImpl, NumberImpl, VariableImpl};
use parse::parse;
use std::collections::HashMap;
use tokenize::{TokenType::*, tokenize};

type Env = HashMap<String, i32>;

fn interpret(exprs: &Vec<Expr>, env: &mut Env) {
    for expr in exprs {
        evaluate(expr, env);
    }
}

fn evaluate(expr: &Expr, env: &mut Env) -> i32 {
    match expr {
        Expr::Assignment(AssignmentImpl { target, value }) => {
            let value = evaluate(value, env);

            env.insert(target.name.lexeme.clone(), value);

            value
        }
        Expr::BinaryOperation(BinaryOpImpl {
            lhs,
            operation,
            rhs,
        }) => {
            let lhs_value = evaluate(lhs, env);
            let rhs_value = evaluate(rhs, env);

            match &operation.token_type {
                Plus => lhs_value + rhs_value,
                Minus => lhs_value - rhs_value,
                Star => lhs_value * rhs_value,
                Slash => lhs_value / rhs_value,
                t => panic!("Invalid binary operation: {:?}", t),
            }
        }
        Expr::FunCall(FunCallImpl { name, arg }) => {
            if name.name.lexeme == "print" {
                let value = evaluate(arg, env);

                println!("{}", value);

                value
            } else {
                panic!("Undefined function {}", name.name.lexeme);
            }
        }
        Expr::Number(NumberImpl { value, .. }) => *value,
        Expr::Variable(VariableImpl { name }) => {
            if let Some(value) = env.get(&name.lexeme) {
                *value
            } else {
                panic!("Variable {} isn't defined", name.lexeme);
            }
        }
    }
}

fn main() {
    let src = "a = b = 2 * 3
               c = print(b + 3)
               print(c / a)";
    let tokens = tokenize(src);
    for token in tokens.iter() {
        println!("{:?}", token);
    }

    let exprs = parse(tokens);

    let mut env: Env = HashMap::new();

    interpret(&exprs, &mut env);
}
