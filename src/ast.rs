//! Abstract Syntax Tree (AST) definitions for TypeScript

use serde::{Deserialize, Serialize};

/// Root program node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub statements: Vec<Statement>,
}

/// Statement types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    ClassDeclaration(ClassDeclaration),
    InterfaceDeclaration(InterfaceDeclaration),
    TypeAlias(TypeAlias),
    EnumDeclaration(EnumDeclaration),
    ImportDeclaration(ImportDeclaration),
    ExportDeclaration(Box<ExportDeclaration>),
    NamespaceDeclaration(NamespaceDeclaration),
    ModuleDeclaration(ModuleDeclaration),
    DeclareStatement(Box<DeclareStatement>),
    BlockStatement(BlockStatement),
    ExpressionStatement(ExpressionStatement),
    IfStatement(Box<IfStatement>),
    WhileStatement(WhileStatement),
    ForStatement(ForStatement),
    ReturnStatement(ReturnStatement),
    BreakStatement(BreakStatement),
    ContinueStatement(ContinueStatement),
    ThrowStatement(ThrowStatement),
    TryStatement(Box<TryStatement>),
    SwitchStatement(SwitchStatement),
}

/// Variable declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableDeclaration {
    pub keyword: crate::lexer::Keyword,
    pub name: String,
    pub type_annotation: Option<Type>,
    pub initializer: Option<Expression>,
}

/// Function declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDeclaration {
    pub name: String,
    pub type_parameters: Vec<TypeParameter>,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub body: Box<Statement>,
}

/// Class declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassDeclaration {
    pub name: String,
    pub type_parameters: Vec<TypeParameter>,
    pub extends: Option<Type>,
    pub implements: Vec<Type>,
    pub body: ClassBody,
}

/// Interface declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceDeclaration {
    pub name: String,
    pub type_parameters: Vec<TypeParameter>,
    pub extends: Vec<Type>,
    pub body: InterfaceBody,
}

/// Type alias
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeAlias {
    pub name: String,
    pub type_parameters: Vec<TypeParameter>,
    pub type_definition: Type,
}

/// Enum declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumDeclaration {
    pub name: String,
    pub members: Vec<EnumMember>,
}

/// Import declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportDeclaration {
    pub specifiers: Vec<ImportSpecifier>,
    pub source: String,
}

/// Export declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportDeclaration {
    pub declaration: Box<Statement>,
}

/// Namespace declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamespaceDeclaration {
    pub name: String,
    pub body: Box<Statement>,
}

/// Module declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDeclaration {
    pub name: String,
    pub body: Box<Statement>,
}

/// Declare statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeclareStatement {
    pub declaration: Box<Statement>,
}

/// Block statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

/// Expression statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressionStatement {
    pub expression: Expression,
}

/// If statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IfStatement {
    pub condition: Expression,
    pub consequent: Box<Statement>,
    pub alternate: Option<Statement>,
}

/// While statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Box<Statement>,
}

/// For statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForStatement {
    pub init: Option<Expression>,
    pub condition: Option<Expression>,
    pub update: Option<Expression>,
    pub body: Box<Statement>,
}

/// Return statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnStatement {
    pub argument: Option<Expression>,
}

/// Break statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakStatement {
    pub label: Option<String>,
}

/// Continue statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinueStatement {
    pub label: Option<String>,
}

/// Throw statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThrowStatement {
    pub argument: Expression,
}

/// Try statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TryStatement {
    pub block: Box<Statement>,
    pub handler: Option<CatchClause>,
    pub finalizer: Option<Statement>,
}

/// Switch statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwitchStatement {
    pub discriminant: Expression,
    pub cases: Vec<SwitchCase>,
}

/// Expression types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Logical(LogicalExpression),
    Conditional(ConditionalExpression),
    Assignment(AssignmentExpression),
    Call(CallExpression),
    Member(MemberExpression),
    Array(ArrayExpression),
    Object(ObjectExpression),
    Parenthesized(ParenthesizedExpression),
    Arrow(Box<ArrowFunctionExpression>),
    New(NewExpression),
    Super(SuperExpression),
    This(ThisExpression),
    Yield(Box<YieldExpression>),
    Await(AwaitExpression),
    TypeAssertion(TypeAssertion),
    AsExpression(AsExpression),
    NonNull(NonNullExpression),
    Optional(OptionalExpression),
    Template(TemplateLiteral),
    TaggedTemplate(TaggedTemplateExpression),
}

