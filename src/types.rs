//! Type mapping from TypeScript to Rust

use crate::ast::*;
use crate::error::Result;
use std::collections::HashMap;

/// Type mapper for converting TypeScript types to Rust types
pub struct TypeMapper {
    /// Mapping of TypeScript types to Rust types
    type_mappings: HashMap<String, String>,
    /// Generic type parameters
    generics: Vec<String>,
    /// Runtime support enabled
    runtime: bool,
}

/// Extract struct name from generated code
fn extract_struct_name(code: &str) -> String {
    if let Some(start) = code.find("struct ") {
        if let Some(end) = code[start + 7..].find(" {") {
            return code[start + 7..start + 7 + end].to_string();
        }
    }
    "Unknown".to_string()
}

impl TypeMapper {
    /// Create a new type mapper
    pub fn new(runtime: bool) -> Self {
        let mut type_mappings = HashMap::new();

        // Primitive type mappings
        type_mappings.insert("string".to_string(), "String".to_string());
        type_mappings.insert("number".to_string(), "f64".to_string());
        type_mappings.insert("boolean".to_string(), "bool".to_string());
        type_mappings.insert("void".to_string(), "()".to_string());
        type_mappings.insert("never".to_string(), "!".to_string());
        type_mappings.insert("any".to_string(), "Box<dyn Any>".to_string());
        type_mappings.insert("unknown".to_string(), "Box<dyn Any>".to_string());
        type_mappings.insert("null".to_string(), "Option<()>".to_string());
        type_mappings.insert("undefined".to_string(), "Option<()>".to_string());
        type_mappings.insert("object".to_string(), "Box<dyn Any>".to_string());
        type_mappings.insert("symbol".to_string(), "Symbol".to_string());
        type_mappings.insert("bigint".to_string(), "i64".to_string());

        Self {
            type_mappings,
            generics: Vec::new(),
            runtime,
        }
    }

    /// Map a TypeScript type to Rust type
    pub fn map_type(&mut self, ts_type: &Type) -> Result<String> {
        match ts_type {
            // Primitive types
            Type::String => Ok("String".to_string()),
            Type::Number => Ok("f64".to_string()),
            Type::Boolean => Ok("bool".to_string()),
            Type::Any => {
                if self.runtime {
                    Ok("Box<dyn Any>".to_string())
                } else {
                    Ok("serde_json::Value".to_string())
                }
            }
            Type::Void => Ok("()".to_string()),
            Type::Never => Ok("!".to_string()),
            Type::Unknown => {
                if self.runtime {
                    Ok("Box<dyn Any>".to_string())
                } else {
                    Ok("serde_json::Value".to_string())
                }
            }
            Type::Null => Ok("Option<()>".to_string()),
            Type::Undefined => Ok("Option<()>".to_string()),
            Type::Object => {
                if self.runtime {
                    Ok("Box<dyn Any>".to_string())
                } else {
                    Ok("serde_json::Value".to_string())
                }
            }
            Type::Symbol => Ok("Symbol".to_string()),
            Type::BigInt => Ok("i64".to_string()),

            // Named types
            Type::Named(name) => self.map_named_type(name),
            Type::Qualified(qualified) => self.map_qualified_type(qualified),

            // Generic types
            Type::Generic(generic) => self.map_generic_type(generic),
            Type::GenericNamed {
                name,
                type_parameters,
            } => {
                let rust_name = self.map_named_type(name)?;
                if type_parameters.is_empty() {
                    Ok(rust_name)
                } else {
                    let param_types: Result<Vec<String>> = type_parameters
                        .iter()
                        .map(|param| self.map_type(&Type::Named(param.name.clone())))
                        .collect();
                    let param_types = param_types?;
                    Ok(format!("{}<{}>", rust_name, param_types.join(", ")))
                }
            }
            
            // Union and intersection types
            Type::Union { left, right } => {
                let left_type = self.map_type(left)?;
                let right_type = self.map_type(right)?;
                Ok(format!("Union<{}, {}>", left_type, right_type))
            }
            Type::Intersection { left, right } => {
                // For intersection types, we need to handle them differently
                // since they can't be directly represented in Rust
                let left_type = self.map_type(left)?;
                let right_type = self.map_type(right)?;
                
                // Check if both types are object types
                if left_type.starts_with("#[derive") && right_type.starts_with("#[derive") {
                    // Extract struct names and create a combined type
                    let left_struct = extract_struct_name(&left_type);
                    let right_struct = extract_struct_name(&right_type);
                    Ok(format!("{}And{}", left_struct, right_struct))
                } else {
                    Ok(format!("Intersection<{}, {}>", left_type, right_type))
                }
            }


            // Array types
            Type::Array(element_type) => {
                let element_rust = self.map_type(element_type)?;
                Ok(format!("Vec<{}>", element_rust))
            }

            // Tuple types
            Type::Tuple(types) => self.map_tuple_type(types),

            // Function types
            Type::Function(func_type) => self.map_function_type(func_type),

            // Object types
            Type::ObjectType(obj_type) => self.map_object_type(obj_type),

            // Index signatures
            Type::IndexSignature(index_sig) => self.map_index_signature(index_sig),

            // Mapped types
            Type::Mapped(mapped) => self.map_mapped_type(mapped),

            // Conditional types
            Type::Conditional(conditional) => self.map_conditional_type(conditional),

            // Template literal types
            Type::TemplateLiteral(template) => self.map_template_literal_type(template),

            // Parenthesized types
            Type::Parenthesized(inner) => self.map_type(inner),

            // Type queries
            Type::TypeQuery(query) => self.map_type_query(query),

            // Import types
            Type::Import(import) => self.map_import_type(import),
        }
    }

