use crate::token;

pub trait Node {
    fn literal(&self) -> String;
    fn to_string(&self) -> String;
}

#[derive(Debug)]
pub struct DefaultNode {
    token: token::Token
} 

impl DefaultNode {
    fn new() -> Self {
        DefaultNode {
            token: token::Token::new()
        }
    }
}

impl Node for DefaultNode {
    fn literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        self.token.literal.clone()
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub token: token::Token,
    pub value: String
}

impl Identifier {
    pub fn new() -> Self {
        Identifier {
            token: token::Token::new(),
            value: String::new()
        }
    }
}

impl Node for Identifier {
    fn literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        format!("{}", self.value)
    }
}

pub struct Program {
    pub statements: Vec<Box<dyn Node>>
}

impl Program {
    pub fn new() -> Self {
        Program{
            statements: vec![]       
        }
    }
}

impl Node for Program {
    fn literal(&self) -> String {
        if self.statements.len() > 0 {
            return self.statements[0].literal();
        }

        String::new()
    }

    fn to_string(&self) -> String {
        self.statements
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

// Let
pub struct LetStatement {
    pub token: token::Token,
    pub name: Identifier,
    pub value: Box<dyn Node>
}

impl LetStatement {
    pub fn new(token: token::Token) -> Self {
        LetStatement {
            token,
            name: Identifier::new(),
            value: Box::new(DefaultNode::new())
        }
    }
}

impl Node for LetStatement {
    fn literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        format!("{} {} = {};",
            self.token.literal,
            self.name.value,
            self.value.literal())
    }
}

// Return
pub struct ReturnStatement {
    pub token: token::Token,
    pub value: Box<dyn Node>
}

impl ReturnStatement {
    pub fn new(token: token::Token) -> Self {
        Self {
            token,
            value: Box::new(DefaultNode::new())
        }
    }
}

impl Node for ReturnStatement {
    fn literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        format!("{} {};",
            self.token.literal,
            self.value.to_string())
    }
}

// Expression
pub struct ExpressionStatement {
    pub token: token::Token,
    pub value: Box<dyn Node>
}

impl ExpressionStatement {
    pub fn new(token: token::Token) -> Self {
        Self {
            token,
            value: Box::new(DefaultNode::new())
        }
    }
}

impl Node for ExpressionStatement {
    fn literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        format!("{}", self.value.to_string())
    }
}


pub struct IntegerLiteral {
    pub token: token::Token,
    pub value: i64
}

impl IntegerLiteral {
    pub fn new() -> Self {
        Self {
            token: token::Token::new(),
            value: 0
        }
    }
}

impl Node for IntegerLiteral {
    fn literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        format!("{}", self.token.literal)
    }
}

pub struct PrefixExpression {
    pub token: token::Token,
    pub operator: String,
    pub right: Box<dyn Node>
}

impl PrefixExpression {
    pub fn new(token: token::Token) -> Self {
        Self {
            token: token.clone(),
            operator: String::new(),
            right: Box::new(ExpressionStatement::new(token))
        }
    }
}

impl Node for PrefixExpression {
    fn literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        format!("({} {})", self.operator.clone(), self.right.to_string())
    }
}
