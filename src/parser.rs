//! Parser for TypeScript code using nom parser combinators

use crate::ast::*;
use crate::error::{CompilerError, Result};
use crate::lexer::Token;

/// Parser for TypeScript code
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    /// Create a new parser
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    /// Parse the tokens into an AST
    pub fn parse(&mut self) -> Result<Program> {
        let mut statements = Vec::new();

        while self.position < self.tokens.len() {
            if let Some(statement) = self.parse_statement()? {
                statements.push(statement);
            } else {
                break;
            }
        }

        Ok(Program { statements })
    }

    /// Parse a statement
    fn parse_statement(&mut self) -> Result<Option<Statement>> {
        if self.position >= self.tokens.len() {
            return Ok(None);
        }

        let token = &self.tokens[self.position];
        let statement = match token {
            Token::EOF => return Ok(None),
            Token::Keyword(keyword) => match keyword {
                crate::lexer::Keyword::Let | crate::lexer::Keyword::Const | crate::lexer::Keyword::Var => {
                    self.parse_variable_declaration()?
                }
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
                _ => self.parse_expression_statement()?,
            },
            Token::LeftBrace => self.parse_block_statement()?,
            Token::Semicolon => {
                self.advance();
                return self.parse_statement();
            }
            _ => self.parse_expression_statement()?,
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
        self.expect_keyword()?; // function
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
        self.expect_keyword()?; // class
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
        self.expect_keyword()?; // interface
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
        self.expect_keyword()?; // type
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
        self.expect_keyword()?; // enum
        let name = self.expect_identifier()?;
        let members = self.parse_enum_members()?;

        Ok(Statement::EnumDeclaration(EnumDeclaration { name, members }))
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
        self.expect_keyword()?; // export
        let declaration = self.parse_statement()?;
        Ok(Statement::ExportDeclaration(Box::new(ExportDeclaration { declaration: Box::new(declaration.unwrap()) })))
    }

    /// Parse namespace declaration
    fn parse_namespace_declaration(&mut self) -> Result<Statement> {
        self.expect_keyword()?; // namespace
        let name = self.expect_identifier()?;
        let body = self.parse_block_statement()?;

        Ok(Statement::NamespaceDeclaration(NamespaceDeclaration { name, body: Box::new(body) }))
    }

    /// Parse module declaration
    fn parse_module_declaration(&mut self) -> Result<Statement> {
        self.expect_keyword()?; // module
        let name = self.parse_string_literal()?;
        let body = self.parse_block_statement()?;

        Ok(Statement::ModuleDeclaration(ModuleDeclaration { name, body: Box::new(body) }))
    }

    /// Parse declare statement
    fn parse_declare_statement(&mut self) -> Result<Statement> {
        self.expect_keyword()?; // declare
        let declaration = self.parse_statement()?;
        Ok(Statement::DeclareStatement(Box::new(DeclareStatement { declaration: Box::new(declaration.unwrap()) })))
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

    /// Parse expression statement
    fn parse_expression_statement(&mut self) -> Result<Statement> {
        let expression = self.parse_expression()?;
        self.expect_semicolon()?;
        Ok(Statement::ExpressionStatement(ExpressionStatement { expression }))
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
            Token::Identifier(name) => {
                self.advance();
                Ok(Expression::Identifier(name))
            }
            Token::LeftParen => {
                self.advance();
                let expression = self.parse_expression()?;
                self.expect_token(&Token::RightParen)?;
                Ok(Expression::Parenthesized(ParenthesizedExpression {
                    expression: Box::new(expression),
                }))
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
                method: false 
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
            Token::Identifier(name) => {
                self.advance();
                Ok(Type::Named(name.to_string()))
            }
            Token::LeftParen => {
                self.advance();
                let type_ = self.parse_type()?;
                self.expect_token(&Token::RightParen)?;
                Ok(Type::Parenthesized(Box::new(type_)))
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
            Token::Assign | Token::PlusAssign | Token::MinusAssign | Token::MultiplyAssign | Token::DivideAssign
        )
    }

    fn is_equality_operator(&self) -> bool {
        matches!(self.current_token(), Token::Equal | Token::NotEqual)
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
        matches!(self.current_token(), Token::Multiply | Token::Divide | Token::Modulo)
    }

    fn is_unary_operator(&self) -> bool {
        matches!(self.current_token(), Token::Plus | Token::Minus | Token::Not)
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
            
            while self.current_token() != &Token::GreaterThan {
                let name = self.expect_identifier()?;
                let constraint = if self.current_token() == &Token::Keyword(crate::lexer::Keyword::Extends) {
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
                }
            }
            
            self.expect_token(&Token::GreaterThan)?;
            Ok(type_parameters)
        } else {
            Ok(Vec::new())
        }
    }

    fn parse_parameters(&mut self) -> Result<Vec<Parameter>> {
        println!("Current token: {:?}", self.current_token());
        self.expect_token(&Token::LeftParen)?;
        let mut parameters = Vec::new();
        
        while self.current_token() != &Token::RightParen {
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

    fn parse_implements(&mut self) -> Result<Vec<Type>> {
        // TODO: Implement implements parsing
        Ok(Vec::new())
    }

    fn parse_extends(&mut self) -> Result<Vec<Type>> {
        // TODO: Implement extends parsing
        Ok(Vec::new())
    }

    fn parse_class_body(&mut self) -> Result<ClassBody> {
        // TODO: Implement class body parsing
        Ok(ClassBody { members: Vec::new() })
    }

    fn parse_interface_body(&mut self) -> Result<InterfaceBody> {
        // TODO: Implement interface body parsing
        Ok(InterfaceBody { members: Vec::new() })
    }

    fn parse_enum_members(&mut self) -> Result<Vec<EnumMember>> {
        // TODO: Implement enum member parsing
        Ok(Vec::new())
    }

    fn parse_import_specifiers(&mut self) -> Result<Vec<ImportSpecifier>> {
        // TODO: Implement import specifier parsing
        Ok(Vec::new())
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

    fn parse_property_key(&mut self) -> Result<Expression> {
        // TODO: Implement property key parsing
        self.parse_expression()
    }
}
