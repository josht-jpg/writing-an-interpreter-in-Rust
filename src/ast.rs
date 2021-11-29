use crate::ast;
use crate::token;

trait Node {
    fn token_literal(&self) -> String;
}

// struct Statement {}

pub enum ExpressionKinds {
    Identifier,
}

pub struct Expression {
    pub kind: ExpressionKinds,
    pub Token: token::Token,
    pub Value: String,
}

impl Expression {
    fn expression_node() {}
    pub fn token_literal(&self) -> String {
        return self.Token.Literal.to_owned();
    }
}

pub enum StatementKinds {
    LetStatement,
    ReturnStatement,
}

//Should be just called Statement

pub struct Statement {
    pub kind: StatementKinds,
    pub token: token::Token,
    //TODO: might be wrong
    pub name: ast::Expression,
    //return_value
    pub value: Expression,
    pub is_empty: bool,
}

impl Statement {
    fn statement_node() {}

    pub fn token_literal(&self) -> String {
        return self.token.Literal.to_owned();
    }
}
/*enum Statement {
    LetStatement,
}*/

/*trait Expression: Node {
    fn expression_node();
}*/

pub struct Program {
    pub Statements: Vec<Statement>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.Statements.len() > 0 {
            return self.Statements[0].token_literal();
        } else {
            return "".to_owned();
        }
    }
}

/*struct Identifier {
    token: token::Token,
    value: &str,
}*/