/// Literal values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    Undefined,
    RegExp(String, String), // pattern, flags
    BigInt(String),
}

/// Binary expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: crate::lexer::Token,
    pub right: Box<Expression>,
}

/// Unary expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnaryExpression {
    pub operator: crate::lexer::Token,
    pub argument: Box<Expression>,
}

/// Logical expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicalExpression {
    pub left: Box<Expression>,
    pub operator: crate::lexer::Token,
    pub right: Box<Expression>,
}

/// Conditional expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalExpression {
    pub test: Box<Expression>,
    pub consequent: Box<Expression>,
    pub alternate: Box<Expression>,
}

/// Assignment expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignmentExpression {
    pub left: Box<Expression>,
    pub operator: crate::lexer::Token,
    pub right: Box<Expression>,
}

/// Call expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallExpression {
    pub callee: Box<Expression>,
    pub arguments: Vec<Expression>,
}

/// Member expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberExpression {
    pub object: Box<Expression>,
    pub property: Box<Expression>,
    pub computed: bool,
}

/// Array expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrayExpression {
    pub elements: Vec<Option<Expression>>,
}

/// Object expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectExpression {
    pub properties: Vec<ObjectProperty>,
}

/// Object property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectProperty {
    pub key: Expression,
    pub value: Expression,
    pub shorthand: bool,
    pub computed: bool,
    pub method: bool,
}

/// Parenthesized expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParenthesizedExpression {
    pub expression: Box<Expression>,
}

/// Arrow function expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrowFunctionExpression {
    pub type_parameters: Vec<TypeParameter>,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Box<Type>>,
    pub body: Box<Statement>,
}

/// New expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewExpression {
    pub callee: Box<Expression>,
    pub arguments: Vec<Expression>,
}

/// Super expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuperExpression;

/// This expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThisExpression;

/// Yield expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YieldExpression {
    pub argument: Option<Box<Expression>>,
    pub delegate: bool,
}

/// Await expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwaitExpression {
    pub argument: Box<Expression>,
}

/// Type assertion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeAssertion {
    pub expression: Box<Expression>,
    pub type_: Type,
}

/// As expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsExpression {
    pub expression: Box<Expression>,
    pub type_: Type,
}

/// Non-null expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonNullExpression {
    pub expression: Box<Expression>,
}

/// Optional expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionalExpression {
    pub expression: Box<Expression>,
    pub optional: bool,
}

/// Template literal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateLiteral {
    pub quasis: Vec<TemplateElement>,
    pub expressions: Vec<Expression>,
}

/// Template element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateElement {
    pub value: String,
    pub tail: bool,
}

/// Tagged template expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaggedTemplateExpression {
    pub tag: Box<Expression>,
    pub quasi: TemplateLiteral,
}

/// Type definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Type {
    // Primitive types
    String,
    Number,
    Boolean,
    Any,
    Void,
    Never,
    Unknown,
    Null,
    Undefined,
    Object,
    Symbol,
    BigInt,

    // Named types
    Named(String),
    Qualified(QualifiedTypeName),

    // Generic types
    Generic(GenericType),
    GenericNamed { name: String, type_parameters: Vec<TypeParameter> },

    // Union and intersection types
    Union(Vec<Type>),
    Intersection(Vec<Type>),

    // Array and tuple types
    Array(Box<Type>),
    Tuple(Vec<Type>),

    // Function types
    Function(Box<FunctionType>),

    // Object types
    ObjectType(ObjectType),

    // Index signatures
    IndexSignature(Box<IndexSignature>),

    // Mapped types
    Mapped(Box<MappedType>),

    // Conditional types
    Conditional(ConditionalType),

    // Template literal types
    TemplateLiteral(TemplateLiteralType),

    // Parenthesized types
    Parenthesized(Box<Type>),

    // Type queries
    TypeQuery(Box<TypeQuery>),

    // Import types
    Import(Box<ImportType>),
}

/// Qualified type name
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualifiedTypeName {
    pub left: Box<Type>,
    pub right: String,
}

/// Generic type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericType {
    pub type_: Box<Type>,
    pub type_arguments: Vec<Type>,
}

/// Function type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionType {
    pub type_parameters: Vec<TypeParameter>,
    pub parameters: Vec<Parameter>,
    pub return_type: Box<Type>,
}

/// Object type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectType {
    pub members: Vec<ObjectTypeMember>,
}