    /// Map named type
    fn map_named_type(&self, name: &str) -> Result<String> {
        if let Some(mapped) = self.type_mappings.get(name) {
            Ok(mapped.clone())
        } else {
            // Convert to PascalCase for Rust structs/traits
            Ok(self.to_pascal_case(name))
        }
    }

    /// Map qualified type
    fn map_qualified_type(&mut self, qualified: &QualifiedTypeName) -> Result<String> {
        let left = self.map_type(&qualified.left)?;
        Ok(format!("{}::{}", left, qualified.right))
    }

    /// Map generic type
    fn map_generic_type(&mut self, generic: &GenericType) -> Result<String> {
        let base_type = self.map_type(&generic.type_)?;
        let type_args: Result<Vec<String>> = generic
            .type_arguments
            .iter()
            .map(|t| self.map_type(t))
            .collect();
        let type_args = type_args?;

        if type_args.is_empty() {
            Ok(base_type)
        } else {
            Ok(format!("{}<{}>", base_type, type_args.join(", ")))
        }
    }

    /// Map union type
    fn map_union_type(&mut self, types: &[Type]) -> Result<String> {
        if types.is_empty() {
            return Ok("()".to_string());
        }

        if types.len() == 1 {
            return self.map_type(&types[0]);
        }

        // Convert union to enum
        let mut enum_variants = Vec::new();
        for (i, ts_type) in types.iter().enumerate() {
            let rust_type = self.map_type(ts_type)?;
            enum_variants.push(format!("Variant{}({})", i, rust_type));
        }

        Ok(format!(
            "UnionType {{\n    {}\n}}",
            enum_variants.join(",\n    ")
        ))
    }

    /// Map intersection type
    fn map_intersection_type(&mut self, types: &[Type]) -> Result<String> {
        if types.is_empty() {
            return Ok("()".to_string());
        }

        if types.len() == 1 {
            return self.map_type(&types[0]);
        }

        // Convert intersection to trait bounds
        let rust_types: Result<Vec<String>> = types.iter().map(|t| self.map_type(t)).collect();
        let rust_types = rust_types?;

        Ok(format!("({})", rust_types.join(" + ")))
    }

