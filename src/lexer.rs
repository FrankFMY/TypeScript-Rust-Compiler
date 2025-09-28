//! Lexical analysis for TypeScript code

use crate::error::{CompilerError, Result};
use serde::{Deserialize, Serialize};

/// Token types for TypeScript
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Token {
    // Literals
    Number(f64),
    String(String),
    TemplateLiteral(String),
    Boolean(bool),
    Null,
    Undefined,

    // Identifiers and keywords
    Identifier(String),
    Keyword(Keyword),

    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    StrictEqual,
    StrictNotEqual,
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    Not,
    Assign,
    PlusAssign,
    MinusAssign,
    MultiplyAssign,
    DivideAssign,
    Union, // |
    Intersection, // &

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,
    Dot,
    Colon,
    QuestionMark,
    Arrow,

    // Type annotations
    TypeAnnotation,
    GenericStart,
    GenericEnd,

    // Special
    Newline,
    Whitespace,
    Comment(String),
    EOF,
}

/// TypeScript keywords
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Keyword {
    // Declarations
    Let,
    Const,
    Var,
    Function,
    Class,
    Interface,
    Type,
    Enum,
    Namespace,
    Module,
    Import,
    Export,
    From,
    As,
    Default,

    // Control flow
    If,
    Else,
    Switch,
    Case,
    DefaultCase,
    For,
    While,
    Do,
    Break,
    Continue,
    Return,
    Throw,
    Try,
    Catch,
    Finally,

    // OOP
    Extends,
    Implements,
    Super,
    This,
    New,
    Static,
    Public,
    Private,
    Protected,
    Abstract,
    Readonly,

    // Async
    Async,
    Await,
    Promise,

    // Types
    Any,
    Unknown,
    Never,
    Void,
    Null,
    Undefined,
    Boolean,
    Number,
    String,
    Object,
    Array,
    Tuple,
    Union,
    Intersection,
    Literal,
    Mapped,
    Conditional,
    Template,

    // Utility types
    Partial,
    Required,
    Pick,
    Omit,
    Record,
    Exclude,
    Extract,
    NonNullable,
    Parameters,
    ReturnType,
    InstanceType,
    ThisParameterType,
    OmitThisParameter,
    ThisType,

    // Other
    True,
    False,
    In,
    Of,
    Instanceof,
    Typeof,
    Keyof,
    Key,
    Is,
    Asserts,
    Infer,
    Declare,
    Ambient,
    Global,
}

