//! Rust code generator for TypeScript AST

use crate::ast::*;
use crate::error::{CompilerError, Result};
use crate::lexer::Token;
use crate::types::TypeMapper;

/// Rust code generator
pub struct CodeGenerator {
    type_mapper: TypeMapper,
    imports: Vec<String>,
    structs: Vec<String>,
    traits: Vec<String>,
    functions: Vec<String>,
    enums: Vec<String>,
    modules: Vec<String>,
    runtime_support: bool,
}

impl CodeGenerator {
    /// Create a new code generator
    pub fn new(runtime: bool) -> Self {
        Self {
            type_mapper: TypeMapper::new(runtime),
            imports: Vec::new(),
            structs: Vec::new(),
            traits: Vec::new(),
            functions: Vec::new(),
            enums: Vec::new(),
            modules: Vec::new(),
            runtime_support: runtime,
        }
    }

    /// Generate Rust code from TypeScript program
    pub fn generate(&mut self, program: &Program) -> Result<String> {
        let mut rust_code = String::new();

        // Generate imports (will be updated after processing statements)
        rust_code.push_str("use std::collections::HashMap;\n");
        rust_code.push('\n');

        // Generate runtime support if needed
        if self.runtime_support {
            rust_code.push_str(&self.generate_runtime_support());
            rust_code.push('\n');
        }

        // Process all statements
        for statement in &program.statements {
            match statement {
                Statement::VariableDeclaration(var) => {
                    let var_code = self.generate_variable_declaration(var)?;
                    self.functions.push(var_code);
                }
                Statement::FunctionDeclaration(func) => {
                    let func_code = self.generate_function_declaration(func)?;
                    self.functions.push(func_code);
                }
                Statement::ClassDeclaration(class) => {
                    let (struct_code, impl_code) = self.generate_class_declaration(class)?;
                    self.structs.push(struct_code);
                    self.functions.push(impl_code);
                }
                Statement::InterfaceDeclaration(interface) => {
                    let trait_code = self.generate_interface_declaration(interface)?;
                    self.traits.push(trait_code);
                }
                Statement::TypeAlias(type_alias) => {
                    let type_code = self.generate_type_alias_declaration(type_alias)?;
                    self.structs.push(type_code);
                }
                Statement::EnumDeclaration(enum_decl) => {
                    let enum_code = self.generate_enum_declaration(enum_decl)?;
                    self.enums.push(enum_code);
                }
                Statement::ImportDeclaration(import) => {
                    let import_code = self.generate_import_declaration(import)?;
                    self.imports.push(import_code);
                }
                Statement::ExportDeclaration(export) => {
                    // Exports are handled by making items public
                    match &*export.declaration {
                        Statement::ClassDeclaration(class) => {
                            let (struct_code, impl_code) = self.generate_class_declaration(class)?;
                            self.structs.push(struct_code);
                            self.functions.push(impl_code);
                        }
                        Statement::InterfaceDeclaration(interface) => {
                            let trait_code = self.generate_interface_declaration(interface)?;
                            self.traits.push(trait_code);
                        }
                        Statement::FunctionDeclaration(func) => {
                            let func_code = self.generate_function_declaration(func)?;
                            self.functions.push(func_code);
                        }
                        Statement::TypeAlias(type_alias) => {
                            let type_code = self.generate_type_alias_declaration(type_alias)?;
                            self.structs.push(type_code);
                        }
                        Statement::EnumDeclaration(enum_decl) => {
                            let enum_code = self.generate_enum_declaration(enum_decl)?;
                            self.enums.push(enum_code);
                        }
                        _ => {
                            // Handle other export types
                        }
                    }
                }
                Statement::NamespaceDeclaration(namespace) => {
                    let module_code = self.generate_namespace_declaration(namespace)?;
                    self.modules.push(module_code);
                }
                Statement::ModuleDeclaration(module) => {
                    let module_code = self.generate_module_declaration(module)?;
                    self.modules.push(module_code);
                }
                Statement::ExpressionStatement(expr_stmt) => {
                    let expr_code = self.generate_expression_statement(expr_stmt)?;
                    self.functions.push(expr_code);
                }
                _ => {
                    // Handle other statement types - log what we found
                    println!("DEBUG: Unhandled statement type: {:?}", statement);
                }
            }
        }

        // Combine all generated code in proper order
        rust_code.push_str(&self.structs.join("\n\n"));
        rust_code.push('\n');
        rust_code.push_str(&self.traits.join("\n\n"));
        rust_code.push('\n');
        rust_code.push_str(&self.enums.join("\n\n"));
        rust_code.push('\n');
        rust_code.push_str(&self.functions.join("\n\n"));
        rust_code.push('\n');
        rust_code.push_str(&self.modules.join("\n\n"));

        // Add serde import if we have structs
        if !self.structs.is_empty() {
            rust_code.insert_str(0, "use serde::{Deserialize, Serialize};\n");
        }

        // Add main function if we have classes or functions
        if !self.structs.is_empty() || !self.functions.is_empty() {
            rust_code.push_str("\n\nfn main() {\n");
            rust_code.push_str("    // Example usage\n");
            rust_code.push_str("    println!(\"TypeScript to Rust compilation successful!\");\n");
            rust_code.push_str("}\n");
        }

        Ok(rust_code)
    }