    /// Map tuple type
    fn map_tuple_type(&mut self, types: &[Type]) -> Result<String> {
        if types.is_empty() {
            return Ok("()".to_string());
        }

        let rust_types: Result<Vec<String>> = types.iter().map(|t| self.map_type(t)).collect();
        let rust_types = rust_types?;

        Ok(format!("({})", rust_types.join(", ")))
    }

    /// Map function type
    fn map_function_type(&mut self, func_type: &FunctionType) -> Result<String> {
        let params: Result<Vec<String>> = func_type
            .parameters
            .iter()
            .map(|param| {
                let param_type = if let Some(ref t) = param.type_ {
                    self.map_type(t)?
                } else {
                    "Box<dyn Any>".to_string()
                };
                Ok(format!("{}: {}", param.name, param_type))
            })
            .collect();
        let params = params?;

        let return_type = self.map_type(&func_type.return_type)?;

        Ok(format!("fn({}) -> {}", params.join(", "), return_type))
    }

    /// Map object type
    fn map_object_type(&mut self, obj_type: &ObjectType) -> Result<String> {
        let mut struct_fields = Vec::new();

        for member in &obj_type.members {
            match member {
                ObjectTypeMember::Property(prop) => {
                    let field_type = if let Some(ref t) = prop.type_ {
                        self.map_type(t)?
                    } else {
                        "Box<dyn Any>".to_string()
                    };

                    let field_name = if prop.optional {
                        format!("{}: Option<{}>", prop.name, field_type)
                    } else {
                        format!("{}: {}", prop.name, field_type)
                    };

                    struct_fields.push(field_name);
                }
                ObjectTypeMember::Method(method) => {
                    // Methods become associated functions
                    let params: Result<Vec<String>> = method
                        .parameters
                        .iter()
                        .map(|param| {
                            let param_type = if let Some(ref t) = param.type_ {
                                self.map_type(t)?
                            } else {
                                "Box<dyn Any>".to_string()
                            };
                            Ok(format!("{}: {}", param.name, param_type))
                        })
                        .collect();
                    let params = params?;

                    let return_type = if let Some(ref t) = method.return_type {
                        self.map_type(t)?
                    } else {
                        "()".to_string()
                    };

                    struct_fields.push(format!(
                        "fn {}({}) -> {}",
                        method.name,
                        params.join(", "),
                        return_type
                    ));
                }
                _ => {
                    // Handle other member types as needed
                }
            }
        }

        // Generate a proper struct name for object types
        let struct_name = format!("ObjectType_{}", struct_fields.len());
        Ok(format!(
            "#[derive(Debug, Clone, Serialize, Deserialize)]\npub struct {} {{\n    {}\n}}",
            struct_name,
            struct_fields.join(",\n    ")
        ))
    }

    /// Map index signature
    fn map_index_signature(&mut self, index_sig: &IndexSignature) -> Result<String> {
        let key_type = self.map_type(
            &index_sig
                .parameter
                .type_
                .as_ref()
                .map_or(Type::String, |v| *v.clone()),
        )?;
        let value_type = self.map_type(&index_sig.type_)?;
        Ok(format!("HashMap<{}, {}>", key_type, value_type))
    }

    /// Map mapped type
    fn map_mapped_type(&mut self, mapped: &MappedType) -> Result<String> {
        // Convert mapped type to generic struct
        let key_type = self.map_type(
            &mapped
                .type_parameter
                .constraint
                .as_ref()
                .map_or(Type::String, |v| *v.clone()),
        )?;
        let value_type = self.map_type(&mapped.type_)?;
        Ok(format!("HashMap<{}, {}>", key_type, value_type))
    }

    /// Map conditional type
    fn map_conditional_type(&mut self, conditional: &ConditionalType) -> Result<String> {
        // Convert conditional type to trait with associated type
        let check_type = self.map_type(&conditional.check_type)?;
        let extends_type = self.map_type(&conditional.extends_type)?;
        let _true_type = self.map_type(&conditional.true_type)?;
        let _false_type = self.map_type(&conditional.false_type)?;

        Ok(format!(
            "trait ConditionalType {{\n    type Output: PartialEq<{}>;\n    fn condition<{}>() -> Self::Output;\n}}",
            extends_type, check_type
        ))
    }