/// Lexer for TypeScript code
pub struct Lexer {
    input: String,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    /// Create a new lexer
    pub fn new(input: String) -> Self {
        Self {
            input,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// Tokenize the input string
    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while self.position < self.input.len() {
            match self.next_token()? {
                Some(token) => {
                    println!("Token: {:?}", token);
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

        if self.position >= self.input.len() {
            return Ok(None);
        }

        let ch = self.current_char();
        let token = match ch {
            '+' => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    Ok(Some(Token::PlusAssign))
                } else if self.peek_char() == Some('+') {
                    self.advance();
                    Ok(Some(Token::Plus)) // ++ operator
                } else {
                    Ok(Some(Token::Plus))
                }
            }
            '-' => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    Ok(Some(Token::MinusAssign))
                } else if self.peek_char() == Some('>') {
                    self.advance();
                    Ok(Some(Token::Arrow))
                } else {
                    Ok(Some(Token::Minus))
                }
            }
            '*' => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    Ok(Some(Token::MultiplyAssign))
                } else {
                    Ok(Some(Token::Multiply))
                }
            }
            '/' => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    Ok(Some(Token::DivideAssign))
                } else if self.peek_char() == Some('/') {
                    self.advance();
                    self.skip_line_comment();
                    Ok(None)
                } else if self.peek_char() == Some('*') {
                    self.advance();
                    self.skip_block_comment();
                    Ok(None)
                } else {
                    Ok(Some(Token::Divide))
                }
            }
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
            'a'..='z' | 'A'..='Z' | '_' | '$' => Ok(self.parse_identifier_or_keyword()?),
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
            'a'..='z' | 'A'..='Z' | '_' | '$' => {
                // parse_identifier_or_keyword manages position itself
            }
            '0'..='9' => {
                // parse_number manages position itself
            }
            '"' | '\'' => {
                // parse_string manages position itself
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
        self.input.chars().nth(self.position).unwrap_or('\0')
    }

    /// Peek at next character
    fn peek_char(&self) -> Option<char> {
        self.input.chars().nth(self.position + 1)
    }

    /// Advance position
    fn advance(&mut self) {
        if self.current_char() == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        self.position += 1;
    }

    /// Skip whitespace
    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() {
            let ch = self.current_char();
            if ch.is_whitespace() {
                self.advance();
            } else if ch == '/' && self.peek_char() == Some('/') {
                // Skip line comment
                self.advance(); // skip first /
                self.advance(); // skip second /
                while self.position < self.input.len() && self.current_char() != '\n' {
                    self.advance();
                }
            } else if ch == '/' && self.peek_char() == Some('*') {
                // Skip block comment
                self.advance(); // skip /
                self.advance(); // skip *
                while self.position < self.input.len() {
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

    /// Skip line comment
    fn skip_line_comment(&mut self) -> Option<Token> {
        while self.position < self.input.len() && self.current_char() != '\n' {
            self.advance();
        }
        None
    }

    /// Skip block comment
    fn skip_block_comment(&mut self) -> Option<Token> {
        while self.position < self.input.len() {
            if self.current_char() == '*' && self.peek_char() == Some('/') {
                self.advance();
                self.advance();
                break;
            }
            self.advance();
        }
        None
    }

    /// Parse string literal
    fn parse_string(&mut self) -> Result<Option<Token>> {
        let quote = self.current_char();
        let mut value = String::new();
        self.advance();

        while self.position < self.input.len() {
            let ch = self.current_char();
            if ch == quote {
                self.advance();
                return Ok(Some(Token::String(value)));
            } else if ch == '\\' {
                self.advance();
                if self.position < self.input.len() {
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
        self.advance(); // consume opening backtick

        while self.position < self.input.len() {
            let ch = self.current_char();
            if ch == '`' {
                self.advance();
                return Ok(Some(Token::TemplateLiteral(value)));
            } else if ch == '\\' {
                self.advance();
                if self.position < self.input.len() {
                    let escaped = self.current_char();
                    value.push(match escaped {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '\\' => '\\',
                        '`' => '`',
                        '$' => '$',
                        _ => escaped,
                    });
                    self.advance();
                }
            } else if ch == '$' && self.position + 1 < self.input.len() && self.input.chars().nth(self.position + 1) == Some('{') {
                // Handle ${} interpolation - include the full ${} in the string for now
                value.push('$');
                self.advance();
                if self.position < self.input.len() {
                    value.push('{');
                    self.advance();
                    // Skip to closing brace
                    while self.position < self.input.len() && self.current_char() != '}' {
                        value.push(self.current_char());
                        self.advance();
                    }
                    if self.position < self.input.len() {
                        value.push('}');
                        self.advance();
                    }
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
        let mut has_dot = false;

        while self.position < self.input.len() {
            let ch = self.current_char();
            if ch.is_ascii_digit() {
                value.push(ch);
                self.advance();
            } else if ch == '.' && !has_dot {
                has_dot = true;
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        let number: f64 = value.parse().map_err(|_| {
            CompilerError::parse_error(self.line, self.column, "Invalid number literal")
        })?;

        Ok(Some(Token::Number(number)))
    }

    /// Parse identifier or keyword
    fn parse_identifier_or_keyword(&mut self) -> Result<Option<Token>> {
        let mut value = String::new();

        while self.position < self.input.len() {
            let ch = self.current_char();
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '$' {
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        // Check if it's a boolean literal
        if value == "true" {
            Ok(Some(Token::Boolean(true)))
        } else if value == "false" {
            Ok(Some(Token::Boolean(false)))
        } else if let Some(keyword) = self.parse_keyword(&value) {
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
            "import" => Some(Keyword::Import),
            "export" => Some(Keyword::Export),
            "from" => Some(Keyword::From),
            "as" => Some(Keyword::As),
            "default" => Some(Keyword::Default),
            "if" => Some(Keyword::If),
            "else" => Some(Keyword::Else),
            "switch" => Some(Keyword::Switch),
            "case" => Some(Keyword::Case),
            "for" => Some(Keyword::For),
            "while" => Some(Keyword::While),
            "do" => Some(Keyword::Do),
            "break" => Some(Keyword::Break),
            "continue" => Some(Keyword::Continue),
            "return" => Some(Keyword::Return),
            "throw" => Some(Keyword::Throw),
            "try" => Some(Keyword::Try),
            "catch" => Some(Keyword::Catch),
            "finally" => Some(Keyword::Finally),
            "extends" => Some(Keyword::Extends),
            "implements" => Some(Keyword::Implements),
            "super" => Some(Keyword::Super),
            "this" => Some(Keyword::This),
            "new" => Some(Keyword::New),
            "static" => Some(Keyword::Static),
            "public" => Some(Keyword::Public),
            "private" => Some(Keyword::Private),
            "protected" => Some(Keyword::Protected),
            "abstract" => Some(Keyword::Abstract),
            "readonly" => Some(Keyword::Readonly),
            "async" => Some(Keyword::Async),
            "await" => Some(Keyword::Await),
            "Promise" => Some(Keyword::Promise),
            "any" => Some(Keyword::Any),
            "unknown" => Some(Keyword::Unknown),
            "never" => Some(Keyword::Never),
            "void" => Some(Keyword::Void),
            "null" => Some(Keyword::Null),
            "undefined" => Some(Keyword::Undefined),
            "boolean" => Some(Keyword::Boolean),
            "number" => Some(Keyword::Number),
            "string" => Some(Keyword::String),
            "object" => Some(Keyword::Object),
            "Array" => Some(Keyword::Array),
            "true" => Some(Keyword::True),
            "false" => Some(Keyword::False),
            "in" => Some(Keyword::In),
            "of" => Some(Keyword::Of),
            "instanceof" => Some(Keyword::Instanceof),
            "typeof" => Some(Keyword::Typeof),
            "keyof" => Some(Keyword::Keyof),
            "key" => Some(Keyword::Key),
            "is" => Some(Keyword::Is),
            "asserts" => Some(Keyword::Asserts),
            "infer" => Some(Keyword::Infer),
            "declare" => Some(Keyword::Declare),
            "global" => Some(Keyword::Global),
            _ => None,
        }
    }
}
