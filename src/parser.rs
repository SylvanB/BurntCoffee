use crate::tokeniser::{TokenKind, TokenOpcode};

#[derive(Debug, Copy, Clone)]
pub(crate) enum Operator {
    Multiply,
    Divide,
    Add,
    Subtract
}

#[derive(Debug)]
pub(crate) enum Expression {
    NumberLiteral(f32),
    BinaryExpression(Box<Expression>, Operator, Box<Expression>)
}

pub(crate) struct Parser {
    tokens: Vec<TokenKind>,
    idx: usize,
}

impl Parser {

    pub fn new() -> Self {
        Parser {
            tokens: vec![],
            idx: 0,
        }
    }

    pub fn with_tokens(mut self, tokens: Vec<TokenKind>) -> Self {
        self.tokens = tokens;
        self
    }

    pub fn parse(&mut self) -> Vec<Expression> {
        if self.tokens.len() == 0 {
            return vec![];
        }
        
        let expr = match self.tokens[self.idx] {
            TokenKind::Number(_) => self.parse_expression(),
            _ => return vec![],
        };

        vec![expr]
    }

    fn get_tok_at(&self, count: usize) -> Option<TokenKind> {
        if self.idx + 1 >= self.tokens.len() {
            return None
        }

        Some(self.tokens[self.idx + count])
    }
    
    fn get_curr_tok(&self) -> TokenKind {
        self.tokens[self.idx]
    }

    fn get_next_tok(&mut self) -> Option<TokenKind> {
        if self.idx + 1 > self.tokens.len() {
            return None
        }

        self.idx += 1;
        Some(self.tokens[self.idx])
    }

    fn parse_op(&self, op: TokenOpcode) -> Operator {
        match op {
            TokenOpcode::Multiply => Operator::Multiply,
            TokenOpcode::Add => Operator::Add,
            TokenOpcode::Subtract => Operator::Subtract,
            TokenOpcode::Divide => Operator::Divide,
        }
    }

    fn parse_expression(&mut self) -> Expression 
    {
        match self.get_curr_tok() {
            TokenKind::Operator(_) => panic!("Unexpected Token at token index {}: {:#?}", self.idx, self.get_curr_tok()),
            TokenKind::Number(n) => {
                match self.get_tok_at(1) {
                    Some(t) => {
                        if let TokenKind::Operator(_) = t {
                            return self.parse_binary_expr();
                        }
                        Expression::NumberLiteral(n)
                    },
                    None => Expression::NumberLiteral(n)
                }
            }
        }
    }

    fn parse_number_lit(&self) -> Expression {
        match self.get_curr_tok() {
            TokenKind::Operator(_) => panic!("Unexpected Token at token index {}: {:#?}", self.idx, self.get_curr_tok()),
            TokenKind::Number(n) => Expression::NumberLiteral(n)
        }
    }

    fn parse_binary_expr(&mut self) -> Expression {
        let left = self.parse_number_lit();
        
        let op_token = match self.get_next_tok().unwrap() {
            TokenKind::Operator(op) => op,
            _ => panic!("Unexpected Token at token index {}: {:#?}", self.idx, self.get_curr_tok()),
        };
        
        let op = self.parse_op(op_token);
        self.idx += 1;
        let right = self.parse_expression();

        Expression::BinaryExpression(Box::new(left), op, Box::new(right))
    }

}