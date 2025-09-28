//! Semantic analysis for TypeScript code

use crate::ast::*;
use crate::error::Result;
use std::collections::HashMap;

/// Semantic analyzer for TypeScript code
pub struct SemanticAnalyzer {
    /// Symbol table for variables and functions
    symbols: HashMap<String, SymbolInfo>,
    /// Current scope
    current_scope: Vec<String>,
}

/// Information about a symbol
#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub name: String,
    pub symbol_type: SymbolType,
    pub scope: Vec<String>,
    pub defined_at: usize,
}

/// Type of symbol
#[derive(Debug, Clone)]
pub enum SymbolType {
    Variable(Type),
    Function(FunctionSignature),
    Class(ClassSignature),
    Interface(InterfaceSignature),
    Type(Type),
    Enum(EnumSignature),
}

/// Function signature
#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Type,
    pub type_parameters: Vec<TypeParameter>,
}

/// Class signature
#[derive(Debug, Clone)]
pub struct ClassSignature {
    pub name: String,
    pub extends: Option<Type>,
    pub implements: Vec<Type>,
    pub type_parameters: Vec<TypeParameter>,
}

/// Interface signature
#[derive(Debug, Clone)]
pub struct InterfaceSignature {
    pub name: String,
    pub extends: Vec<Type>,
    pub type_parameters: Vec<TypeParameter>,
}

/// Enum signature
#[derive(Debug, Clone)]
pub struct EnumSignature {
    pub name: String,
    pub members: Vec<String>,
}

