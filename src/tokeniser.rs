
#[derive(Debug, Copy, Clone)]
pub(crate) enum TokenKind {
    Operator(TokenOpcode),
    Number(i32),
}

impl Into<Option<TokenOpcode>> for TokenKind {
    fn into(self) -> Option<TokenOpcode> {
        match self {
            TokenKind::Operator(op) => Some(op),
            _ => None,
        }
    }
}

impl Into<Option<i32>> for TokenKind {
    fn into(self) -> Option<i32> {
        match self {
            TokenKind::Number(n) => Some(n),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum TokenOpcode {
    Multiply,
    Add,
    Subtract,
    Divide,
}

pub(crate) struct Tokeniser {
    tokens: Vec<TokenKind>,
}

impl Tokeniser {

    pub fn new() -> Self {
        Self {
            tokens: vec![],
        }
    }

    pub fn from_string(mut self, str: String) -> Self {
        let split = str.split_whitespace().collect::<Vec<&str>>();

        for x in split {
            if let Some(op) = self.to_opcode(&String::from(x)) {
                self.tokens.push(TokenKind::Operator(op));
            }
            if self.is_int(&String::from(x)) {
                let n = x.parse::<i32>().unwrap();
                self.tokens.push(TokenKind::Number(n));
            }
        }

        self
    }

    pub fn get_tokens(self) -> Vec<TokenKind> {
        self.tokens
    }

    fn to_opcode(&self, value: &String) -> Option<TokenOpcode> {
        let opcode = match value.as_str() {
            "+" => TokenOpcode::Add,
            "-" => TokenOpcode::Subtract,
            "*" => TokenOpcode::Multiply,
            "/" => TokenOpcode::Divide,
            _ => return None,
        };
        Some(opcode)
    }

    fn is_int(&self, value: &String) -> bool {
        value.chars().all(|x| x.is_digit(10))
    }

}