    /// Map template literal type
    fn map_template_literal_type(&mut self, template: &TemplateLiteralType) -> Result<String> {
        // Convert template literal type to const generic or macro
        Ok(format!("\"{}\"", template.head))
    }

    /// Map type query
    fn map_type_query(&mut self, _query: &TypeQuery) -> Result<String> {
        // Convert type query to associated type
        Ok("TypeQuery".to_string())
    }

    /// Map import type
    fn map_import_type(&mut self, import: &ImportType) -> Result<String> {
        let base_type = self.map_type(&import.argument)?;
        if let Some(ref qualifier) = import.qualifier {
            Ok(format!("{}::{}", base_type, qualifier))
        } else {
            Ok(base_type)
        }
    }

    /// Convert string to PascalCase
    fn to_pascal_case(&self, s: &str) -> String {
        let mut result = String::new();
        let mut capitalize = true;

        for ch in s.chars() {
            if ch == '_' || ch == '-' {
                capitalize = true;
            } else if capitalize {
                result.push(ch.to_uppercase().next().unwrap_or(ch));
                capitalize = false;
            } else {
                result.push(ch);
            }
        }

        result
    }

    /// Add generic type parameter
    pub fn add_generic(&mut self, name: String) {
        self.generics.push(name);
    }

    /// Get all generic type parameters
    pub fn get_generics(&self) -> &[String] {
        &self.generics
    }

    /// Clear generic type parameters
    pub fn clear_generics(&mut self) {
        self.generics.clear();
    }
}

/// Type mapping utilities
pub struct TypeMappingUtils;

impl TypeMappingUtils {
    /// Check if a TypeScript type is primitive
    pub fn is_primitive(ts_type: &Type) -> bool {
        matches!(
            ts_type,
            Type::String
                | Type::Number
                | Type::Boolean
                | Type::Void
                | Type::Never
                | Type::Any
                | Type::Unknown
                | Type::Null
                | Type::Undefined
                | Type::Object
                | Type::Symbol
                | Type::BigInt
        )
    }

    /// Check if a TypeScript type is nullable
    pub fn is_nullable(ts_type: &Type) -> bool {
        matches!(ts_type, Type::Null | Type::Undefined)
    }

    /// Check if a TypeScript type is optional
    pub fn is_optional(_ts_type: &Type) -> bool {
        // This would need to be determined from context
        false
    }

    /// Get the underlying type for optional types
    pub fn get_underlying_type(ts_type: &Type) -> &Type {
        // This would need to be implemented based on the specific type
        ts_type
    }

    /// Check if a TypeScript type needs runtime support
    pub fn needs_runtime(ts_type: &Type) -> bool {
        matches!(
            ts_type,
            Type::Any | Type::Unknown | Type::Object | Type::Union { .. } | Type::Intersection { .. }
        )
    }

    /// Generate Rust imports for a type
    pub fn generate_imports(ts_type: &Type) -> Vec<String> {
        let mut imports = Vec::new();

        match ts_type {
            Type::Any | Type::Unknown | Type::Object => {
                imports.push("use std::any::Any;".to_string());
                imports.push("use std::boxed::Box;".to_string());
            }
            Type::Union { .. } => {
                imports.push("use serde::{Deserialize, Serialize};".to_string());
            }
            Type::Intersection { .. } => {
                imports.push("use std::ops::Add;".to_string());
            }
            Type::Array(_) => {
                imports.push("use std::vec::Vec;".to_string());
            }
            Type::Tuple(_) => {
                // Tuples don't need special imports
            }
            Type::Function(_) => {
                imports.push("use std::boxed::Box;".to_string());
            }
            Type::Null => {
                // No specific import needed for Null (unit type)
            }
            _ => {}
        }

        imports
    }
}