/// Object type member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectTypeMember {
    Property(PropertySignature),
    Method(MethodSignature),
    Index(IndexSignature),
    Call(CallSignature),
    Construct(ConstructSignature),
}

/// Property signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertySignature {
    pub name: String,
    pub optional: bool,
    pub type_: Option<Type>,
    pub readonly: bool,
}

/// Method signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodSignature {
    pub name: String,
    pub optional: bool,
    pub type_parameters: Vec<TypeParameter>,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
}

/// Index signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexSignature {
    pub parameter: Box<Parameter>,
    pub type_: Type,
    pub readonly: bool,
}

/// Call signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallSignature {
    pub type_parameters: Vec<TypeParameter>,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
}

/// Construct signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructSignature {
    pub type_parameters: Vec<TypeParameter>,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
}

/// Mapped type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappedType {
    pub type_parameter: Box<TypeParameter>,
    pub constraint: Option<Box<Type>>,
    pub name_type: Option<Box<Type>>,
    pub type_: Box<Type>,
    pub readonly: Option<bool>,
    pub optional: Option<bool>,
}

/// Conditional type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalType {
    pub check_type: Box<Type>,
    pub extends_type: Box<Type>,
    pub true_type: Box<Type>,
    pub false_type: Box<Type>,
}

/// Template literal type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateLiteralType {
    pub head: String,
    pub spans: Vec<TemplateLiteralTypeSpan>,
}

/// Template literal type span
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateLiteralTypeSpan {
    pub type_: Type,
    pub literal: String,
}

/// Type query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeQuery {
    pub expr_name: Box<Expression>,
}

/// Import type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportType {
    pub argument: Box<Type>,
    pub qualifier: Option<String>,
    pub type_arguments: Option<Vec<Type>>,
}

/// Type parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeParameter {
    pub name: String,
    pub constraint: Option<Box<Type>>,
    pub default: Option<Box<Type>>,
}

/// Parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub optional: bool,
    pub type_: Option<Box<Type>>,
    pub initializer: Option<Expression>,
    pub rest: bool,
}

/// Class body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassBody {
    pub members: Vec<ClassMember>,
}

/// Class member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClassMember {
    Property(PropertyDeclaration),
    Method(MethodDeclaration),
    Constructor(ConstructorDeclaration),
    Getter(GetterDeclaration),
    Setter(SetterDeclaration),
    Index(IndexSignature),
}

/// Property declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyDeclaration {
    pub name: String,
    pub optional: bool,
    pub type_: Option<Type>,
    pub initializer: Option<Expression>,
    pub modifiers: Vec<Modifier>,
}

/// Method declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodDeclaration {
    pub name: String,
    pub optional: bool,
    pub type_parameters: Vec<TypeParameter>,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub body: Option<Statement>,
    pub modifiers: Vec<Modifier>,
}

/// Constructor declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructorDeclaration {
    pub parameters: Vec<Parameter>,
    pub body: Option<Statement>,
    pub modifiers: Vec<Modifier>,
}

/// Getter declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetterDeclaration {
    pub name: String,
    pub type_: Option<Type>,
    pub body: Option<Statement>,
    pub modifiers: Vec<Modifier>,
}

/// Setter declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetterDeclaration {
    pub name: String,
    pub parameter: Parameter,
    pub body: Option<Statement>,
    pub modifiers: Vec<Modifier>,
}

/// Interface body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceBody {
    pub members: Vec<ObjectTypeMember>,
}

/// Enum member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumMember {
    pub name: String,
    pub initializer: Option<Expression>,
}

/// Import specifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImportSpecifier {
    Named(NamedImportSpecifier),
    Default(DefaultImportSpecifier),
    Namespace(NamespaceImportSpecifier),
}

/// Named import specifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedImportSpecifier {
    pub name: String,
    pub imported: String,
}

/// Default import specifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultImportSpecifier {
    pub name: String,
}

/// Namespace import specifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamespaceImportSpecifier {
    pub name: String,
}

/// Catch clause
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatchClause {
    pub parameter: Option<Parameter>,
    pub body: Box<Statement>,
}

/// Switch case
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwitchCase {
    pub expression: Option<Expression>,
    pub statements: Vec<Statement>,
}

/// Modifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Modifier {
    Public,
    Private,
    Protected,
    Static,
    Readonly,
    Abstract,
    Async,
    Override,
}

/// Source location information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceLocation {
    pub start: Position,
    pub end: Position,
}

/// Position in source code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}
