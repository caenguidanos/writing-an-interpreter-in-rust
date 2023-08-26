trait Node {
    fn token_literal(&self) -> String;
}

trait Statement: Node {
    fn statement_node(&self);
}

trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program<'statements> {
    statements: Vec<&'statements dyn Statement>,
}

impl<'statements> Node for Program<'statements> {
    fn token_literal(&self) -> String {
        if self.statements.is_empty() {
            String::new()
        } else {
            self.statements[0].token_literal()
        }
    }
}

struct LetStatement<'name, 'value> {
    token: token::Token,
    name: &'name Identifier,
    value: &'value dyn Expression,
}

impl<'name, 'value> Node for LetStatement<'name, 'value> {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl<'name, 'value> Statement for LetStatement<'name, 'value> {
    fn statement_node(&self) {
        todo!()
    }
}

struct Identifier {
    token: token::Token,
    value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {
        todo!()
    }
}
