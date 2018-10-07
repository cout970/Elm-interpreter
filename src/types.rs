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
    BinaryOp(String, Box<Pattern>, Box<Pattern>),
    Record(Vec<String>),
    Literal(Literal),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Alias(Vec<String>, Type),
    Adt(Vec<String>, Vec<(String, Vec<Type>)>),
    Port(String, Type),
    Def(Definition),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Unit,
    Tuple(Vec<Expr>),
    List(Vec<Expr>),
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
    Ref(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Unit,
    Int(i32),
    Float(f32),
    String(String),
    Char(char),
    List(Vec<Value>),
    Tuple(Vec<Value>),
    Record(Vec<(String, Value)>),
    Adt(String, Vec<Value>),
    Fun(CurriedFunc),
}

#[derive(Debug, PartialEq, Clone)]
pub struct CurriedFunc {
    pub args: Vec<Value>,
    pub fun: Fun,
    pub arg_count: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Fun {
    Builtin(u32, Type),
    Expr(Vec<Pattern>, Expr, Type),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ValueDefinition {
    pub name: String,
    pub patterns: Vec<Pattern>,
    pub expr: Expr,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Definition(
    pub Option<Type>,
    pub ValueDefinition,
);

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Module {
    pub header: Option<ModuleHeader>,
    pub imports: Vec<Import>,
    pub statements: Vec<Statement>,
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
    AdtRef(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Import {
    pub path: Vec<String>,
    pub alias: Option<String>,
    pub exposing: Vec<Export>,
}