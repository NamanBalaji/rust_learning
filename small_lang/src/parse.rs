use crate::expr::*;
use crate::tokenize::{Token, TokenType};

pub fn parse(mut tokens: Vec<Token>) -> Vec<Expr> {
    tokens.reverse();

    let mut result = Vec::new();

    while !tokens.is_empty() {
        let expr = parse_expr(&mut tokens);

        expect(TokenType::Newline, &mut tokens);

        result.push(expr);
    }

    result
}

fn parse_expr(tokens: &mut Vec<Token>) -> Expr {
    parse_assignment(tokens)
}

fn parse_assignment(tokens: &mut Vec<Token>) -> Expr {
    if tokens.len() > 1 && tokens[tokens.len() - 2].token_type == TokenType::Equal {
        let variable = parse_variable(tokens);
        expect(TokenType::Equal, tokens);
        let value = parse_expr(tokens);

        Expr::Assignment(AssignmentImpl {
            target: variable,
            value: Box::new(value),
        })
    } else {
        parse_term(tokens)
    }
}

fn parse_variable(tokens: &mut Vec<Token>) -> VariableImpl {
    let token = tokens.pop().unwrap();
    if token.token_type == TokenType::Identifier {
        VariableImpl { name: token }
    } else {
        panic!("Expected Identifier, found {:?}", token.token_type);
    }
}

fn parse_term(tokens: &mut Vec<Token>) -> Expr {
    let mut result = parse_factor(tokens);

    while !tokens.is_empty() {
        let next_token = &tokens[tokens.len() - 1];
        match next_token.token_type {
            TokenType::Plus | TokenType::Minus => {
                let op_token = tokens.pop().unwrap();
                let rhs = parse_factor(tokens);

                result = Expr::BinaryOperation(BinaryOpImpl {
                    lhs: Box::new(result),
                    operation: op_token,
                    rhs: Box::new(rhs),
                });
            }
            _ => break,
        }
    }

    result
}

fn parse_factor(tokens: &mut Vec<Token>) -> Expr {
    let mut result = parse_primary(tokens);

    while tokens.len() > 1 {
        let next_token = &tokens[tokens.len() - 1];
        match next_token.token_type {
            TokenType::Star | TokenType::Slash => {
                let op_token = tokens.pop().unwrap();
                let rhs = parse_primary(tokens);

                result = Expr::BinaryOperation(BinaryOpImpl {
                    lhs: Box::new(result),
                    operation: op_token,
                    rhs: Box::new(rhs),
                })
            }
            _ => break,
        }
    }

    result
}

fn parse_primary(tokens: &mut Vec<Token>) -> Expr {
    let token = tokens.pop().unwrap();

    match token.token_type {
        TokenType::NumberLiteral => Expr::Number(NumberImpl {
            value: parse_number(&token.lexeme),
        }),
        TokenType::Identifier => {
            if !tokens.is_empty() {
                let next_token = &tokens[tokens.len() - 1];
                if next_token.token_type == TokenType::LeftParen {
                    let fun_name = VariableImpl { name: token };

                    // Remove the LeftParen
                    tokens.pop().unwrap();

                    let arg = parse_expr(tokens);

                    expect(TokenType::RightParen, tokens);

                    return Expr::FunCall(FunCallImpl {
                        name: fun_name,
                        arg: Box::new(arg),
                    });
                }
            }
            Expr::Variable(VariableImpl { name: token })
        }
        TokenType::LeftParen => {
            let expr = parse_expr(tokens);
            expect(TokenType::RightParen, tokens);
            expr
        }
        t => panic!("Unexpected token type: {:?}", t),
    }
}

fn parse_number(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}

fn expect(expected: TokenType, tokens: &mut Vec<Token>) {
    match tokens.pop() {
        None => {}
        Some(token) => {
            if token.token_type != expected {
                panic!("Expected token {:?} got {:?}", expected, token.token_type);
            }
        }
    }
}
