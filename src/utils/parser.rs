use std::io;
use ast::{self, Grammar};

use regex::Regex;

struct Parser;

pub impl Parser {
    fn parse(&self, tokens: Vec<Token>, rule_count: i32) -> Result<Grammar, usize> {
        let mut grammar_object = Grammar { rules: vec![], main: 0 };
        let mut insert_order = vec![];
        let mut i = 0;

        while let Some(token) = tokens.get(i) {
            if let &Token::Name(id) = token {
                i += 1;
                if let Some(brace_token) = tokens.get(i) {
                    i += 1;
                    if brace_token == &Token::OpenBrace {
                        let pattern = parse_expression(&mut i, &tokens);
                        match tokens.get(i) {
                            Some(&Token::CloseBrace) => i += 1,
                            _ => return Err(i)
                        }
                        match pattern {
                            Ok(p) => insert_order.push((id, p)),
                            Err(x) => return Err(x)
                        }
                    } else {
                        return Err(i);
                    }
                } else {
                    return Err(i);
                }
            } else {
                return Err(i);
            }
        }

        insert_order.sort_by(|a, b| a.0.cmp(&b.0));

        if insert_order.iter().map(|x| x.0).eq(0..rule_count) {
            grammar_object.rules = insert_order.drain(..).map(|x| x.1).collect();
            Ok(grammar_object)
        } else {
            Err(i)
        }
    }
}