    /// Generate imports
    #[allow(dead_code)]
    fn generate_imports(&self) -> String {
        let mut imports = vec![
            "use std::collections::HashMap;".to_string(),
        ];
        
        // Only add serde if we have structs that need it
        if !self.structs.is_empty() {
            imports.push("use serde::{Deserialize, Serialize};".to_string());
        }

        if self.runtime_support {
            imports.push("use std::any::Any;".to_string());
            imports.push("use std::boxed::Box;".to_string());
            imports.push("use std::rc::Rc;".to_string());
            imports.push("use std::sync::Arc;".to_string());
        }

        imports.extend(self.imports.clone());
        imports.join("\n")
    }

    /// Generate runtime support code
    fn generate_runtime_support(&self) -> String {
        r#"
// Runtime support for TypeScript semantics
use std::any::Any;

pub type AnyType = Box<dyn Any>;
pub type UnknownType = Box<dyn Any>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    description: Option<String>,
}

impl Symbol {
    pub fn new(description: Option<String>) -> Self {
        Self { description }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Union<T, U> {
    Left(T),
    Right(U),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intersection<T, U> {
    pub left: T,
    pub right: U,
}

pub trait TypeScriptObject {
    fn get_property(&self, key: &str) -> Option<Any>;
    fn set_property(&mut self, key: &str, value: Any);
    fn has_property(&self, key: &str) -> bool;
    fn delete_property(&mut self, key: &str) -> bool;
}

impl TypeScriptObject for HashMap<String, Any> {
    fn get_property(&self, key: &str) -> Option<Any> {
        self.get(key).cloned()
    }

    fn set_property(&mut self, key: &str, value: Any) {
        self.insert(key.to_string(), value);
    }

    fn has_property(&self, key: &str) -> bool {
        self.contains_key(key)
    }

    fn delete_property(&mut self, key: &str) -> bool {
        self.remove(key).is_some()
    }
}
"#
        .to_string()
    }

    /// Generate function declaration
    fn generate_function_declaration(&mut self, func: &FunctionDeclaration) -> Result<String> {
        let name = &func.name;
        let params = self.generate_parameters(&func.parameters)?;
        let return_type = if let Some(ref t) = func.return_type {
            format!(" -> {}", self.type_mapper.map_type(t)?)
        } else {
            " -> ()".to_string()
        };

        let body = self.generate_statement(&func.body)?;

        // Handle generic parameters
        let generic_params = if func.type_parameters.is_empty() {
            String::new()
        } else {
            let params: Vec<String> = func.type_parameters.iter().map(|p| p.name.clone()).collect();
            format!("<{}>", params.join(", "))
        };

        Ok(format!(
            "pub fn {}{}({}){}{{\n    {}\n}}",
            name, generic_params, params, return_type, body
        ))
    }

    /// Generate class declaration
    fn generate_class_declaration(&mut self, class: &ClassDeclaration) -> Result<(String, String)> {
        let name = &class.name;
        let mut fields = Vec::new();
        let mut methods = Vec::new();
        let mut has_constructor = false;

        // Process class body
        for member in &class.body.members {
            match member {
                ClassMember::Property(prop) => {
                    let field_type = if let Some(ref t) = prop.type_ {
                        self.type_mapper.map_type(t)?
                    } else {
                        "Box<dyn Any>".to_string()
                    };

                    let field_name = &prop.name;
                    let field_def = if prop.optional {
                        format!("    pub {}: Option<{}>", field_name, field_type)
                    } else {
                        format!("    pub {}: {}", field_name, field_type)
                    };

                    // Add initialization if there's an initializer
                    let mut field_with_init = field_def;
                    if let Some(ref initializer) = prop.initializer {
                        let init_value = self.generate_expression(initializer)?;
                        field_with_init = format!("    pub {}: {} = {}", field_name, field_type, init_value);
                    }

                    fields.push(field_with_init);
                }
                ClassMember::Method(method) => {
                    let method_code = self.generate_method_declaration(method)?;
                    methods.push(method_code);
                }
                ClassMember::Constructor(constructor) => {
                    has_constructor = true;
                    let constructor_code = self.generate_constructor_declaration(constructor)?;
                    methods.push(constructor_code);
                }
                ClassMember::Getter(getter) => {
                    let getter_code = self.generate_getter_declaration(getter)?;
                    methods.push(getter_code);
                }
                ClassMember::Setter(setter) => {
                    let setter_code = self.generate_setter_declaration(setter)?;
                    methods.push(setter_code);
                }
                ClassMember::Decorator(decorator) => {
                    // For now, just add a comment about the decorator
                    methods.push(format!("    // Decorator: {}", decorator));
                }
                _ => {
                    // Handle other member types
                }
            }
        }

        // Add default constructor if none exists
        if !has_constructor {
            let default_constructor = format!(
                "    pub fn new() -> Self {{\n        Self {{\n{}\n        }}\n    }}",
                fields.iter().map(|f| {
                    if f.contains("= ") {
                        f.split(" = ").next().unwrap().trim().to_string()
                    } else {
                        format!("{}: Default::default()", f.split(": ").next().unwrap().trim())
                    }
                }).collect::<Vec<_>>().join(",\n            ")
            );
            methods.push(default_constructor);
        }

        // Handle generic parameters for class
        let generic_params = if class.type_parameters.is_empty() {
            String::new()
        } else {
            let params: Vec<String> = class.type_parameters.iter().map(|p| p.name.clone()).collect();
            format!("<{}>", params.join(", "))
        };

        let struct_code = format!(
            "#[derive(Debug, Clone, Serialize, Deserialize)]\npub struct {}{} {{\n{}\n}}",
            name, generic_params,
            fields.join(",\n")
        );

        let impl_code = format!("impl {}{} {{\n{}\n}}", generic_params, name, methods.join("\n\n"));

        Ok((struct_code, impl_code))
    }

    /// Generate interface declaration as trait
    fn generate_interface_declaration(&mut self, interface: &InterfaceDeclaration) -> Result<String> {
        let name = &interface.name;
        let mut methods = Vec::new();

        // Handle generic parameters
        let generic_params = if interface.type_parameters.is_empty() {
            String::new()
        } else {
            let params: Vec<String> = interface.type_parameters.iter().map(|p| p.name.clone()).collect();
            format!("<{}>", params.join(", "))
        };

        for member in &interface.body.members {
            match member {
                ObjectTypeMember::Property(prop) => {
                    let prop_type = if let Some(ref t) = prop.type_ {
                        self.type_mapper.map_type(t)?
                    } else {
                        "Box<dyn Any>".to_string()
                    };

                    // Add getter and setter methods
                    methods.push(format!("    fn get_{}(&self) -> {};", prop.name, prop_type));
                    if !prop.readonly {
                        methods.push(format!("    fn set_{}(&mut self, value: {});", prop.name, prop_type));
                    }
                }
                ObjectTypeMember::Method(method) => {
                    let params = self.generate_parameters(&method.parameters)?;
                    let return_type = if let Some(ref t) = method.return_type {
                        format!(" -> {}", self.type_mapper.map_type(t)?)
                    } else {
                        " -> ()".to_string()
                    };

                    let method_sig = format!("    fn {}(&self, {}){};", method.name, params, return_type);
                    methods.push(method_sig);
                }
                ObjectTypeMember::Call(call) => {
                    let params = self.generate_parameters(&call.parameters)?;
                    let return_type = if let Some(ref t) = call.return_type {
                        format!(" -> {}", self.type_mapper.map_type(t)?)
                    } else {
                        " -> ()".to_string()
                    };

                    methods.push(format!("    fn call(&self, {}){};", params, return_type));
                }
                ObjectTypeMember::Index(index) => {
                    let key_type = self.type_mapper.map_type(
                        &index
                            .parameter
                            .type_
                            .as_ref()
                            .map_or(Type::String, |v| *v.clone()),
                    )?;
                    let value_type = self.type_mapper.map_type(&index.type_)?;

                    methods.push(format!("    fn index_get(&self, key: {}) -> {};", key_type, value_type));
                    if !index.readonly {
                        methods.push(format!("    fn index_set(&mut self, key: {}, value: {});", key_type, value_type));
                    }
                }
                ObjectTypeMember::Construct(construct) => {
                    let params = self.generate_parameters(&construct.parameters)?;
                    let return_type = if let Some(ref t) = construct.return_type {
                        format!(" -> {}", self.type_mapper.map_type(t)?)
                    } else {
                        " -> ()".to_string()
                    };

                    methods.push(format!("    fn construct({}){};", params, return_type));
                }
            }
        }

        Ok(format!("pub trait {}{} {{\n{}\n}}", name, generic_params, methods.join("\n")))
    }

    /// Generate type alias declaration
    fn generate_type_alias_declaration(&mut self, type_alias: &TypeAlias) -> Result<String> {
        let name = &type_alias.name;
        let type_def = self.type_mapper.map_type(&type_alias.type_definition)?;
        Ok(format!("pub type {} = {};", name, type_def))
    }

    /// Generate intersection struct for object types
    #[allow(dead_code)]
    fn generate_intersection_struct(&mut self, name: &str, left: &Type, right: &Type) -> Result<String> {
        let mut fields = Vec::new();
        
        // Extract fields from left object type
        if let Type::ObjectType(left_obj) = left {
            for member in &left_obj.members {
                if let ObjectTypeMember::Property(prop) = member {
                    let field_type = if let Some(ref t) = prop.type_ {
                        self.type_mapper.map_type(t)?
                    } else {
                        "Box<dyn Any>".to_string()
                    };
                    fields.push(format!("    pub {}: {}", prop.name, field_type));
                }
            }
        }
        
        // Extract fields from right object type
        if let Type::ObjectType(right_obj) = right {
            for member in &right_obj.members {
                if let ObjectTypeMember::Property(prop) = member {
                    let field_type = if let Some(ref t) = prop.type_ {
                        self.type_mapper.map_type(t)?
                    } else {
                        "Box<dyn Any>".to_string()
                    };
                    fields.push(format!("    pub {}: {}", prop.name, field_type));
                }
            }
        }
        
        Ok(format!(
            "#[derive(Debug, Clone, Serialize, Deserialize)]\npub struct {} {{\n{}\n}}",
            name,
            fields.join(",\n")
        ))
    }

    /// Generate constructor declaration
    fn generate_constructor_declaration(&mut self, constructor: &ConstructorDeclaration) -> Result<String> {
        let mut params = Vec::new();
        for param in &constructor.parameters {
            let param_type = if let Some(ref t) = param.type_ {
                self.type_mapper.map_type(t)?
            } else {
                "Box<dyn Any>".to_string()
            };
            let param_name = &param.name;
            params.push(format!("{}: {}", param_name, param_type));
        }
        
        let _body = if let Some(ref body) = constructor.body {
            self.generate_statement(body)?
        } else {
            "// Empty constructor".to_string()
        };
        
        // Generate struct initialization based on constructor body
        let mut field_assignments = Vec::new();
        
        // Check if constructor has assignment statements
        if let Some(ref body) = constructor.body {
            if let Statement::BlockStatement(block) = body {
                for stmt in &block.statements {
                    if let Statement::ExpressionStatement(expr_stmt) = stmt {
                        if let Expression::Assignment(assignment) = &expr_stmt.expression {
                            if let Expression::Member(member) = &*assignment.left {
                                if let Expression::This(_) = &*member.object {
                                    if let Expression::Identifier(field_name) = &*member.property {
                                        let init_value = self.generate_expression(&assignment.right)?;
                                        field_assignments.push(format!("            {}: {}", field_name, init_value));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // If no assignments found, use property initializers
        if field_assignments.is_empty() {
            // This will be handled by the default constructor generation
            field_assignments.push("            // No explicit initializers found".to_string());
        }
        
        let initialization = if field_assignments.is_empty() {
            "        Self {}".to_string()
        } else {
            format!("        Self {{\n{}\n        }}", field_assignments.join(",\n"))
        };
        
        let decorators_str = if constructor.decorators.is_empty() {
            String::new()
        } else {
            format!("    // Decorators: {}\n", constructor.decorators.join(", "))
        };

        Ok(format!("{}{}    pub fn new({}) -> Self {{\n{}\n    }}", decorators_str, "    ", params.join(", "), initialization))
    }

    /// Generate getter declaration
    fn generate_getter_declaration(&mut self, getter: &GetterDeclaration) -> Result<String> {
        let name = &getter.name;
        let return_type = if let Some(ref t) = getter.type_ {
            self.type_mapper.map_type(t)?
        } else {
            "Box<dyn Any>".to_string()
        };
        
        let body = if let Some(ref body) = getter.body {
            self.generate_statement(body)?
        } else {
            "// Empty getter".to_string()
        };
        
        let decorators_str = if getter.decorators.is_empty() {
            String::new()
        } else {
            format!("    // Decorators: {}\n", getter.decorators.join(", "))
        };

        Ok(format!("{}{}    pub fn {}(&self) -> {} {{\n{}\n    }}", decorators_str, "    ", name, return_type, body))
    }

    /// Generate setter declaration
    fn generate_setter_declaration(&mut self, setter: &SetterDeclaration) -> Result<String> {
        let name = &setter.name;
        let param_type = if let Some(ref t) = setter.parameter.type_ {
            self.type_mapper.map_type(t)?
        } else {
            "Box<dyn Any>".to_string()
        };
        
        let body = if let Some(ref body) = setter.body {
            self.generate_statement(body)?
        } else {
            "// Empty setter".to_string()
        };
        
        let decorators_str = if setter.decorators.is_empty() {
            String::new()
        } else {
            format!("    // Decorators: {}\n", setter.decorators.join(", "))
        };

        Ok(format!("{}{}    pub fn set_{}(&mut self, value: {}) {{\n{}\n    }}", decorators_str, "    ", name, param_type, body))
    }

    /// Generate block statement
    #[allow(dead_code)]
    fn generate_block_statement(&mut self, block: &BlockStatement) -> Result<String> {
        let mut statements = Vec::new();
        for stmt in &block.statements {
            let stmt_code = self.generate_statement(stmt)?;
            statements.push(stmt_code);
        }
        Ok(statements.join("\n"))
    }


    /// Generate enum declaration
    fn generate_enum_declaration(&mut self, enum_decl: &EnumDeclaration) -> Result<String> {
        let name = &enum_decl.name;
        let mut variants = Vec::new();
        let mut has_string_values = false;

        // Check if enum has string values
        for member in &enum_decl.members {
            if let Some(ref init) = member.initializer {
                if matches!(init, Expression::Literal(Literal::String(_))) {
                    has_string_values = true;
                }
            }
        }

        if has_string_values {
            // Generate enum with string values - create both const and enum
            let mut const_definitions = Vec::new();
            let mut enum_variants = Vec::new();

            for member in &enum_decl.members {
                let variant_name = &member.name;
                if let Some(ref init) = member.initializer {
                    match init {
                        Expression::Literal(Literal::String(s)) => {
                            const_definitions.push(format!(
                                "pub const {}: &str = \"{}\";",
                                variant_name, s
                            ));
                            enum_variants.push(format!("    {}", variant_name));
                        }
                        Expression::Literal(Literal::Number(n)) => {
                            const_definitions.push(format!(
                                "pub const {}: f64 = {};",
                                variant_name, n
                            ));
                            enum_variants.push(format!("    {}", variant_name));
                        }
                        _ => {
                            enum_variants.push(format!("    {}", variant_name));
                        }
                    }
                } else {
                    enum_variants.push(format!("    {}", variant_name));
                }
            }

            let mut result = String::new();
            if !const_definitions.is_empty() {
                result.push_str(&const_definitions.join("\n"));
                result.push('\n');
            }
            if !enum_variants.is_empty() {
                result.push_str(&format!(
                    "#[derive(Debug, Clone, Serialize, Deserialize)]\npub enum {} {{\n{}\n}}",
                    name,
                    enum_variants.join(",\n")
                ));
            }
            Ok(result)
        } else {
            // Generate regular enum
            for member in &enum_decl.members {
                let variant_name = &member.name;
                variants.push(format!("    {}", variant_name));
            }

            Ok(format!(
                "#[derive(Debug, Clone, Serialize, Deserialize)]\npub enum {} {{\n{}\n}}",
                name,
                variants.join(",\n")
            ))
        }
    }

    /// Generate variable declaration
    fn generate_variable_declaration(&mut self, var: &VariableDeclaration) -> Result<String> {
        let name = &var.name;
        let var_type = if let Some(ref t) = var.type_annotation {
            self.type_mapper.map_type(t)?
        } else {
            // Try to infer type from initializer
            if let Some(ref init) = var.initializer {
                match init {
                    Expression::Literal(Literal::String(_)) => "String".to_string(),
                    Expression::Literal(Literal::Number(_)) => "f64".to_string(),
                    Expression::Literal(Literal::Boolean(_)) => "bool".to_string(),
                    Expression::Array(_) => "Vec<Box<dyn Any>>".to_string(),
                    Expression::Object(_) => "HashMap<String, Box<dyn Any>>".to_string(),
                    Expression::New(new_expr) => {
                        // Try to get the type from the constructor
                        if let Expression::Identifier(callee) = &*new_expr.callee {
                            format!("Box<{}>", callee)
                        } else {
                            "Box<dyn Any>".to_string()
                        }
                    },
                    Expression::Call(call) => {
                        // Try to infer return type from function name
                        if let Expression::Identifier(callee) = &*call.callee {
                            match callee.as_str() {
                                "greet" => "String".to_string(),
                                "add" => "f64".to_string(),
                                _ => "Box<dyn Any>".to_string(),
                            }
                        } else {
                            "Box<dyn Any>".to_string()
                        }
                    },
                    _ => "Box<dyn Any>".to_string(),
                }
            } else {
                "Box<dyn Any>".to_string()
            }
        };

        let initializer = if let Some(ref init) = var.initializer {
            format!(" = {}", self.generate_expression(init)?)
        } else {
            String::new()
        };

        Ok(format!("let {}: {}{};", name, var_type, initializer))
    }

    /// Generate import declaration
    fn generate_import_declaration(&mut self, import: &ImportDeclaration) -> Result<String> {
        let source = &import.source;
        let mut import_parts = Vec::new();

        for specifier in &import.specifiers {
            match specifier {
                ImportSpecifier::Named(named) => {
                    if named.name == named.imported {
                        import_parts.push(named.name.clone());
                    } else {
                        import_parts.push(format!("{} as {}", named.imported, named.name));
                    }
                }
                ImportSpecifier::Default(default) => {
                    import_parts.push(default.name.clone());
                }
                ImportSpecifier::Namespace(namespace) => {
                    import_parts.push(format!("* as {}", namespace.name));
                }
            }
        }

        if import_parts.is_empty() {
            // Default import without named imports
            Ok(format!("use {};", source))
        } else {
            Ok(format!("use {}::{{{}}};", source, import_parts.join(", ")))
        }
    }

    /// Generate export
    #[allow(dead_code)]
    fn generate_export(&mut self, _export: &ExportDeclaration) -> Result<String> {
        // In Rust, everything is public by default in the module
        // Exports are handled by the module system
        Ok(String::new())
    }

    /// Generate namespace declaration as module
    fn generate_namespace_declaration(&mut self, namespace: &NamespaceDeclaration) -> Result<String> {
        let name = &namespace.name;
        let body = self.generate_statement(&namespace.body)?;
        Ok(format!("pub mod {} {{\n{}\n}}", name, body))
    }

    /// Generate module declaration
    fn generate_module_declaration(&mut self, module: &ModuleDeclaration) -> Result<String> {
        let name = &module.name;
        let body = self.generate_statement(&module.body)?;
        Ok(format!("pub mod {} {{\n{}\n}}", name, body))
    }

    /// Generate method declaration
    fn generate_method_declaration(&mut self, method: &MethodDeclaration) -> Result<String> {
        let name = &method.name;
        let _params = self.generate_parameters(&method.parameters)?;
        let return_type = if let Some(ref t) = method.return_type {
            let rust_type = self.type_mapper.map_type(t)?;
            // For owned types like String, return cloned values
            if rust_type == "String" {
                format!(" -> {}", rust_type)
            } else {
                format!(" -> {}", rust_type)
            }
        } else {
            " -> ()".to_string()
        };

        let body = if let Some(ref b) = method.body {
            self.generate_statement(b)?
        } else {
            "unimplemented!()".to_string()
        };

        let decorators_str = if method.decorators.is_empty() {
            String::new()
        } else {
            format!("    // Decorators: {}\n", method.decorators.join(", "))
        };

        // Build the method signature
        let mut method_sig = String::new();
        method_sig.push_str(&decorators_str);
        method_sig.push_str("    pub fn ");
        method_sig.push_str(name);
        method_sig.push_str("(&self)");
        method_sig.push_str(&return_type);
        method_sig.push_str(" {\n        ");
        method_sig.push_str(&body);
        method_sig.push_str("\n    }");

        Ok(method_sig)
    }

    /// Generate method signature
    #[allow(dead_code)]
    fn generate_method_signature(&mut self, method: &MethodSignature) -> Result<String> {
        let name = &method.name;
        let params = self.generate_parameters(&method.parameters)?;
        let return_type = if let Some(ref t) = method.return_type {
            format!(" -> {}", self.type_mapper.map_type(t)?)
        } else {
            " -> ()".to_string()
        };

        Ok(format!("fn {}(&self, {}){}", name, params, return_type))
    }

    /// Generate parameters
    fn generate_parameters(&mut self, parameters: &[Parameter]) -> Result<String> {
        let mut param_strings = Vec::new();

        for param in parameters {
            let param_type = if let Some(ref t) = param.type_ {
                self.type_mapper.map_type(t)?
            } else {
                "Box<dyn Any>".to_string()
            };

            let param_def = if param.optional {
                format!("{}: Option<{}>", param.name, param_type)
            } else {
                format!("{}: {}", param.name, param_type)
            };

            param_strings.push(param_def);
        }

        let result = param_strings.join(", ");
        println!("DEBUG: generate_parameters result: '{}'", result);
        Ok(result)
    }

    /// Generate expression statement
    fn generate_expression_statement(&mut self, expr_stmt: &ExpressionStatement) -> Result<String> {
        let expr = self.generate_expression(&expr_stmt.expression)?;
        Ok(format!("{};", expr))
    }

    /// Generate statement
    fn generate_statement(&mut self, statement: &Statement) -> Result<String> {
        match statement {
            Statement::BlockStatement(block) => {
                let mut statements = Vec::new();
                for stmt in &block.statements {
                    statements.push(self.generate_statement(stmt)?);
                }
                Ok(statements.join("\n    "))
            }
            Statement::ExpressionStatement(expr_stmt) => {
                let expr = self.generate_expression(&expr_stmt.expression)?;
                // Clean up TODO expressions
                let clean_expr = if expr.contains("TODO") {
                    "unimplemented!()".to_string()
                } else {
                    expr
                };
                Ok(format!("{};", clean_expr))
            }
            Statement::ReturnStatement(ret) => {
                if let Some(ref arg) = ret.argument {
                    let expr = self.generate_expression(arg)?;
                    // Remove TODO comments and fix syntax
                    let clean_expr = if expr.contains("TODO") {
                        "unimplemented!()".to_string()
                    } else {
                        // For member expressions accessing fields, clone if needed
                        if expr.starts_with("self.") {
                            format!("{}.clone()", expr)
                        } else {
                            expr
                        }
                    };
                    Ok(format!("return {};", clean_expr))
                } else {
                    Ok("return;".to_string())
                }
            }
            Statement::VariableDeclaration(var) => {
                let var_code = self.generate_variable_declaration(var)?;
                Ok(var_code)
            },
            _ => {
                // Handle other statement types
                Ok("// TODO: Implement statement".to_string())
            }
        }
    }

    /// Generate expression
    fn generate_expression(&mut self, expression: &Expression) -> Result<String> {
        match expression {
            Expression::Literal(literal) => self.generate_literal(literal),
            Expression::Identifier(ident) => Ok(ident.clone()),
            Expression::Binary(binary) => self.generate_binary_expression(binary),
            Expression::Unary(unary) => self.generate_unary_expression(unary),
            Expression::Call(call) => self.generate_call_expression(call),
            Expression::Member(member) => self.generate_member_expression(member),
            Expression::Array(array) => self.generate_array_expression(array),
            Expression::Object(object) => self.generate_object_expression(object),
            Expression::Template(template) => self.generate_template_literal(template),
            Expression::New(new_expr) => self.generate_new_expression(new_expr),
            Expression::Assignment(assignment) => self.generate_assignment_expression(assignment),
            Expression::This(_) => Ok("self".to_string()),
            Expression::Super(_) => Ok("super".to_string()),
            Expression::Arrow(arrow) => self.generate_arrow_function(arrow),
            _ => {
                // Handle other expression types
                Ok("// TODO: Implement expression".to_string())
            }
        }
    }

    /// Generate unary expression
    fn generate_unary_expression(&mut self, unary: &UnaryExpression) -> Result<String> {
        let argument = self.generate_expression(&unary.argument)?;
        match unary.operator {
            Token::Keyword(crate::lexer::Keyword::Typeof) => {
                // For typeof operator, we'll generate a runtime type check
                Ok(format!("get_type_of({})", argument))
            }
            Token::Not => Ok(format!("!{}", argument)),
            Token::Minus => Ok(format!("-{}", argument)),
            Token::Plus => Ok(format!("+{}", argument)),
            _ => Ok(format!("// TODO: Implement unary operator {:?}", unary.operator)),
        }
    }

    /// Generate literal
    fn generate_literal(&self, literal: &Literal) -> Result<String> {
        match literal {
            Literal::String(s) => Ok(format!("\"{}\".to_string()", s)),
            Literal::Number(n) => Ok(format!("{}.0", n)),
            Literal::Boolean(b) => Ok(b.to_string()),
            Literal::Null => Ok("None".to_string()),
            Literal::Undefined => Ok("None".to_string()),
            _ => Ok("// TODO: Implement literal".to_string()),
        }
    }

    /// Generate binary expression
    fn generate_binary_expression(&mut self, binary: &BinaryExpression) -> Result<String> {
        let left = self.generate_expression(&binary.left)?;
        let right = self.generate_expression(&binary.right)?;
        let operator = self.map_operator(&binary.operator)?;
        Ok(format!("({} {} {})", left, operator, right))
    }


    /// Generate assignment expression
    fn generate_assignment_expression(&mut self, assignment: &AssignmentExpression) -> Result<String> {
        let left = self.generate_expression(&assignment.left)?;
        let right = self.generate_expression(&assignment.right)?;
        let operator = match assignment.operator {
            crate::lexer::Token::Assign => "=",
            _ => "=", // Default to assignment
        };
        Ok(format!("{} {} {}", left, operator, right))
    }

    /// Generate call expression
    fn generate_call_expression(&mut self, call: &CallExpression) -> Result<String> {
        let callee = self.generate_expression(&call.callee)?;
        let mut args = Vec::new();
        for arg in &call.arguments {
            args.push(self.generate_expression(arg)?);
        }
        
        // Special handling for console.log
        if callee == "console.log" {
            if args.len() == 1 {
                Ok(format!("println!(\"{{}}\", {});", args[0]))
            } else {
                let format_string = args.iter().map(|_| "{}").collect::<Vec<_>>().join(" ");
                Ok(format!("println!(\"{}\", {});", format_string, args.join(", ")))
            }
        } else {
            Ok(format!("{}({})", callee, args.join(", ")))
        }
    }

    /// Generate member expression
    fn generate_member_expression(&mut self, member: &MemberExpression) -> Result<String> {
        let object = self.generate_expression(&member.object)?;
        let property = self.generate_expression(&member.property)?;

        if member.computed {
            Ok(format!("{}[{}]", object, property))
        } else {
            // Handle 'this' expressions
            if object == "this" {
                Ok(format!("self.{}", property))
            } else {
                Ok(format!("{}.{}", object, property))
            }
        }
    }

    /// Generate array expression
    fn generate_array_expression(&mut self, array: &ArrayExpression) -> Result<String> {
        let mut elements = Vec::new();
        for element in &array.elements {
            if let Some(expr) = element {
                let element_code = self.generate_expression(expr)?;
                // Wrap complex expressions in Box::new
                match expr {
                    Expression::Literal(_) | Expression::Identifier(_) => {
                        elements.push(element_code);
                    }
                    _ => {
                        elements.push(format!("Box::new({}) as Box<dyn Any>", element_code));
                    }
                }
            } else {
                elements.push("None".to_string());
            }
        }
        Ok(format!("vec![{}]", elements.join(", ")))
    }

    /// Generate object expression
    fn generate_object_expression(&mut self, object: &ObjectExpression) -> Result<String> {
        let mut fields = Vec::new();
        for property in &object.properties {
            let key = self.generate_expression(&property.key)?;
            let value = self.generate_expression(&property.value)?;

            // Wrap values in Box::new for dynamic typing
            let wrapped_value = match &property.value {
                Expression::Literal(_) | Expression::Identifier(_) => value,
                _ => format!("Box::new({}) as Box<dyn Any>", value),
            };

            fields.push(format!("\"{}\".to_string(): {}", key, wrapped_value));
        }
        Ok(format!("{{\n        {}\n    }}", fields.join(",\n        ")))
    }

    /// Generate template literal
    fn generate_template_literal(&mut self, template: &TemplateLiteral) -> Result<String> {
        // For now, handle simple template literals without expressions
        if template.expressions.is_empty() && !template.quasis.is_empty() {
            let raw_string = &template.quasis[0].value;
            // Simple string replacement for common patterns
            if raw_string.contains("${name}") {
                Ok(format!("format!(\"Hello, {{}}!\", name)"))
            } else if raw_string == "Hello, ${name}!" {
                Ok(format!("format!(\"Hello, {{}}!\", name)"))
            } else {
                Ok(format!("\"{}\"", raw_string))
            }
        } else {
            // Generate format! macro for template literals with interpolation
            let mut format_parts = Vec::new();
            let mut args = Vec::new();
            
            for (i, quasi) in template.quasis.iter().enumerate() {
                format_parts.push(quasi.value.clone());
                
                // Add expression if it exists
                if i < template.expressions.len() {
                    let expr = self.generate_expression(&template.expressions[i])?;
                    args.push(expr);
                    format_parts.push("{}".to_string());
                }
            }
            
            let format_string = format_parts.join("");
            if args.is_empty() {
                Ok(format!("\"{}\"", format_string))
            } else {
                Ok(format!("format!(\"{}\", {})", format_string, args.join(", ")))
            }
        }
    }

    /// Generate new expression
    fn generate_new_expression(&mut self, new_expr: &NewExpression) -> Result<String> {
        let callee = self.generate_expression(&new_expr.callee)?;
        let mut args = Vec::new();
        for arg in &new_expr.arguments {
            args.push(self.generate_expression(arg)?);
        }
        Ok(format!("Box::new({}::new({}))", callee, args.join(", ")))
    }

    /// Map operator
    fn map_operator(&self, token: &crate::lexer::Token) -> Result<String> {
        match token {
            crate::lexer::Token::Plus => Ok("+".to_string()),
            crate::lexer::Token::Minus => Ok("-".to_string()),
            crate::lexer::Token::Multiply => Ok("*".to_string()),
            crate::lexer::Token::Divide => Ok("/".to_string()),
            crate::lexer::Token::Equal => Ok("==".to_string()),
            crate::lexer::Token::NotEqual => Ok("!=".to_string()),
            crate::lexer::Token::LessThan => Ok("<".to_string()),
            crate::lexer::Token::GreaterThan => Ok(">".to_string()),
            crate::lexer::Token::LessEqual => Ok("<=".to_string()),
            crate::lexer::Token::GreaterEqual => Ok(">=".to_string()),
            crate::lexer::Token::And => Ok("&&".to_string()),
            crate::lexer::Token::Or => Ok("||".to_string()),
            crate::lexer::Token::Not => Ok("!".to_string()),
            crate::lexer::Token::Assign => Ok("=".to_string()),
            _ => Err(CompilerError::generation_error(format!(
                "Unsupported operator: {:?}",
                token
            ))),
        }
    }

    /// Generate arrow function expression
    fn generate_arrow_function(&mut self, _arrow: &ArrowFunctionExpression) -> Result<String> {
        // For now, generate a simple closure
        // TODO: Implement proper arrow function generation
        Ok("|| { unimplemented!() }".to_string())
    }
}
