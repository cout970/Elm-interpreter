pub type Int = i32;
pub type Float = f32;

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Int(Int),
    Float(Float),
    String(String),
    Char(char),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Ref {
    Name(String),
    Operand(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Var(String),
    Tag(String, Vec<Type>),
    Fun(Box<Type>, Box<Type>),
    Unit,
    Tuple(Vec<Type>),
    Record(Vec<(String, Type)>),
    RecExt(String, Vec<(String, Type)>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Pattern {
    Var(String),
    Adt(String, Vec<Pattern>),
    Wildcard,
    Unit,
    Tuple(Vec<Pattern>),
    List(Vec<Pattern>),
    Binop(String, Box<Pattern>, Box<Pattern>),
    Record(Vec<Pattern>),
    Literal(Literal),
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeDefinition(pub String, pub Type);

#[derive(Debug, PartialEq, Clone)]
pub enum ValueDefinition {
    PrefixOp(String, Vec<Pattern>, Expr),
    InfixOp(Pattern, String, Vec<Pattern>, Expr),
    Name(String, Vec<Pattern>, Expr),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Definition(
    pub Option<TypeDefinition>,
    pub ValueDefinition,
);


#[derive(Debug, PartialEq, Clone, Default)]
pub struct Module {
    pub header: Option<ModuleHeader>,
    pub imports: Vec<Import>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ModuleHeader {
    pub name: Vec<String>,
    pub exports: Vec<Export>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Export {
    Adt(String, Vec<String>),
    AdtNone(String),
    AdtAll(String),
    AdtRef(Ref),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Import {
    pub path: Vec<String>,
    pub alias: Option<String>,
    pub exposing: Vec<Export>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Alias(Vec<String>, Type),
    Adt(Vec<String>, Vec<(String, Vec<Type>)>),
    Port(TypeDefinition, ValueDefinition),
    Def(Definition),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Unit,
    Tuple(Vec<Expr>),
    List(Vec<Expr>),
    Range(Box<Expr>, Box<Expr>),
    Record(Vec<(String, Expr)>),
    Adt(String),
    RecordUpdate(String, Vec<(String, Expr)>),
    QualifiedRef(Vec<String>, String),
    RecordField(Box<Expr>, String),
    RecordAccess(String),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Case(Box<Expr>, Vec<(Pattern, Expr)>),
    Lambda(Vec<Pattern>, Box<Expr>),
    Application(Box<Expr>, Box<Expr>),
    Let(Vec<Definition>, Box<Expr>),
    OpChain(Vec<Expr>, Vec<String>),
    Literal(Literal),
    Ref(Ref),
}