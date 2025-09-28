//! UTF-8 compatible lexer for TypeScript

use crate::error::{CompilerError, Result};
use crate::lexer::{Token, Keyword};

/// UTF-8 compatible lexer
pub struct Utf8Lexer {
    chars: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Utf8Lexer {
    /// Create a new UTF-8 lexer
    pub fn new(input: String) -> Self {
        Self {
            chars: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// Tokenize the input string
    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while self.position < self.chars.len() {
            match self.next_token()? {
                Some(token) => {
                    tokens.push(token);
                }
                None => break,
            }
        }

        tokens.push(Token::EOF);
        Ok(tokens)
    }

    /// Get the next token
    fn next_token(&mut self) -> Result<Option<Token>> {
        self.skip_whitespace();

        if self.position >= self.chars.len() {
            return Ok(None);
        }

        let ch = self.current_char();
        let token = match ch {
            '+' => Ok(Some(Token::Plus)),
            '-' => Ok(Some(Token::Minus)),
            '*' => Ok(Some(Token::Multiply)),
            '/' => Ok(Some(Token::Divide)),
            '%' => Ok(Some(Token::Modulo)),
            '=' => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    if self.peek_char() == Some('=') {
                        self.advance();
                        Ok(Some(Token::StrictEqual))
                    } else {
                        Ok(Some(Token::Equal))
                    }
                } else {
                    Ok(Some(Token::Assign))
                }
            }
            '!' => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    if self.peek_char() == Some('=') {
                        self.advance();
                        Ok(Some(Token::StrictNotEqual))
                    } else {
                        Ok(Some(Token::NotEqual))
                    }
                } else {
                    Ok(Some(Token::Not))
                }
            }
            '<' => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    Ok(Some(Token::LessEqual))
                } else {
                    Ok(Some(Token::LessThan))
                }
            }
            '>' => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    Ok(Some(Token::GreaterEqual))
                } else {
                    Ok(Some(Token::GreaterThan))
                }
            }
            '&' => {
                if self.peek_char() == Some('&') {
                    self.advance();
                    Ok(Some(Token::And))
                } else {
                    Ok(Some(Token::Intersection))
                }
            }
            '|' => {
                if self.peek_char() == Some('|') {
                    self.advance();
                    Ok(Some(Token::Or))
                } else {
                    Ok(Some(Token::Union))
                }
            }
            '(' => Ok(Some(Token::LeftParen)),
            ')' => Ok(Some(Token::RightParen)),
            '{' => Ok(Some(Token::LeftBrace)),
            '}' => Ok(Some(Token::RightBrace)),
            '[' => Ok(Some(Token::LeftBracket)),
            ']' => Ok(Some(Token::RightBracket)),
            ';' => Ok(Some(Token::Semicolon)),
            ',' => Ok(Some(Token::Comma)),
            '.' => Ok(Some(Token::Dot)),
            ':' => Ok(Some(Token::Colon)),
            '?' => Ok(Some(Token::QuestionMark)),
            '"' | '\'' => Ok(self.parse_string()?),
            '`' => Ok(self.parse_template_literal()?),
            '0'..='9' => Ok(self.parse_number()?),
            _ if ch.is_alphabetic() || ch == '_' || ch == '$' => Ok(self.parse_identifier_or_keyword()?),
            _ => {
                return Err(CompilerError::parse_error(
                    self.line,
                    self.column,
                    format!("Unexpected character: {}", ch),
                ));
            }
        };

        // Only advance for simple tokens that don't manage position themselves
        match ch {
            '0'..='9' => {
                // parse_number manages position itself
            }
            '"' | '\'' => {
                // parse_string manages position itself
            }
            _ if ch.is_alphabetic() || ch == '_' || ch == '$' => {
                // parse_identifier_or_keyword manages position itself
            }
            _ => {
                // Simple tokens need to advance
                self.advance();
            }
        }
        token
    }

    /// Get current character
    fn current_char(&self) -> char {
        self.chars.get(self.position).copied().unwrap_or('\0')
    }

    /// Peek at next character
    fn peek_char(&self) -> Option<char> {
        self.chars.get(self.position + 1).copied()
    }

    /// Advance position
    fn advance(&mut self) {
        if self.position < self.chars.len() {
            let ch = self.current_char();
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.position += 1;
        }
    }

    /// Skip whitespace
    fn skip_whitespace(&mut self) {
        while self.position < self.chars.len() {
            let ch = self.current_char();
            if ch.is_whitespace() {
                self.advance();
            } else if ch == '/' && self.peek_char() == Some('/') {
                // Skip line comment
                self.advance(); // skip first /
                self.advance(); // skip second /
                while self.position < self.chars.len() && self.current_char() != '\n' {
                    self.advance();
                }
            } else if ch == '/' && self.peek_char() == Some('*') {
                // Skip block comment
                self.advance(); // skip /
                self.advance(); // skip *
                while self.position < self.chars.len() {
                    if self.current_char() == '*' && self.peek_char() == Some('/') {
                        self.advance(); // skip *
                        self.advance(); // skip /
                        break;
                    }
                    self.advance();
                }
            } else {
                break;
            }
        }
    }

    /// Parse string literal
    fn parse_string(&mut self) -> Result<Option<Token>> {
        let quote = self.current_char();
        let mut value = String::new();
        self.advance();

        while self.position < self.chars.len() {
            let ch = self.current_char();
            if ch == quote {
                self.advance();
                return Ok(Some(Token::String(value)));
            } else if ch == '\\' {
                self.advance();
                if self.position < self.chars.len() {
                    let escaped = self.current_char();
                    value.push(match escaped {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '\\' => '\\',
                        '"' => '"',
                        '\'' => '\'',
                        _ => escaped,
                    });
                    self.advance();
                }
            } else {
                value.push(ch);
                self.advance();
            }
        }

        Err(CompilerError::parse_error(
            self.line,
            self.column,
            "Unterminated string literal",
        ))
    }

    /// Parse template literal
    fn parse_template_literal(&mut self) -> Result<Option<Token>> {
        let mut value = String::new();
        self.advance();

        while self.position < self.chars.len() {
            let ch = self.current_char();
            if ch == '`' {
                self.advance();
                return Ok(Some(Token::TemplateLiteral(value)));
            } else if ch == '\\' {
                self.advance();
                if self.position < self.chars.len() {
                    let escaped = self.current_char();
                    value.push(match escaped {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '\\' => '\\',
                        '`' => '`',
                        _ => escaped,
                    });
                    self.advance();
                }
            } else {
                value.push(ch);
                self.advance();
            }
        }

        Err(CompilerError::parse_error(
            self.line,
            self.column,
            "Unterminated template literal",
        ))
    }

    /// Parse number literal
    fn parse_number(&mut self) -> Result<Option<Token>> {
        let mut value = String::new();

        while self.position < self.chars.len() {
            let ch = self.current_char();
            if ch.is_ascii_digit() || ch == '.' {
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        match value.parse::<f64>() {
            Ok(num) => Ok(Some(Token::Number(num))),
            Err(_) => Err(CompilerError::parse_error(
                self.line,
                self.column,
                format!("Invalid number: {}", value),
            )),
        }
    }

    /// Parse identifier or keyword
    fn parse_identifier_or_keyword(&mut self) -> Result<Option<Token>> {
        let mut value = String::new();

        while self.position < self.chars.len() {
            let ch = self.current_char();
            if ch.is_alphanumeric() || ch == '_' || ch == '$' {
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        // Check if it's a keyword
        if let Some(keyword) = self.parse_keyword(&value) {
            Ok(Some(Token::Keyword(keyword)))
        } else {
            Ok(Some(Token::Identifier(value)))
        }
    }

    /// Parse keyword from string
    fn parse_keyword(&self, value: &str) -> Option<Keyword> {
        match value {
            "let" => Some(Keyword::Let),
            "const" => Some(Keyword::Const),
            "var" => Some(Keyword::Var),
            "function" => Some(Keyword::Function),
            "class" => Some(Keyword::Class),
            "interface" => Some(Keyword::Interface),
            "type" => Some(Keyword::Type),
            "enum" => Some(Keyword::Enum),
            "namespace" => Some(Keyword::Namespace),
            "module" => Some(Keyword::Module),
            "export" => Some(Keyword::Export),
            "import" => Some(Keyword::Import),
            "default" => Some(Keyword::Default),
            "public" => Some(Keyword::Public),
            "private" => Some(Keyword::Private),
            "protected" => Some(Keyword::Protected),
            "static" => Some(Keyword::Static),
            "readonly" => Some(Keyword::Readonly),
            "abstract" => Some(Keyword::Abstract),
            "async" => Some(Keyword::Async),
            "await" => Some(Keyword::Await),
            "extends" => Some(Keyword::Extends),
            "implements" => Some(Keyword::Implements),
            "constructor" => Some(Keyword::Constructor),
            "get" => Some(Keyword::Get),
            "set" => Some(Keyword::Set),
            "this" => Some(Keyword::This),
            "super" => Some(Keyword::Super),
            "new" => Some(Keyword::New),
            "return" => Some(Keyword::Return),
            "if" => Some(Keyword::If),
            "else" => Some(Keyword::Else),
            "while" => Some(Keyword::While),
            "for" => Some(Keyword::For),
            "do" => Some(Keyword::Do),
            "break" => Some(Keyword::Break),
            "continue" => Some(Keyword::Continue),
            "switch" => Some(Keyword::Switch),
            "case" => Some(Keyword::Case),
            "default" => Some(Keyword::Default),
            "try" => Some(Keyword::Try),
            "catch" => Some(Keyword::Catch),
            "finally" => Some(Keyword::Finally),
            "throw" => Some(Keyword::Throw),
            "true" => Some(Keyword::True),
            "false" => Some(Keyword::False),
            "null" => Some(Keyword::Null),
            "undefined" => Some(Keyword::Undefined),
            "void" => Some(Keyword::Void),
            "never" => Some(Keyword::Never),
            "any" => Some(Keyword::Any),
            "unknown" => Some(Keyword::Unknown),
            "object" => Some(Keyword::Object),
            "string" => Some(Keyword::String),
            "number" => Some(Keyword::Number),
            "boolean" => Some(Keyword::Boolean),
            "symbol" => Some(Keyword::Symbol),
            "bigint" => Some(Keyword::BigInt),
            _ => None,
        }
    }
}
