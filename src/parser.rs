use crate::ast;
use crate::lexer;
use crate::token;

pub struct Parser {
    l: lexer::Lexer,

    current_token: token::Token,
    peek_token: token::Token,

    errors: Vec<String>,
}

impl Parser {
    fn next_token(&mut self) {
        self.current_token = token::Token {
            Type: self.peek_token.Type.to_owned(),
            Literal: self.peek_token.Literal.to_owned(),
        };
        self.peek_token = self.l.next_token();
    }

    fn peek_token_is(&self, t: token::TokenType) -> bool {
        return self.peek_token.Type == t;
    }

    fn expect_peek(&mut self, t: token::TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            return true;
        } else {
            // println!("error: {}", &t);
            self.peek_error();
            return false;
        }
    }

    fn peek_error(&mut self) {
        let mut message = "expected next token to be".to_string();
        // message.push_str(&t);
        message.push_str(", got");
        message.push_str(&self.peek_token.Type);
        message.push_str(" instead");

        println!(
            "expected next token to be: --, got {} instead",
            self.peek_token.Type
        );
        self.errors.push(message);
    }

    fn current_token_is(&self, t: token::TokenType) -> bool {
        return self.peek_token.Type == t;
    }

    fn parse_let_statement(&mut self) -> ast::Statement {
        let mut statement = ast::Statement {
            kind: ast::StatementKinds::LetStatement,
            token: token::Token {
                Type: self.current_token.Type.to_owned(),
                Literal: self.current_token.Literal.to_owned(),
            },
            name: ast::Expression {
                kind: ast::ExpressionKinds::Identifier,
                Token: token::Token {
                    Type: "".to_owned(),
                    Literal: "".to_owned(),
                },
                Value: "".to_owned(),
            },
            value: ast::Expression {
                kind: ast::ExpressionKinds::Identifier,
                Token: token::Token {
                    Type: "".to_owned(),
                    Literal: "".to_owned(),
                },
                Value: "".to_owned(),
            },
            is_empty: false,
        };

        if !self.expect_peek(token::IDENT.to_owned()) {
            return ast::Statement {
                kind: ast::StatementKinds::LetStatement,
                token: token::Token {
                    Type: "".to_owned(),
                    Literal: "".to_owned(),
                }, //TODO: might be wrong
                name: ast::Expression {
                    kind: ast::ExpressionKinds::Identifier,
                    Token: token::Token {
                        Type: "".to_owned(),
                        Literal: "".to_owned(),
                    },
                    Value: "".to_owned(),
                },
                value: ast::Expression {
                    kind: ast::ExpressionKinds::Identifier,
                    Token: token::Token {
                        Type: "".to_owned(),
                        Literal: "".to_owned(),
                    },
                    Value: "".to_owned(),
                },
                is_empty: true,
            };
        }

        statement.name = ast::Expression {
            kind: ast::ExpressionKinds::Identifier,
            Token: token::Token {
                Type: self.current_token.Type.to_owned(),
                Literal: self.current_token.Literal.to_owned(),
            },
            Value: self.current_token.Literal.to_owned(),
        };

        if !self.expect_peek(token::ASSIGN.to_owned()) {
            return ast::Statement {
                kind: ast::StatementKinds::LetStatement,
                token: token::Token {
                    Type: "".to_owned(),
                    Literal: "".to_owned(),
                }, //TODO: might be wrong
                name: ast::Expression {
                    kind: ast::ExpressionKinds::Identifier,
                    Token: token::Token {
                        Type: "".to_owned(),
                        Literal: "".to_owned(),
                    },
                    Value: "".to_owned(),
                },
                value: ast::Expression {
                    kind: ast::ExpressionKinds::Identifier,
                    Token: token::Token {
                        Type: "".to_owned(),
                        Literal: "".to_owned(),
                    },
                    Value: "".to_owned(),
                },
                is_empty: true,
            };
        }

        while !self.current_token_is(token::SEMICOLON.to_owned()) {
            self.next_token();
        }

        return statement;
    }

    fn parse_statement(&mut self) -> ast::Statement {
        if self.current_token.Type == token::LET {
            return self.parse_let_statement();
        } else {
            return ast::Statement {
                kind: ast::StatementKinds::LetStatement,
                token: token::Token {
                    Type: "".to_owned(),
                    Literal: "".to_owned(),
                }, //TODO: might be wrong
                name: ast::Expression {
                    kind: ast::ExpressionKinds::Identifier,
                    Token: token::Token {
                        Type: "".to_owned(),
                        Literal: "".to_owned(),
                    },
                    Value: "".to_owned(),
                },
                value: ast::Expression {
                    kind: ast::ExpressionKinds::Identifier,
                    Token: token::Token {
                        Type: "".to_owned(),
                        Literal: "".to_owned(),
                    },
                    Value: "".to_owned(),
                },
                is_empty: true,
            };
        }
    }

    pub fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program {
            Statements: Vec::new(),
        };

        while self.current_token.Type != token::EOF {
            let statement = self.parse_statement();

            if !statement.is_empty {
                program.Statements.push(statement);
            }

            self.next_token();
        }

        return program;
    }

    pub fn errors(&self) -> Vec<String> {
        return self.errors.to_owned();
    }
}

pub fn new(l: lexer::Lexer) -> Parser {
    let empty_token_current = token::Token {
        Type: "".to_owned(),
        Literal: "".to_owned(),
    };

    let empty_token_peek = token::Token {
        Type: "".to_owned(),
        Literal: "".to_owned(),
    };

    let mut p = Parser {
        l,
        current_token: empty_token_current,
        peek_token: empty_token_peek,
        errors: Vec::new(),
    };

    p.next_token();
    p.next_token();

    return p;
}
