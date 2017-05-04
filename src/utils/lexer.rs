use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
enum Token {
    PIPE,
    AMPERSAND,
    OUTFILE,
    INFILE,
    TERMINATE,
    ID,
}

struct Lexer;

pub impl Lexer {
    fn grab_tokens(&self, input: &str) -> Vec<&str> {
        let tokens = Regex::new(r"-?[a-zA-Z]+|[.<>|&$]+").unwrap();
        let matches: Vec<&str> = tokens
            .find_iter(input)
            .map(|x| x.as_str())
            .collect();

        matches
    }
    fn tokenize(&self, grammar: &str) -> Vec<Token> {
        let mut iterator = grammar.chars();
        let mut tokens = vec![];

        while let Some(item) = iterator.next() {
            match item {
                '|' => tokens.push(Token::PIPE),
                '&' => tokens.push(Token::AMPERSAND),
                '>' => tokens.push(Token::OUTFILE),
                '<' => tokens.push(Token::INFILE),
                ';' => tokens.push(Token::TERMINATE),
                '$' => tokens.push(Token::ID),
                _ => { }
            }
        }

        tokens
    }
}
