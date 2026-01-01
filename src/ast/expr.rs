use crate::prelude::*;

#[derive(Debug, Print, Walk, Respace)]
pub enum ExprKind {
    Literal(Literal),
    Block(LabeledBlock),
    AsyncBlock(AsyncBlock),
    TryBlock(TryBlock),
    Unsafe(UnsafeBlock),
    Const(ConstBlock),
    If(IfExpr),
    While(While),
    For(For),
    Loop(Loop),
    Break(Break),
    Continue(Continue),
    Return(Return),
    Yield(Yield),
    Become(Become),
    QPath(QPath),
    Struct(ExprStruct),
}

#[derive(Debug, Print, Walk, Respace)]
pub struct AsyncBlock {
    pub token: Token![async],
    #[sourcery(spaces = 1)]
    pub t1: Trivia,
    // TODO `move`
    pub block: Block,
}

#[derive(Debug, Print, Walk, Respace)]
pub struct TryBlock {
    pub token: Token![try],
    #[sourcery(spaces = 1)]
    pub t1: Trivia,
    // TODO `move`
    pub block: Block,
}

#[derive(Debug, Print, Walk, Respace)]
pub struct UnsafeBlock {
    pub token: Token![unsafe],
    #[sourcery(spaces = 1)]
    pub t1: Trivia,
    pub block: Block,
}

#[derive(Debug, Print, Walk, Respace)]
pub struct ConstBlock {
    pub token: Token![const],
    #[sourcery(spaces = 1)]
    pub t1: Trivia,
    pub block: Block,
}

#[derive(Debug, Print, Walk, Respace)]
pub struct IfExpr {
    pub token: Token![if],
    #[sourcery(spaces = 1)]
    pub t1: Trivia,
    pub cond: Box<Expr>,
    #[sourcery(spaces = 1)]
    pub t2: Trivia,
    pub then: Block,
    pub else_: Option<Else>,
}

#[derive(Debug, Print, Walk, Respace)]
pub struct Else {
    #[sourcery(spaces = 1)]
    pub t3: Trivia,
    pub token: Token![else],
    #[sourcery(spaces = 1)]
    pub t4: Trivia,
    pub kind: ElseKind,
}

#[derive(Debug, Print, Walk, Respace)]
pub enum ElseKind {
    Else(Block),
    ElseIf(Box<IfExpr>),
}

#[derive(Debug, Print, Walk, Respace)]
pub struct While {
    pub label: Option<Label>,
    pub token: Token![while],
    #[sourcery(spaces = 1)]
    pub t1: Trivia,
    pub cond: Box<Expr>,
    #[sourcery(spaces = 1)]
    pub t2: Trivia,
    pub then: Block,
}

#[derive(Debug, Print, Walk, Respace)]
pub struct For {
    pub label: Option<Label>,
    pub token: Token![for],
    #[sourcery(spaces = 1)]
    pub t1: Trivia,
    pub pat: Pat,
    #[sourcery(spaces = 1)]
    pub t2: Trivia,
    pub in_: Token![in],
    #[sourcery(spaces = 1)]
    pub t3: Trivia,
    pub expr: Box<Expr>,
    #[sourcery(spaces = 1)]
    pub t4: Trivia,
    pub block: Block,
}

#[derive(Debug, Print, Walk, Respace)]
pub struct Loop {
    pub label: Option<Label>,
    pub token: Token![loop],
    #[sourcery(spaces = 1)]
    pub t1: Trivia,
    pub block: Block,
}

#[derive(Debug, Print, Walk, Respace)]
pub struct Break {
    pub token: Token![break],
    pub label: Option<L<Ident>>,
    pub expr: Option<L<Box<Expr>>>,
}

#[derive(Debug, Print, Walk, Respace)]
pub struct Continue {
    pub token: Token![continue],
    pub label: Option<L<Ident>>,
}

#[derive(Debug, Print, Walk, Respace)]
pub struct Return {
    pub token: Token![return],
    pub expr: Option<L<Box<Expr>>>,
}

#[derive(Debug, Print, Walk, Respace)]
pub struct Yield {
    pub token: Token![yield],
    pub expr: Option<L<Box<Expr>>>,
}

#[derive(Debug, Print, Walk, Respace)]
pub struct Become {
    pub token: Token![become],
    #[sourcery(spaces = 1)]
    pub t1: Trivia,
    pub expr: Box<Expr>,
}

#[derive(Debug, Print, Walk)]
pub struct ExprStructField {
    pub attrs: List<Attribute>,
    pub ident: Ident,
    pub expr: Option<(L<Token![:]>, L<Box<Expr>>)>,
}

#[derive(Debug, Print, Walk)]
pub struct ExprStructFields {
    pub t1: Trivia,
    pub list: SeparatedList<ExprStructField, Token![,]>,
    pub dot2: Option<L<Token![..]>>,
    pub rest: Option<L<Box<Expr>>>,
    pub tlast: Trivia,
}

#[derive(Debug, Print, Walk)]
pub struct ExprStruct {
    pub qpath: QPath,
    pub t0: Trivia,
    pub fields: Braces<ExprStructFields>,
}

impl Respace for ExprStruct {
    fn respace(&mut self, _: &mut Spaces) {
        todo!()
    }
}

#[derive(Debug, Print, Walk, Respace)]
pub struct Expr {
    pub attrs: List<Attribute>,
    pub kind: ExprKind,
}
