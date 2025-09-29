//! Parser for TypeScript code using nom parser combinators

use crate::ast::*;
use crate::error::{CompilerError, Result};
use crate::lexer::{Token, Keyword};

/// Parser for TypeScript code
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    /// Create a new parser
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    /// Parse the tokens into an AST
    pub fn parse(&mut self) -> Result<Program> {
        let mut statements = Vec::new();
        let mut iterations = 0;
        let max_iterations = self.tokens.len() * 2; // Prevent infinite loops
        let mut errors = Vec::new();

        while self.position < self.tokens.len() && iterations < max_iterations {
            let old_position = self.position;

            match self.parse_statement() {
                Ok(Some(statement)) => {
                    statements.push(statement);
                }
                Ok(None) => {
                    break;
                }
                Err(error) => {
                    // Log error but continue parsing
                    errors.push(error);
                    // Skip current token and continue
                    self.advance();
                }
            }
            
            // Check if we made progress
            if self.position == old_position {
                // No progress made, advance position to prevent infinite loop
                self.position += 1;
            }
            
            // If we encounter an error, try to recover by skipping tokens
            if self.position < self.tokens.len() {
                let current_token = &self.tokens[self.position];
                if matches!(current_token, Token::EOF) {
                    break;
                }
            }
            
            iterations += 1;
        }

        if iterations >= max_iterations {
            return Err(CompilerError::parse_error(
                self.position,
                1,
                "Parser stuck in infinite loop".to_string(),
            ));
        }

        // If we have statements, return them even if there were errors
        if !statements.is_empty() {
            Ok(Program { statements })
        } else if !errors.is_empty() {
            // If no statements but we have errors, return the first error
            Err(errors.into_iter().next().unwrap())
        } else {
            Ok(Program { statements })
        }
    }

    /// Parse a statement
    fn parse_statement(&mut self) -> Result<Option<Statement>> {
        if self.position >= self.tokens.len() {
            return Ok(None);
        }

        // Check if we've reached EOF
        if self.current_token() == &Token::EOF {
            return Ok(None);
        }

        let token = &self.tokens[self.position];

        let statement = match token {
            Token::EOF => return Ok(None),
            Token::Keyword(keyword) => match keyword {
                crate::lexer::Keyword::Let
                | crate::lexer::Keyword::Var => self.parse_variable_declaration()?,
                crate::lexer::Keyword::Const => {
                    // Check if this is "const enum"
                    if self.position + 1 < self.tokens.len() {
                        if let Token::Keyword(crate::lexer::Keyword::Enum) = &self.tokens[self.position + 1] {
                            // This is "const enum", parse as enum declaration
                            self.parse_const_enum_declaration()?
                        } else {
                            // This is regular "const", parse as variable declaration
                            self.parse_variable_declaration()?
                        }
                    } else {
                        self.parse_variable_declaration()?
                    }
                },
                crate::lexer::Keyword::Function => self.parse_function_declaration()?,
                crate::lexer::Keyword::Class => self.parse_class_declaration()?,
                crate::lexer::Keyword::Interface => self.parse_interface_declaration()?,
                crate::lexer::Keyword::Type => self.parse_type_alias()?,
                crate::lexer::Keyword::Enum => self.parse_enum_declaration()?,
                crate::lexer::Keyword::Import => self.parse_import_declaration()?,
                crate::lexer::Keyword::Export => self.parse_export_declaration()?,
                crate::lexer::Keyword::Namespace => self.parse_namespace_declaration()?,
                crate::lexer::Keyword::Module => self.parse_module_declaration()?,
                crate::lexer::Keyword::Declare => self.parse_declare_statement()?,
                crate::lexer::Keyword::Return => self.parse_return_statement()?,
                crate::lexer::Keyword::Throw => self.parse_throw_statement()?,
                crate::lexer::Keyword::If => self.parse_if_statement()?,
                crate::lexer::Keyword::Else => self.parse_expression_statement()?,
                _ => self.parse_expression_statement()?,
            },
            Token::LeftBrace => self.parse_block_statement()?,
            Token::Semicolon => {
                self.advance();
                return self.parse_statement();
            }
            _ => {
                // Try to parse as expression statement, but if it fails, skip the token
                match self.parse_expression_statement() {
                    Ok(expr_stmt) => expr_stmt,
                    Err(_) => {
                        // Skip problematic token and continue
                        self.advance();
                        return Ok(None);
                    }
                }
            }
        };

        Ok(Some(statement))
    }

    /// Parse variable declaration
    fn parse_variable_declaration(&mut self) -> Result<Statement> {
        let keyword = self.expect_keyword()?;
        let name = self.expect_identifier()?;
        let type_annotation = if self.current_token() == &Token::Colon {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        let initializer = if self.current_token() == &Token::Assign {
            self.advance();
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.expect_semicolon()?;

        Ok(Statement::VariableDeclaration(VariableDeclaration {
            keyword,
            name,
            type_annotation,
            initializer,
        }))
    }

    /// Parse function declaration
    fn parse_function_declaration(&mut self) -> Result<Statement> {
        self.expect_keyword()?; // consume 'function' keyword
        let name = self.expect_identifier()?;
        let type_parameters = self.parse_type_parameters()?;
        let parameters = self.parse_parameters()?;
        let return_type = if self.current_token() == &Token::Colon {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };
        let body = self.parse_block_statement()?;

        Ok(Statement::FunctionDeclaration(FunctionDeclaration {
            name,
            type_parameters,
            parameters,
            return_type,
            body: Box::new(body),
        }))
    }

    /// Parse class declaration
    fn parse_class_declaration(&mut self) -> Result<Statement> {
        self.expect_keyword()?; // consume 'class' keyword
        let name = self.expect_identifier()?;
        let type_parameters = self.parse_type_parameters()?;
        let extends = if self.current_token() == &Token::Keyword(crate::lexer::Keyword::Extends) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };
        let implements = self.parse_implements()?;
        let body = self.parse_class_body()?;

        Ok(Statement::ClassDeclaration(ClassDeclaration {
            name,
            type_parameters,
            extends,
            implements,
            body,
        }))
    }

    /// Parse interface declaration
    fn parse_interface_declaration(&mut self) -> Result<Statement> {
        self.expect_keyword()?; // consume 'interface' keyword
        let name = self.expect_identifier()?;
        let type_parameters = self.parse_type_parameters()?;
        let extends = self.parse_extends()?;
        let body = self.parse_interface_body()?;

        Ok(Statement::InterfaceDeclaration(InterfaceDeclaration {
            name,
            type_parameters,
            extends,
            body,
        }))
    }

    /// Parse type alias
    fn parse_type_alias(&mut self) -> Result<Statement> {
        self.expect_keyword()?; // consume 'type' keyword
        let name = self.expect_identifier()?;
        let type_parameters = self.parse_type_parameters()?;
        self.expect_token(&Token::Assign)?;
        let type_definition = self.parse_type()?;
        self.expect_semicolon()?;

        Ok(Statement::TypeAlias(TypeAlias {
            name,
            type_parameters,
            type_definition,
        }))
    }

    /// Parse enum declaration
    fn parse_enum_declaration(&mut self) -> Result<Statement> {
        self.expect_keyword()?; // consume 'enum' keyword
        let name = self.expect_identifier()?;
        let members = self.parse_enum_members()?;

        Ok(Statement::EnumDeclaration(EnumDeclaration {
            name,
            members,
        }))
    }

    fn parse_const_enum_declaration(&mut self) -> Result<Statement> {
        self.expect_keyword()?; // consume 'const' keyword
        self.expect_keyword()?; // consume 'enum' keyword
        let name = self.expect_identifier()?;
        let members = self.parse_enum_members()?;

        // For now, treat const enum the same as regular enum
        // In a full implementation, we'd have a ConstEnumDeclaration type
        Ok(Statement::EnumDeclaration(EnumDeclaration {
            name,
            members,
        }))
    }

    /// Parse import declaration
    fn parse_import_declaration(&mut self) -> Result<Statement> {
        self.expect_keyword()?; // import
        let specifiers = self.parse_import_specifiers()?;
        self.expect_keyword()?; // from
        let source = self.parse_string_literal()?;
        self.expect_semicolon()?;

        Ok(Statement::ImportDeclaration(ImportDeclaration {
            specifiers,
            source,
        }))
    }

    /// Parse export declaration
    fn parse_export_declaration(&mut self) -> Result<Statement> {
        // Consume 'export' keyword
        self.advance();
        // Parse the specific declaration type directly
        let token = self.current_token().clone();
        let declaration = match token {
            Token::Keyword(crate::lexer::Keyword::Class) => self.parse_class_declaration()?,
            Token::Keyword(crate::lexer::Keyword::Interface) => {
                self.parse_interface_declaration()?
            }
            Token::Keyword(crate::lexer::Keyword::Function) => self.parse_function_declaration()?,
            Token::Keyword(crate::lexer::Keyword::Const) => self.parse_variable_declaration()?,
            Token::Keyword(crate::lexer::Keyword::Let) => self.parse_variable_declaration()?,
            Token::Keyword(crate::lexer::Keyword::Var) => self.parse_variable_declaration()?,
            Token::Keyword(crate::lexer::Keyword::Enum) => self.parse_enum_declaration()?,
            Token::Keyword(crate::lexer::Keyword::Type) => {
                // Check if this is "export type { ... }" or "export type Name = ..."
                if self.position + 1 < self.tokens.len() {
                    if let Token::LeftBrace = &self.tokens[self.position + 1] {
                        // This is "export type { ... }", parse as export type statement
                        self.parse_export_type_statement()?
                    } else {
                        // This is "export type Name = ...", parse as type alias
                        self.parse_type_alias()?
                    }
                } else {
                    self.parse_type_alias()?
                }
            },
            Token::LeftBrace => {
                // This is "export { ... }", parse as export statement
                self.parse_export_statement()?
            },
            _ => {
                return Err(CompilerError::parse_error(
                    1,
                    1,
                    format!("Unexpected token in export declaration: {:?}", token),
                ))
            }
        };
        Ok(Statement::ExportDeclaration(Box::new(ExportDeclaration {
            declaration: Box::new(declaration),
        })))
    }

    fn parse_export_statement(&mut self) -> Result<Statement> {
        self.expect_token(&Token::LeftBrace)?; // consume '{'
        
        let mut exports = Vec::new();
        while self.current_token() != &Token::RightBrace && self.current_token() != &Token::EOF {
            let name = self.expect_identifier()?;
            exports.push(name);
            
            if self.current_token() == &Token::Comma {
                self.advance(); // consume ','
            }
        }
        
        self.expect_token(&Token::RightBrace)?; // consume '}'
        self.expect_semicolon()?;
        
        Ok(Statement::ExportDeclaration(Box::new(ExportDeclaration {
            declaration: Box::new(Statement::ExpressionStatement(ExpressionStatement {
                expression: Expression::Literal(Literal::String(format!("Export: {}", exports.join(", ")))),
            })),
        })))
    }

    fn parse_export_type_statement(&mut self) -> Result<Statement> {
        self.expect_keyword()?; // consume 'type' keyword
        self.expect_token(&Token::LeftBrace)?; // consume '{'
        
        let mut type_names = Vec::new();
        while self.current_token() != &Token::RightBrace && self.current_token() != &Token::EOF {
            let name = self.expect_identifier()?;
            type_names.push(name);
            
            if self.current_token() == &Token::Comma {
                self.advance();
            }
        }
        
        self.expect_token(&Token::RightBrace)?; // consume '}'
        self.expect_semicolon()?;
        
        // For now, create a simple export statement
        // In a full implementation, we'd have a proper ExportTypeStatement type
        Ok(Statement::ExportDeclaration(Box::new(ExportDeclaration {
            declaration: Box::new(Statement::TypeAlias(TypeAlias {
                name: "exported_types".to_string(),
                type_parameters: Vec::new(),
                type_definition: Type::Any,
            })),
        })))
    }

    /// Parse namespace declaration
    fn parse_namespace_declaration(&mut self) -> Result<Statement> {
        self.expect_keyword()?; // namespace
        let name = self.expect_identifier()?;
        let body = self.parse_block_statement()?;

        Ok(Statement::NamespaceDeclaration(NamespaceDeclaration {
            name,
            body: Box::new(body),
        }))
    }

    /// Parse module declaration
    fn parse_module_declaration(&mut self) -> Result<Statement> {
        self.expect_keyword()?; // module
        let name = self.parse_string_literal()?;
        let body = self.parse_block_statement()?;

        Ok(Statement::ModuleDeclaration(ModuleDeclaration {
            name,
            body: Box::new(body),
        }))
    }

    /// Parse declare statement
    fn parse_declare_statement(&mut self) -> Result<Statement> {
        self.expect_keyword()?; // declare
        let declaration = self.parse_statement()?;
        Ok(Statement::DeclareStatement(Box::new(DeclareStatement {
            declaration: Box::new(declaration.unwrap()),
        })))
    }

    /// Parse return statement
    fn parse_return_statement(&mut self) -> Result<Statement> {
        self.expect_keyword()?; // return

        let argument = if self.current_token() == &Token::Semicolon {
            None
        } else {
            Some(self.parse_expression()?)
        };

        // Optional semicolon
        if self.current_token() == &Token::Semicolon {
            self.advance();
        }

        Ok(Statement::ReturnStatement(ReturnStatement { argument }))
    }

    /// Parse throw statement
    fn parse_throw_statement(&mut self) -> Result<Statement> {
        self.expect_keyword()?; // throw

        let argument = self.parse_expression()?;

        // Optional semicolon
        if self.current_token() == &Token::Semicolon {
            self.advance();
        }

        Ok(Statement::ThrowStatement(ThrowStatement { argument }))
    }

    /// Parse expression statement
    fn parse_expression_statement(&mut self) -> Result<Statement> {
        let expression = self.parse_expression()?;
        self.expect_semicolon()?;
        Ok(Statement::ExpressionStatement(ExpressionStatement {
            expression,
        }))
    }

    /// Parse block statement
    fn parse_block_statement(&mut self) -> Result<Statement> {
        self.expect_token(&Token::LeftBrace)?;
        let mut statements = Vec::new();

        while self.current_token() != &Token::RightBrace {
            if let Some(statement) = self.parse_statement()? {
                statements.push(statement);
            } else {
                break;
            }
        }

        self.expect_token(&Token::RightBrace)?;
        Ok(Statement::BlockStatement(BlockStatement { statements }))
    }

    /// Parse expression
    fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_assignment_expression()
    }

    /// Parse assignment expression
    fn parse_assignment_expression(&mut self) -> Result<Expression> {
        let left = self.parse_conditional_expression()?;

        if self.is_assignment_operator() {
            let operator = self.current_token().clone();
            self.advance();
            let right = self.parse_assignment_expression()?;
            Ok(Expression::Assignment(AssignmentExpression {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            }))
        } else {
            Ok(left)
        }
    }

    /// Parse conditional expression
    fn parse_conditional_expression(&mut self) -> Result<Expression> {
        let test = self.parse_logical_or_expression()?;

        if self.current_token() == &Token::QuestionMark {
            self.advance();
            let consequent = self.parse_expression()?;
            self.expect_token(&Token::Colon)?;
            let alternate = self.parse_expression()?;
            Ok(Expression::Conditional(ConditionalExpression {
                test: Box::new(test),
                consequent: Box::new(consequent),
                alternate: Box::new(alternate),
            }))
        } else {
            Ok(test)
        }
    }

    /// Parse logical OR expression
    fn parse_logical_or_expression(&mut self) -> Result<Expression> {
        let mut left = self.parse_logical_and_expression()?;

        while self.current_token() == &Token::Or {
            self.advance();
            let right = self.parse_logical_and_expression()?;
            left = Expression::Logical(LogicalExpression {
                left: Box::new(left),
                operator: Token::Or,
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    /// Parse logical AND expression
    fn parse_logical_and_expression(&mut self) -> Result<Expression> {
        let mut left = self.parse_equality_expression()?;

        while self.current_token() == &Token::And {
            self.advance();
            let right = self.parse_equality_expression()?;
            left = Expression::Logical(LogicalExpression {
                left: Box::new(left),
                operator: Token::And,
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    /// Parse equality expression
    fn parse_equality_expression(&mut self) -> Result<Expression> {
        let mut left = self.parse_relational_expression()?;

        while self.is_equality_operator() {
            let operator = self.current_token().clone();
            self.advance();
            let right = self.parse_relational_expression()?;
            left = Expression::Binary(BinaryExpression {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    /// Parse relational expression
    fn parse_relational_expression(&mut self) -> Result<Expression> {
        let mut left = self.parse_additive_expression()?;

        while self.is_relational_operator() {
            let operator = self.current_token().clone();
            self.advance();
            let right = self.parse_additive_expression()?;
            left = Expression::Binary(BinaryExpression {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    /// Parse additive expression
    fn parse_additive_expression(&mut self) -> Result<Expression> {
        let mut left = self.parse_multiplicative_expression()?;

        while self.is_additive_operator() {
            let operator = self.current_token().clone();
            self.advance();
            let right = self.parse_multiplicative_expression()?;
            left = Expression::Binary(BinaryExpression {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    /// Parse multiplicative expression
    fn parse_multiplicative_expression(&mut self) -> Result<Expression> {
        let mut left = self.parse_unary_expression()?;

        while self.is_multiplicative_operator() {
            let operator = self.current_token().clone();
            self.advance();
            let right = self.parse_unary_expression()?;
            left = Expression::Binary(BinaryExpression {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    /// Parse unary expression
    fn parse_unary_expression(&mut self) -> Result<Expression> {
        if self.is_unary_operator() {
            let operator = self.current_token().clone();
            self.advance();
            let argument = self.parse_unary_expression()?;
            Ok(Expression::Unary(UnaryExpression {
                operator,
                argument: Box::new(argument),
            }))
        } else {
            self.parse_postfix_expression()
        }
    }

    /// Parse postfix expression
    fn parse_postfix_expression(&mut self) -> Result<Expression> {
        let mut left = self.parse_primary_expression()?;

        while self.is_postfix_operator() {
            match self.current_token() {
                Token::LeftParen => {
                    self.advance();
                    let arguments = self.parse_arguments()?;
                    self.expect_token(&Token::RightParen)?;
                    left = Expression::Call(CallExpression {
                        callee: Box::new(left),
                        arguments,
                    });
                }
                Token::LeftBracket => {
                    self.advance();
                    let index = self.parse_expression()?;
                    self.expect_token(&Token::RightBracket)?;
                    left = Expression::Member(MemberExpression {
                        object: Box::new(left),
                        property: Box::new(index),
                        computed: true,
                    });
                }
                Token::Dot => {
                    self.advance();
                    let property = self.expect_identifier()?;
                    left = Expression::Member(MemberExpression {
                        object: Box::new(left),
                        property: Box::new(Expression::Identifier(property)),
                        computed: false,
                    });
                }
                _ => break,
            }
        }

        Ok(left)
    }

    /// Parse primary expression
    fn parse_primary_expression(&mut self) -> Result<Expression> {
        let token = self.current_token().clone();
        match token {
            Token::Number(n) => {
                self.advance();
                Ok(Expression::Literal(Literal::Number(n)))
            }
            Token::String(s) => {
                self.advance();
                Ok(Expression::Literal(Literal::String(s)))
            }
            Token::RegExp(pattern, flags) => {
                self.advance();
                Ok(Expression::Literal(Literal::RegExp(pattern.clone(), flags.clone())))
            }
            Token::TemplateLiteral(s) => {
                self.advance();
                // Create a simple template literal with one quasi
                let template = TemplateLiteral {
                    quasis: vec![TemplateElement {
                        value: s,
                        tail: true,
                    }],
                    expressions: vec![],
                };
                Ok(Expression::Template(template))
            }
            Token::Boolean(b) => {
                self.advance();
                Ok(Expression::Literal(Literal::Boolean(b)))
            }
            Token::Null => {
                self.advance();
                Ok(Expression::Literal(Literal::Null))
            }
            Token::Undefined => {
                self.advance();
                Ok(Expression::Literal(Literal::Undefined))
            }
            Token::Keyword(crate::lexer::Keyword::Null) => {
                self.advance();
                Ok(Expression::Literal(Literal::Null))
            }
            Token::Keyword(crate::lexer::Keyword::Undefined) => {
                self.advance();
                Ok(Expression::Literal(Literal::Undefined))
            }
            Token::Identifier(name) => {
                self.advance();
                Ok(Expression::Identifier(name))
            }
            Token::Keyword(crate::lexer::Keyword::This) => {
                self.advance();
                // Check for dot notation: this.prop
                if self.current_token() == &Token::Dot {
                    self.advance(); // consume '.'
                    let property = self.expect_identifier()?;
                    Ok(Expression::Member(MemberExpression {
                        object: Box::new(Expression::This(ThisExpression)),
                        property: Box::new(Expression::Identifier(property)),
                        computed: false,
                    }))
                } else {
                    Ok(Expression::This(ThisExpression))
                }
            }
            Token::Keyword(crate::lexer::Keyword::Super) => {
                self.advance();
                Ok(Expression::Super(SuperExpression))
            }
            Token::Keyword(crate::lexer::Keyword::New) => {
                self.advance();
                let callee = self.parse_primary_expression()?;
                let arguments = if self.current_token() == &Token::LeftParen {
                    self.advance(); // consume '('
                    let args = self.parse_arguments()?;
                    self.expect_token(&Token::RightParen)?;
                    args
                } else {
                    Vec::new()
                };
                Ok(Expression::New(NewExpression {
                    callee: Box::new(callee),
                    arguments,
                }))
            }
            Token::LeftParen => {
                // Look ahead to see if this is an arrow function
                let mut pos = self.position + 1;
                let mut paren_count = 1;
                
                // Skip to matching closing paren
                while pos < self.tokens.len() && paren_count > 0 {
                    match &self.tokens[pos] {
                        Token::LeftParen => paren_count += 1,
                        Token::RightParen => paren_count -= 1,
                        _ => {}
                    }
                    pos += 1;
                }
                
                // Check if next token is arrow
                if pos < self.tokens.len() && self.tokens[pos] == Token::Arrow {
                    // This is an arrow function
                    self.advance(); // consume (
                    let parameters = self.parse_parameter_list()?;
                    self.expect_token(&Token::RightParen)?;
                    self.expect_token(&Token::Arrow)?;
                    let body = if self.current_token() == &Token::LeftBrace {
                        self.parse_block_statement()?
                    } else {
                        let expr = self.parse_expression()?;
                        Statement::ExpressionStatement(ExpressionStatement {
                            expression: expr,
                        })
                    };
                    
                    Ok(Expression::Arrow(Box::new(ArrowFunctionExpression {
                        type_parameters: Vec::new(),
                        parameters,
                        return_type: None,
                        body: Box::new(body),
                    })))
                } else {
                    // Regular parenthesized expression
                    self.advance();
                    let expression = self.parse_expression()?;
                    self.expect_token(&Token::RightParen)?;
                    Ok(Expression::Parenthesized(ParenthesizedExpression {
                        expression: Box::new(expression),
                    }))
                }
            }
            Token::LeftBrace => self.parse_object_expression(),
            Token::LeftBracket => self.parse_array_expression(),
            _ => Err(CompilerError::parse_error(
                self.position,
                0,
                format!("Unexpected token: {:?}", self.current_token()),
            )),
        }
    }

    /// Parse object expression
    fn parse_object_expression(&mut self) -> Result<Expression> {
        self.expect_token(&Token::LeftBrace)?;
        let mut properties = Vec::new();

        while self.current_token() != &Token::RightBrace {
            let key = self.parse_property_key()?;
            let value = if self.current_token() == &Token::Colon {
                self.advance();
                self.parse_expression()?
            } else {
                key.clone()
            };

            properties.push(ObjectProperty {
                key,
                value,
                shorthand: false,
                computed: false,
                method: false,
            });

            if self.current_token() == &Token::Comma {
                self.advance();
            }
        }

        self.expect_token(&Token::RightBrace)?;
        Ok(Expression::Object(ObjectExpression { properties }))
    }

    /// Parse array expression
    fn parse_array_expression(&mut self) -> Result<Expression> {
        self.expect_token(&Token::LeftBracket)?;
        let mut elements = Vec::new();

        while self.current_token() != &Token::RightBracket {
            if self.current_token() == &Token::Comma {
                self.advance();
                elements.push(None);
            } else {
                elements.push(Some(self.parse_expression()?));
                if self.current_token() == &Token::Comma {
                    self.advance();
                }
            }
        }

        self.expect_token(&Token::RightBracket)?;
        Ok(Expression::Array(ArrayExpression { elements }))
    }

    /// Parse type
    fn parse_type(&mut self) -> Result<Type> {
        let mut left_type = self.parse_primary_type()?;
        
        // Handle union and intersection types
        while matches!(self.current_token(), Token::Union | Token::Intersection) {
            let operator = self.current_token().clone();
            self.advance();
            let right_type = self.parse_primary_type()?;
            
            left_type = match operator {
                Token::Union => Type::Union {
                    left: Box::new(left_type),
                    right: Box::new(right_type),
                },
                Token::Intersection => Type::Intersection {
                    left: Box::new(left_type),
                    right: Box::new(right_type),
                },
                _ => return Err(CompilerError::parse_error(
                    1,
                    1,
                    "Expected union or intersection operator",
                )),
            };
        }
        
        // Handle array types: T[]
        while self.current_token() == &Token::LeftBracket {
            self.advance(); // consume [
            self.expect_token(&Token::RightBracket)?; // consume ]
            left_type = Type::Array(Box::new(left_type));
        }
        
        Ok(left_type)
    }
    
    fn parse_primary_type(&mut self) -> Result<Type> {
        let token = self.current_token().clone();
        match token {
            Token::Keyword(crate::lexer::Keyword::String) => {
                self.advance();
                Ok(Type::String)
            }
            Token::Keyword(crate::lexer::Keyword::Number) => {
                self.advance();
                Ok(Type::Number)
            }
            Token::Keyword(crate::lexer::Keyword::Boolean) => {
                self.advance();
                Ok(Type::Boolean)
            }
            Token::Keyword(crate::lexer::Keyword::Any) => {
                self.advance();
                Ok(Type::Any)
            }
            Token::Keyword(crate::lexer::Keyword::Void) => {
                self.advance();
                Ok(Type::Void)
            }
            Token::Keyword(crate::lexer::Keyword::Never) => {
                self.advance();
                Ok(Type::Never)
            }
            Token::Keyword(crate::lexer::Keyword::Unknown) => {
                self.advance();
                Ok(Type::Unknown)
            }
            Token::Keyword(crate::lexer::Keyword::Array) => {
                self.advance();
                if self.current_token() == &Token::LessThan {
                    self.advance(); // consume <
                    let element_type = self.parse_primary_type()?;
                    self.expect_token(&Token::GreaterThan)?;
                    Ok(Type::Array(Box::new(element_type)))
                } else {
                    Ok(Type::Array(Box::new(Type::Any)))
                }
            }
            Token::Keyword(crate::lexer::Keyword::Readonly) => {
                self.advance();
                // Parse the type that follows readonly
                let element_type = self.parse_primary_type()?;
                // For now, just return the element type (readonly is handled at runtime)
                Ok(element_type)
            }
            Token::Keyword(crate::lexer::Keyword::Keyof) => {
                self.advance();
                let _target_type = self.parse_type()?;
                // For now, return string as keyof T resolves to string
                Ok(Type::String)
            }
            Token::Keyword(crate::lexer::Keyword::Key) => {
                self.advance();
                Ok(Type::String) // Key -> string for now
            }
            Token::Keyword(crate::lexer::Keyword::Infer) => {
                self.advance();
                Ok(Type::Any) // infer -> any for now
            }
            Token::Keyword(crate::lexer::Keyword::Null) => {
                self.advance();
                Ok(Type::Null) // null -> null for now
            }
            Token::Keyword(crate::lexer::Keyword::Undefined) => {
                self.advance();
                Ok(Type::Undefined) // undefined -> undefined for now
            }
            Token::Null => {
                self.advance();
                Ok(Type::Null)
            }
            Token::Undefined => {
                self.advance();
                Ok(Type::Undefined)
            }
            Token::Identifier(name) => {
                // First, parse the base type (could be array type, generic type, etc.)
                let base_type = if self.current_token() == &Token::LessThan {
                    // Parse generic type
                    self.advance(); // consume <
                    let mut type_args = Vec::new();

                    while self.current_token() != &Token::GreaterThan && self.current_token() != &Token::EOF {
                        let arg = self.parse_type()?;
                        type_args.push(arg);

                        if self.current_token() == &Token::Comma {
                            self.advance(); // consume ,
                        } else {
                            break;
                        }
                    }

                    self.expect_token(&Token::GreaterThan)?; // consume >

                    self.advance(); // consume the identifier token
                    Type::GenericNamed {
                        name: name.to_string(),
                        type_arguments: type_args,
                    }
                } else {
                    self.advance(); // consume the identifier token
                    Type::Named(name.to_string())
                };

                // Then check for array brackets
                if self.current_token() == &Token::LeftBracket {
                    self.advance(); // consume [
                    self.expect_token(&Token::RightBracket)?; // consume ]
                    Ok(Type::Array(Box::new(base_type)))
                } else {
                    Ok(base_type)
                }
            }
            Token::String(s) => {
                self.advance();
                // String literal type
                Ok(Type::Named(format!("\"{}\"", s)))
            }
            Token::Number(n) => {
                self.advance();
                // Number literal type
                Ok(Type::Named(n.to_string()))
            }
            Token::LeftParen => {
                self.advance();
                let type_ = self.parse_type()?;
                self.expect_token(&Token::RightParen)?;
                Ok(type_)
            }
       Token::LeftBrace => {
           // Parse object type: { prop: type; ... } or mapped type { [P in K]: T }
           self.advance(); // consume {
           let mut members = Vec::new();

           while self.current_token() != &Token::RightBrace && self.current_token() != &Token::EOF {
               // Check if this is a mapped type: [P in K] or index signature: [key: type]
               if self.current_token() == &Token::LeftBracket {
                   // Look ahead to determine if this is a mapped type or index signature
                   let mut pos = self.position + 1; // skip [
                   let mut is_mapped_type = false;
                   
                   // Look for 'in' keyword to distinguish mapped type from index signature
                   while pos < self.tokens.len() && self.tokens[pos] != Token::RightBracket {
                       if self.tokens[pos] == Token::Keyword(Keyword::In) {
                           is_mapped_type = true;
                           break;
                       }
                       pos += 1;
                   }
                   
                   if is_mapped_type {
                       let mapped_type = self.parse_mapped_type()?;
                       members.push(ObjectTypeMember::Property(PropertySignature {
                           name: mapped_type.type_parameter.name.clone(),
                           optional: false,
                           type_: Some(*mapped_type.type_.clone()),
                           readonly: mapped_type.readonly.unwrap_or(false),
                       }));
                       // parse_mapped_type already handles semicolon, so continue to next iteration
                       continue;
                   } else {
                       // This is an index signature, parse it as such
                       let index_sig = self.parse_index_signature()?;
                       members.push(ObjectTypeMember::Index(index_sig));
                       if self.current_token() == &Token::Semicolon {
                           self.advance();
                       }
                       continue;
                   }
               } else {
                   // Check for readonly modifier
                   let readonly = if self.current_token() == &Token::Keyword(Keyword::Readonly) {
                       self.advance(); // consume readonly
                       true
                   } else {
                       false
                   };

                   let name = self.expect_identifier()?;
                   let optional = if self.current_token() == &Token::QuestionMark {
                       self.advance();
                       true
                   } else {
                       false
                   };
                   self.expect_token(&Token::Colon)?;
                   let type_ = self.parse_type()?;

                   members.push(ObjectTypeMember::Property(PropertySignature {
                       name,
                       optional,
                       type_: Some(type_),
                       readonly,
                   }));
               }

               if self.current_token() == &Token::Semicolon {
                   self.advance();
               }
           }

           self.expect_token(&Token::RightBrace)?;
           Ok(Type::ObjectType(ObjectType { members }))
       }
            _ => Err(CompilerError::parse_error(
                self.position,
                0,
                format!("Unexpected token in type: {:?}", self.current_token()),
            )),
        }
    }

    // Helper methods
    fn current_token(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    fn expect_token(&mut self, expected: &Token) -> Result<()> {
        if self.current_token() == expected {
            self.advance();
            Ok(())
        } else {
            Err(CompilerError::parse_error(
                self.position,
                0,
                format!("Expected {:?}, found {:?}", expected, self.current_token()),
            ))
        }
    }

    fn expect_keyword(&mut self) -> Result<crate::lexer::Keyword> {
        if let Token::Keyword(keyword) = self.current_token() {
            let keyword = keyword.clone();
            self.advance();
            Ok(keyword)
        } else {
            Err(CompilerError::parse_error(
                self.position,
                0,
                format!("Expected keyword, found {:?}", self.current_token()),
            ))
        }
    }

    fn expect_identifier(&mut self) -> Result<String> {
        if let Token::Identifier(name) = self.current_token() {
            let name = name.clone();
            self.advance();
            Ok(name)
        } else {
            Err(CompilerError::parse_error(
                self.position,
                0,
                format!("Expected identifier, found {:?}", self.current_token()),
            ))
        }
    }

    fn expect_semicolon(&mut self) -> Result<()> {
        self.expect_token(&Token::Semicolon)
    }

    fn parse_string_literal(&mut self) -> Result<String> {
        if let Token::String(s) = self.current_token() {
            let s = s.clone();
            self.advance();
            Ok(s)
        } else {
            Err(CompilerError::parse_error(
                self.position,
                0,
                format!("Expected string literal, found {:?}", self.current_token()),
            ))
        }
    }

    fn is_assignment_operator(&self) -> bool {
        matches!(
            self.current_token(),
            Token::Assign
                | Token::PlusAssign
                | Token::MinusAssign
                | Token::MultiplyAssign
                | Token::DivideAssign
        )
    }

    fn is_equality_operator(&self) -> bool {
        matches!(
            self.current_token(),
            Token::Equal | Token::NotEqual | Token::StrictEqual | Token::StrictNotEqual
        )
    }

    fn is_relational_operator(&self) -> bool {
        matches!(
            self.current_token(),
            Token::LessThan | Token::GreaterThan | Token::LessEqual | Token::GreaterEqual
        )
    }

    fn is_additive_operator(&self) -> bool {
        matches!(self.current_token(), Token::Plus | Token::Minus)
    }

    fn is_multiplicative_operator(&self) -> bool {
        matches!(
            self.current_token(),
            Token::Multiply | Token::Divide | Token::Modulo
        )
    }

    fn is_unary_operator(&self) -> bool {
        matches!(
            self.current_token(),
            Token::Plus | Token::Minus | Token::Not | Token::Keyword(crate::lexer::Keyword::Typeof)
        )
    }

    fn is_postfix_operator(&self) -> bool {
        matches!(
            self.current_token(),
            Token::LeftParen | Token::LeftBracket | Token::Dot
        )
    }

    // Placeholder methods for complex parsing
    fn parse_type_parameters(&mut self) -> Result<Vec<TypeParameter>> {
        if self.current_token() == &Token::LessThan {
            self.advance();
            let mut type_parameters = Vec::new();

            while self.current_token() != &Token::GreaterThan && self.current_token() != &Token::EOF {
                let name = self.expect_identifier()?;

                let constraint =
                    if self.current_token() == &Token::Keyword(crate::lexer::Keyword::Extends) {
                        self.advance();
                        Some(self.parse_type()?)
                    } else {
                        None
                    };

                let default_type = if self.current_token() == &Token::Assign {
                    self.advance();
                    Some(self.parse_type()?)
                } else {
                    None
                };

                type_parameters.push(TypeParameter {
                    name,
                    constraint: constraint.map(Box::new),
                    default: default_type.map(Box::new),
                });

                if self.current_token() == &Token::Comma {
                    self.advance();
                } else {
                    break;
                }
            }

            self.expect_token(&Token::GreaterThan)?;
            Ok(type_parameters)
        } else {
            Ok(Vec::new())
        }
    }

    fn parse_parameters(&mut self) -> Result<Vec<Parameter>> {
        self.expect_token(&Token::LeftParen)?;
        let mut parameters = Vec::new();

        while self.current_token() != &Token::RightParen {
            // Handle access modifiers on parameters (TypeScript feature)
            let mut _modifiers = Vec::new();
            while let Token::Keyword(keyword) = self.current_token() {
                match keyword {
                    crate::lexer::Keyword::Public | 
                    crate::lexer::Keyword::Private | 
                    crate::lexer::Keyword::Protected | 
                    crate::lexer::Keyword::Readonly => {
                        _modifiers.push(keyword.clone());
                        self.advance();
                    }
                    _ => break,
                }
            }
            
            let name = self.expect_identifier()?;
            let optional = if self.current_token() == &Token::QuestionMark {
                self.advance();
                true
            } else {
                false
            };

            let type_annotation = if self.current_token() == &Token::Colon {
                self.advance();
                Some(self.parse_type()?)
            } else {
                None
            };

            let initializer = if self.current_token() == &Token::Assign {
                self.advance();
                Some(self.parse_expression()?)
            } else {
                None
            };

            parameters.push(Parameter {
                name,
                optional,
                type_: type_annotation.map(Box::new),
                initializer,
                rest: false,
            });

            if self.current_token() == &Token::Comma {
                self.advance();
            }
        }

        self.expect_token(&Token::RightParen)?;
        Ok(parameters)
    }
    
    fn parse_parameter_list(&mut self) -> Result<Vec<Parameter>> {
        let mut parameters = Vec::new();

        while self.current_token() != &Token::RightParen {
            // Handle access modifiers on parameters (TypeScript feature)
            let mut _modifiers = Vec::new();
            while let Token::Keyword(keyword) = self.current_token() {
                match keyword {
                    crate::lexer::Keyword::Public | 
                    crate::lexer::Keyword::Private | 
                    crate::lexer::Keyword::Protected | 
                    crate::lexer::Keyword::Readonly => {
                        _modifiers.push(keyword.clone());
                        self.advance();
                    }
                    _ => break,
                }
            }
            
            let name = self.expect_identifier()?;
            let optional = if self.current_token() == &Token::QuestionMark {
                self.advance();
                true
            } else {
                false
            };

            let type_annotation = if self.current_token() == &Token::Colon {
                self.advance();
                Some(self.parse_type()?)
            } else {
                None
            };

            let initializer = if self.current_token() == &Token::Assign {
                self.advance();
                Some(self.parse_expression()?)
            } else {
                None
            };

            parameters.push(Parameter {
                name,
                optional,
                type_: type_annotation.map(Box::new),
                initializer,
                rest: false,
            });

            if self.current_token() == &Token::Comma {
                self.advance();
            }
        }

        Ok(parameters)
    }

    fn parse_implements(&mut self) -> Result<Vec<Type>> {
        if self.current_token() == &Token::Keyword(crate::lexer::Keyword::Implements) {
            self.advance();
            let mut types = Vec::new();

            loop {
                let type_ = self.parse_type()?;
                types.push(type_);

                if self.current_token() == &Token::Comma {
                    self.advance();
                } else {
                    break;
                }
            }

            Ok(types)
        } else {
            Ok(Vec::new())
        }
    }

    fn parse_extends(&mut self) -> Result<Vec<Type>> {
        let mut extends = Vec::new();
        
        if self.current_token() == &Token::Keyword(crate::lexer::Keyword::Extends) {
            self.advance(); // consume 'extends'
            
            // Parse the first extended type
            let type_ = self.parse_type()?;
            extends.push(type_);
            
            // Parse additional extended types (comma-separated)
            while self.current_token() == &Token::Comma {
                self.advance(); // consume ','
                let type_ = self.parse_type()?;
                extends.push(type_);
            }
        }
        
        Ok(extends)
    }

    fn parse_class_body(&mut self) -> Result<ClassBody> {
        self.expect_token(&Token::LeftBrace)?;
        let mut members = Vec::new();

        while self.current_token() != &Token::RightBrace && self.current_token() != &Token::EOF {
            let member = self.parse_class_member()?;
            members.push(member.clone());

            // Handle decorator-only members
            if let ClassMember::Decorator(_) = &member {
                continue;
            }
        }

        self.expect_token(&Token::RightBrace)?;
        Ok(ClassBody { members })
    }

    fn parse_interface_body(&mut self) -> Result<InterfaceBody> {
        self.expect_token(&Token::LeftBrace)?;
        let mut members = Vec::new();

        while self.current_token() != &Token::RightBrace && self.current_token() != &Token::EOF {
            let member = self.parse_interface_member()?;
            members.push(member);
        }
        self.expect_token(&Token::RightBrace)?;
        Ok(InterfaceBody { members })
    }

    fn parse_enum_members(&mut self) -> Result<Vec<EnumMember>> {
        self.expect_token(&Token::LeftBrace)?;
        let mut members = Vec::new();

        while self.current_token() != &Token::RightBrace && self.current_token() != &Token::EOF {
            let member = self.parse_enum_member()?;
            members.push(member);

            if self.current_token() == &Token::Comma {
                self.advance();
            }
        }

        self.expect_token(&Token::RightBrace)?;
        Ok(members)
    }

    fn parse_import_specifiers(&mut self) -> Result<Vec<ImportSpecifier>> {
        let mut specifiers = Vec::new();

        if self.current_token() == &Token::LeftBrace {
            self.advance(); // consume '{'

            while self.current_token() != &Token::RightBrace {
                let name = self.expect_identifier()?;
                specifiers.push(ImportSpecifier::Named(NamedImportSpecifier {
                    imported: name.clone(),
                    name,
                }));

                if self.current_token() == &Token::Comma {
                    self.advance();
                }
            }

            self.expect_token(&Token::RightBrace)?; // consume '}'
        } else {
            // Default import
            let name = self.expect_identifier()?;
            specifiers.push(ImportSpecifier::Default(DefaultImportSpecifier { name }));
        }

        Ok(specifiers)
    }

    fn parse_arguments(&mut self) -> Result<Vec<Expression>> {
        let mut arguments = Vec::new();

        while self.current_token() != &Token::RightParen {
            let argument = self.parse_expression()?;
            arguments.push(argument);

            if self.current_token() == &Token::Comma {
                self.advance();
            } else if self.current_token() != &Token::RightParen {
                return Err(CompilerError::parse_error(
                    1,
                    1,
                    "Expected comma or closing parenthesis".to_string(),
                ));
            }
        }

        Ok(arguments)
    }

    fn parse_class_member(&mut self) -> Result<ClassMember> {
        let mut modifiers = Vec::new();
        let mut decorators = Vec::new();

        // Parse decorators first
        while self.current_token() == &Token::At {
            self.advance(); // consume @
            let decorator_name = self.expect_identifier()?;
            decorators.push(decorator_name);

            // Skip arguments for now (e.g., @log())
            if self.current_token() == &Token::LeftParen {
                self.advance(); // consume (
                // Skip arguments until closing paren
                let mut paren_count = 1;
                while paren_count > 0 && self.position < self.tokens.len() {
                    match self.current_token() {
                        Token::LeftParen => paren_count += 1,
                        Token::RightParen => paren_count -= 1,
                        _ => {}
                    }
                    self.advance();
                }
            }
        }

        // Parse access modifiers
        while let Token::Keyword(keyword) = self.current_token() {
            match keyword {
                crate::lexer::Keyword::Public => {
                    modifiers.push(crate::ast::Modifier::Public);
                    self.advance();
                }
                crate::lexer::Keyword::Private => {
                    modifiers.push(crate::ast::Modifier::Private);
                    self.advance();
                }
                crate::lexer::Keyword::Protected => {
                    modifiers.push(crate::ast::Modifier::Protected);
                    self.advance();
                }
                crate::lexer::Keyword::Readonly => {
                    modifiers.push(crate::ast::Modifier::Readonly);
                    self.advance();
                }
                crate::lexer::Keyword::Static => {
                    modifiers.push(crate::ast::Modifier::Static);
                    self.advance();
                }
                _ => break,
            }
        }

        // If we have decorators but no following member, return just the decorator
        if !decorators.is_empty() && self.position >= self.tokens.len() - 1 {
            return Ok(ClassMember::Decorator(decorators[0].clone()));
        }

        let token = self.current_token().clone();

        match token {
            Token::Keyword(crate::lexer::Keyword::Constructor) => {
                self.advance();
                let parameters = self.parse_parameters()?;
                let body = self.parse_block_statement()?;

                Ok(ClassMember::Constructor(ConstructorDeclaration {
                    parameters,
                    body: Some(body),
                    modifiers,
                    decorators: decorators.clone(),
                }))
            }
            Token::Keyword(crate::lexer::Keyword::Get) => {
                self.advance();
                let name = if let Token::Identifier(name) = self.current_token() {
                    let name = name.clone();
                    self.advance();
                    name
                } else {
                    return Err(CompilerError::parse_error(
                        1,
                        1,
                        "Expected getter name".to_string(),
                    ));
                };

                // Handle getter parameters (empty parentheses)
                if self.current_token() == &Token::LeftParen {
                    self.advance(); // consume '('
                    self.expect_token(&Token::RightParen)?; // consume ')'
                }

                let return_type = if self.current_token() == &Token::Colon {
                    self.advance();
                    Some(self.parse_type()?)
                } else {
                    None
                };
                let body = if self.current_token() == &Token::LeftBrace {
                    self.parse_block_statement()?
                } else {
                    return Err(CompilerError::parse_error(
                        1,
                        1,
                        "Expected block statement for getter".to_string(),
                    ));
                };

                Ok(ClassMember::Getter(GetterDeclaration {
                    name,
                    type_: return_type,
                    body: Some(body),
                    modifiers,
                    decorators,
                }))
            }
            Token::Keyword(crate::lexer::Keyword::Set) => {
                self.advance();
                let name = if let Token::Identifier(name) = self.current_token() {
                    let name = name.clone();
                    self.advance();
                    name
                } else {
                    return Err(CompilerError::parse_error(
                        1,
                        1,
                        "Expected setter name".to_string(),
                    ));
                };

                let parameter = if self.current_token() == &Token::LeftParen {
                    self.advance(); // consume '('
                    let name = self.expect_identifier()?;
                    self.expect_token(&Token::Colon)?;
                    let type_annotation = self.parse_type()?;
                    self.expect_token(&Token::RightParen)?; // consume ')'
                    
                    Parameter {
                        name,
                        optional: false,
                        type_: Some(Box::new(type_annotation)),
                        initializer: None,
                        rest: false,
                    }
                } else {
                    return Err(CompilerError::parse_error(
                        1,
                        1,
                        "Expected setter parameter".to_string(),
                    ));
                };

                let body = if self.current_token() == &Token::LeftBrace {
                    self.parse_block_statement()?
                } else {
                    return Err(CompilerError::parse_error(
                        1,
                        1,
                        "Expected block statement for setter".to_string(),
                    ));
                };

                Ok(ClassMember::Setter(SetterDeclaration {
                    name,
                    parameter,
                    body: Some(body),
                    modifiers,
                    decorators,
                }))
            }
            Token::Identifier(name) => {
                self.advance();

                // Special handling for constructor
                if name == "constructor" {
                    println!("DEBUG: parsing constructor");
                    // It's a constructor
                    let parameters = self.parse_parameters()?;
                    let body = self.parse_block_statement()?;

                    Ok(ClassMember::Constructor(ConstructorDeclaration {
                        parameters,
                        body: Some(body),
                        modifiers,
                        decorators: decorators.clone(),
                    }))
                } else if self.current_token() == &Token::LeftParen {
                    // It's a method
                    let parameters = self.parse_parameters()?;
                    let return_type = if self.current_token() == &Token::Colon {
                        self.advance();
                        Some(self.parse_type()?)
                    } else {
                        None
                    };
                    let body = self.parse_block_statement()?;

                    Ok(ClassMember::Method(MethodDeclaration {
                        name,
                        optional: false,
                        type_parameters: Vec::new(),
                        parameters,
                        return_type,
                        body: Some(body),
                        modifiers,
                        decorators,
                    }))
                } else if self.current_token() == &Token::Colon {
                    // It's a property
                    self.advance();
                    let type_annotation = self.parse_type()?;
                    
                    let initializer = if self.current_token() == &Token::Assign {
                        self.advance();
                        Some(self.parse_expression()?)
                    } else {
                        None
                    };
                    
                    self.expect_token(&Token::Semicolon)?;

                    Ok(ClassMember::Property(PropertyDeclaration {
                        name,
                        optional: false,
                        type_: Some(type_annotation),
                        initializer,
                        modifiers,
                        decorators,
                    }))
                } else {
                    // It's a constructor
                    if name == "constructor" {
                        let parameters = self.parse_parameters()?;
                        let body = self.parse_block_statement()?;

                    Ok(ClassMember::Constructor(ConstructorDeclaration {
                        parameters,
                        body: Some(body),
                        modifiers: Vec::new(),
                        decorators: Vec::new(),
                    }))
                    } else {
                        // If we can't parse as class member, try to skip the token
                        self.advance();
                        Err(CompilerError::parse_error(
                            self.position - 1,
                            1,
                            "Unexpected class member, skipping token".to_string(),
                        ))
                    }
                }
            }
            _ => {
                // If we can't parse as class member, try to skip the token
                self.advance();
                Err(CompilerError::parse_error(
                    self.position - 1,
                    1,
                    "Expected class member, skipping token".to_string(),
                ))
            }
        }
    }

    fn parse_interface_member(&mut self) -> Result<ObjectTypeMember> {
        let mut readonly = false;

        // Check for readonly modifier
        if let Token::Keyword(crate::lexer::Keyword::Readonly) = self.current_token() {
            readonly = true;
            self.advance();
            
            // Check if the next token is also 'readonly' (property name)
            if let Token::Keyword(crate::lexer::Keyword::Readonly) = self.current_token() {
                // Handle case where readonly is both modifier and property name: readonly readonly: boolean;
                let name = "readonly".to_string();
                self.advance(); // consume the property name 'readonly'
                
                if self.current_token() == &Token::Colon {
                    // It's a property signature
                    self.advance();
                    let type_annotation = self.parse_type()?;
                    self.expect_token(&Token::Semicolon)?;

                    return Ok(ObjectTypeMember::Property(PropertySignature {
                        name,
                        optional: false,
                        type_: Some(type_annotation),
                        readonly,
                    }));
                } else {
                    return Err(CompilerError::parse_error(
                        1, 1,
                        "Expected colon after property name".to_string(),
                    ));
                }
            }
        }
        
        let token = self.current_token().clone();

        match token {
            Token::Identifier(_) => {
                // This could be a method signature: methodName(params): ReturnType
                // First, check if there's a '(' after the identifier
                let name = if let Token::Identifier(name) = self.current_token() {
                    name.clone()
                } else {
                    return Err(CompilerError::parse_error(
                        1, 1,
                        "Expected identifier for interface member".to_string(),
                    ));
                };

                // Look ahead to see if this is followed by '('
                if self.position + 1 < self.tokens.len() && self.tokens[self.position + 1] == Token::LeftParen {
                    // This is a method signature
                    self.advance(); // consume method name
                    let parameters = self.parse_parameters()?;
                    let return_type = if self.current_token() == &Token::Colon {
                        self.advance();
                        Some(self.parse_type()?)
                    } else {
                        None
                    };
                    self.expect_token(&Token::Semicolon)?;

                    Ok(ObjectTypeMember::Method(MethodSignature {
                        name,
                        optional: false,
                        type_parameters: Vec::new(),
                        parameters,
                        return_type,
                    }))
                } else {
                    // This is a property signature
                    self.advance(); // consume property name
                    let optional = if self.current_token() == &Token::QuestionMark {
                        self.advance();
                        true
                    } else {
                        false
                    };

                    self.expect_token(&Token::Colon)?;
                    let type_annotation = self.parse_type()?;
                    self.expect_token(&Token::Semicolon)?;

                    Ok(ObjectTypeMember::Property(PropertySignature {
                        name,
                        optional,
                        type_: Some(type_annotation),
                        readonly,
                    }))
                }
            }
            Token::LeftParen => {
                // It's a call signature: (x: number, y: number): number;
                let parameters = self.parse_parameters()?;
                let return_type = if self.current_token() == &Token::Colon {
                    self.advance();
                    Some(self.parse_type()?)
                } else {
                    None
                };
                self.expect_token(&Token::Semicolon)?;

                Ok(ObjectTypeMember::Method(MethodSignature {
                    name: "call".to_string(), // Use a default name for call signatures
                    optional: false,
                    type_parameters: Vec::new(),
                    parameters,
                    return_type,
                }))
            }
            Token::Keyword(crate::lexer::Keyword::New) => {
                // It's a construct signature: new (name: string): BasicInterface;
                self.advance(); // consume 'new'
                let parameters = self.parse_parameters()?;
                let return_type = if self.current_token() == &Token::Colon {
                    self.advance();
                    Some(self.parse_type()?)
                } else {
                    None
                };
                self.expect_token(&Token::Semicolon)?;

                Ok(ObjectTypeMember::Method(MethodSignature {
                    name: "constructor".to_string(), // Use a default name for construct signatures
                    optional: false,
                    type_parameters: Vec::new(),
                    parameters,
                    return_type,
                }))
            }
            Token::Keyword(crate::lexer::Keyword::Readonly) => {
                // Handle case where readonly is the property name (without modifier)
                let name = "readonly".to_string();
                self.advance();
                
                if self.current_token() == &Token::Colon {
                    // It's a property signature
                    self.advance();
                    let type_annotation = self.parse_type()?;
                    self.expect_token(&Token::Semicolon)?;

                    Ok(ObjectTypeMember::Property(PropertySignature {
                        name,
                        optional: false,
                        type_: Some(type_annotation),
                        readonly,
                    }))
                } else {
                    Err(CompilerError::parse_error(
                        1, 1,
                        "Expected colon after property name".to_string(),
                    ))
                }
            }
            Token::LeftBracket => {
                // It's an index signature: [key: string]: any;
                self.advance(); // consume '['
                let key_name = match self.current_token() {
                    Token::Identifier(name) => {
                        let name = name.clone();
                        self.advance();
                        name
                    }
                    Token::Keyword(crate::lexer::Keyword::Key) => {
                        self.advance();
                        "key".to_string()
                    }
                    _ => return Err(CompilerError::parse_error(
                        1, 1,
                        "Expected identifier or 'key' in index signature".to_string(),
                    ))
                };
                self.expect_token(&Token::Colon)?;
                let key_type = self.parse_type()?;
                self.expect_token(&Token::RightBracket)?;
                self.expect_token(&Token::Colon)?;
                let value_type = self.parse_type()?;
                self.expect_token(&Token::Semicolon)?;

                Ok(ObjectTypeMember::Index(IndexSignature {
                    parameter: Box::new(Parameter {
                        name: key_name,
                        type_: Some(Box::new(key_type)),
                        optional: false,
                        initializer: None,
                        rest: false,
                    }),
                    type_: value_type,
                    readonly: false,
                }))
            }
            _ => Err(CompilerError::parse_error(
                1,
                1,
                "Expected interface member".to_string(),
            )),
        }
    }

    fn parse_enum_member(&mut self) -> Result<EnumMember> {
        let name = self.expect_identifier()?;

        let initializer = if self.current_token() == &Token::Assign {
            self.advance();
            Some(self.parse_expression()?)
        } else {
            None
        };

        Ok(EnumMember { name, initializer })
    }

    fn parse_if_statement(&mut self) -> Result<Statement> {
        self.expect_keyword()?; // if

        self.expect_token(&Token::LeftParen)?;
        let test = self.parse_expression()?;
        self.expect_token(&Token::RightParen)?;

        let consequent = self.parse_statement()?.unwrap();

        let alternate = if self.current_token() == &Token::Keyword(crate::lexer::Keyword::Else) {
            self.advance();
            Some(self.parse_statement()?.unwrap())
        } else {
            None
        };

        Ok(Statement::IfStatement(Box::new(IfStatement {
            condition: test,
            consequent: Box::new(consequent),
            alternate,
        })))
    }

    fn parse_property_key(&mut self) -> Result<Expression> {
        // TODO: Implement property key parsing
        self.parse_expression()
    }

    /// Parse index signature: [key: type]: returnType
    fn parse_index_signature(&mut self) -> Result<IndexSignature> {
        self.expect_token(&Token::LeftBracket)?;
        
        let key_name = match self.current_token() {
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();
                name
            }
            Token::Keyword(Keyword::Key) => {
                self.advance();
                "key".to_string()
            }
            _ => {
                return Err(CompilerError::parse_error(
                    self.position,
                    0,
                    format!("Expected identifier or 'key', found {:?}", self.current_token()),
                ));
            }
        };
        
        self.expect_token(&Token::Colon)?;
        let key_type = self.parse_type()?;
        self.expect_token(&Token::RightBracket)?;
        self.expect_token(&Token::Colon)?;
        let value_type = self.parse_type()?;
        
        Ok(IndexSignature {
            parameter: Box::new(Parameter {
                name: key_name,
                type_: Some(Box::new(key_type)),
                optional: false,
                initializer: None,
                rest: false,
            }),
            type_: value_type,
            readonly: false,
        })
    }

    /// Parse mapped type: [P in K] or [P in keyof T]
    fn parse_mapped_type(&mut self) -> Result<MappedType> {
        // Parse [P in K]: T
        self.expect_token(&Token::LeftBracket)?;
        
        let type_parameter_name = match self.current_token() {
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();
                name
            }
            Token::Keyword(Keyword::Key) => {
                self.advance();
                "Key".to_string()
            }
            _ => {
                return Err(CompilerError::parse_error(
                    self.position,
                    0,
                    format!("Expected identifier or Key, found {:?}", self.current_token()),
                ));
            }
        };
        let type_parameter = TypeParameter {
            name: type_parameter_name.clone(),
            constraint: None,
            default: None,
        };

        // Expect 'in' keyword
        if self.current_token() == &Token::Keyword(Keyword::In) {
            self.advance();
        } else {
            return Err(CompilerError::parse_error(
                self.position,
                0,
                format!("Expected 'in', found {:?}", self.current_token()),
            ));
        }
        
        let constraint_type = self.parse_type()?;
        
        self.expect_token(&Token::RightBracket)?;
        self.expect_token(&Token::Colon)?;
        
        let value_type = self.parse_type()?;
        
        // Skip semicolon if present (it's optional in mapped types)
        if self.current_token() == &Token::Semicolon {
            self.advance();
        }
        
        Ok(MappedType {
            type_parameter: Box::new(type_parameter),
            constraint: Some(Box::new(constraint_type)),
            name_type: None,
            type_: Box::new(value_type),
            readonly: None,
            optional: None,
        })
    }

}