impl SemanticAnalyzer {
    /// Create a new semantic analyzer
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            current_scope: Vec::new(),
        }
    }

    /// Analyze a program
    pub fn analyze(&mut self, program: &Program) -> Result<()> {
        for statement in &program.statements {
            self.analyze_statement(statement)?;
        }
        Ok(())
    }

    /// Analyze a statement
    fn analyze_statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::VariableDeclaration(var) => {
                self.analyze_variable_declaration(var)?;
            }
            Statement::FunctionDeclaration(func) => {
                self.analyze_function_declaration(func)?;
            }
            Statement::ClassDeclaration(class) => {
                self.analyze_class_declaration(class)?;
            }
            Statement::InterfaceDeclaration(interface) => {
                self.analyze_interface_declaration(interface)?;
            }
            Statement::TypeAlias(type_alias) => {
                self.analyze_type_alias(type_alias)?;
            }
            Statement::EnumDeclaration(enum_decl) => {
                self.analyze_enum_declaration(enum_decl)?;
            }
            Statement::BlockStatement(block) => {
                self.enter_scope();
                for stmt in &block.statements {
                    self.analyze_statement(stmt)?;
                }
                self.exit_scope();
            }
            _ => {
                // Handle other statement types
            }
        }
        Ok(())
    }

    /// Analyze variable declaration
    fn analyze_variable_declaration(&mut self, var: &VariableDeclaration) -> Result<()> {
        let symbol_type = if let Some(ref t) = var.type_annotation {
            SymbolType::Variable(t.clone())
        } else {
            // Infer type from initializer
            let inferred_type = if let Some(ref init) = var.initializer {
                self.infer_type_from_expression(init)?
            } else {
                Type::Any
            };
            SymbolType::Variable(inferred_type)
        };

        let symbol_info = SymbolInfo {
            name: var.name.clone(),
            symbol_type,
            scope: self.current_scope.clone(),
            defined_at: 0, // TODO: Get actual position
        };

        self.symbols.insert(var.name.clone(), symbol_info);
        Ok(())
    }

    /// Analyze function declaration
    fn analyze_function_declaration(&mut self, func: &FunctionDeclaration) -> Result<()> {
        let signature = FunctionSignature {
            name: func.name.clone(),
            parameters: func.parameters.clone(),
            return_type: func.return_type.clone().unwrap_or(Type::Void),
            type_parameters: func.type_parameters.clone(),
        };

        let symbol_info = SymbolInfo {
            name: func.name.clone(),
            symbol_type: SymbolType::Function(signature),
            scope: self.current_scope.clone(),
            defined_at: 0, // TODO: Get actual position
        };

        self.symbols.insert(func.name.clone(), symbol_info);

        // Analyze function body
        self.enter_scope();
        self.analyze_statement(&func.body)?;
        self.exit_scope();

        Ok(())
    }

    /// Analyze class declaration
    fn analyze_class_declaration(&mut self, class: &ClassDeclaration) -> Result<()> {
        let signature = ClassSignature {
            name: class.name.clone(),
            extends: class.extends.clone(),
            implements: class.implements.clone(),
            type_parameters: class.type_parameters.clone(),
        };

        let symbol_info = SymbolInfo {
            name: class.name.clone(),
            symbol_type: SymbolType::Class(signature),
            scope: self.current_scope.clone(),
            defined_at: 0, // TODO: Get actual position
        };

        self.symbols.insert(class.name.clone(), symbol_info);

        // Analyze class body
        self.enter_scope();
        for member in &class.body.members {
            self.analyze_class_member(member)?;
        }
        self.exit_scope();

        Ok(())
    }

    /// Analyze interface declaration
    fn analyze_interface_declaration(&mut self, interface: &InterfaceDeclaration) -> Result<()> {
        let signature = InterfaceSignature {
            name: interface.name.clone(),
            extends: interface.extends.clone(),
            type_parameters: interface.type_parameters.clone(),
        };

        let symbol_info = SymbolInfo {
            name: interface.name.clone(),
            symbol_type: SymbolType::Interface(signature),
            scope: self.current_scope.clone(),
            defined_at: 0, // TODO: Get actual position
        };

        self.symbols.insert(interface.name.clone(), symbol_info);
        Ok(())
    }

    /// Analyze type alias
    fn analyze_type_alias(&mut self, type_alias: &TypeAlias) -> Result<()> {
        let symbol_info = SymbolInfo {
            name: type_alias.name.clone(),
            symbol_type: SymbolType::Type(type_alias.type_definition.clone()),
            scope: self.current_scope.clone(),
            defined_at: 0, // TODO: Get actual position
        };

        self.symbols.insert(type_alias.name.clone(), symbol_info);
        Ok(())
    }

    /// Analyze enum declaration
    fn analyze_enum_declaration(&mut self, enum_decl: &EnumDeclaration) -> Result<()> {
        let members: Vec<String> = enum_decl.members.iter().map(|m| m.name.clone()).collect();
        let signature = EnumSignature {
            name: enum_decl.name.clone(),
            members,
        };

        let symbol_info = SymbolInfo {
            name: enum_decl.name.clone(),
            symbol_type: SymbolType::Enum(signature),
            scope: self.current_scope.clone(),
            defined_at: 0, // TODO: Get actual position
        };

        self.symbols.insert(enum_decl.name.clone(), symbol_info);
        Ok(())
    }

    /// Analyze class member
    fn analyze_class_member(&mut self, member: &ClassMember) -> Result<()> {
        match member {
            ClassMember::Property(_prop) => {
                // Analyze property
            }
            ClassMember::Method(_method) => {
                // Analyze method
            }
            _ => {
                // Handle other member types
            }
        }
        Ok(())
    }

    /// Infer type from expression
    fn infer_type_from_expression(&self, expression: &Expression) -> Result<Type> {
        match expression {
            Expression::Literal(literal) => self.infer_type_from_literal(literal),
            Expression::Identifier(ident) => {
                if let Some(symbol) = self.symbols.get(ident) {
                    match &symbol.symbol_type {
                        SymbolType::Variable(t) => Ok(t.clone()),
                        _ => Ok(Type::Any),
                    }
                } else {
                    Ok(Type::Any)
                }
            }
            _ => Ok(Type::Any),
        }
    }

    /// Infer type from literal
    fn infer_type_from_literal(&self, literal: &Literal) -> Result<Type> {
        match literal {
            Literal::String(_) => Ok(Type::String),
            Literal::Number(_) => Ok(Type::Number),
            Literal::Boolean(_) => Ok(Type::Boolean),
            Literal::Null => Ok(Type::Null),
            Literal::Undefined => Ok(Type::Undefined),
            _ => Ok(Type::Any),
        }
    }

    /// Enter a new scope
    fn enter_scope(&mut self) {
        self.current_scope.push("block".to_string());
    }

    /// Exit current scope
    fn exit_scope(&mut self) {
        self.current_scope.pop();
    }

    /// Get symbol information
    pub fn get_symbol(&self, name: &str) -> Option<&SymbolInfo> {
        self.symbols.get(name)
    }

    /// Get all symbols
    pub fn get_all_symbols(&self) -> &HashMap<String, SymbolInfo> {
        &self.symbols
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
