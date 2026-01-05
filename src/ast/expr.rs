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
    Match(Match),
    Break(Break),
    Continue(Continue),
    Return(Return),
    Yield(Yield),
    Become(Become),
    Let(ExprLet),
    QPath(QPath),
    Struct(ExprStruct),
    Tuple(Parens<CommaSepExprs>),
    Paren(Parens<ExprParen>),
    Array(Brackets<CommaSepExprs>),
    Repeat(Brackets<ExprRepeat>),
    Macro(MacroCall),
    Closure(Closure),
    Range(ExprRange),
    Call(ExprCall),
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

#[derive(Debug, Print, Walk)]
pub struct Arm {
    pub attrs: List<Attribute>,
    pub pat: Pat,
    pub guard: Option<(Trivia, Token![if], Trivia, Box<Expr>)>,
    pub t1: Trivia,
    pub arrow: Token![=>],
    pub t2: Trivia,
    pub body: Box<Expr>,
    pub comma: Option<(Trivia, Token![,])>,
}

#[derive(Debug, Print, Walk)]
pub struct Match {
    pub token: Token![match],
    pub t1: Trivia,
    pub expr: Box<Expr>,
    pub t2: Trivia,
    pub arms: Braces<(Trivia, List<Arm>)>,
}

impl Respace for Match {
    fn respace(&mut self, _: &mut Spaces) {
        todo!()
    }
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

#[derive(Debug, Print, Walk, Respace)]
pub struct ExprLet {
    pub token: Token![let],
    #[sourcery(spaces = 1)]
    pub t1: Trivia,
    pub pat: Box<Pat>,
    #[sourcery(spaces = 1)]
    pub t2: Trivia,
    pub eq: Token![=],
    #[sourcery(spaces = 1)]
    pub t3: Trivia,
    pub expr: Box<Expr>
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

#[derive(Debug, Print, Walk)]
pub struct CommaSepExprs {
    pub t1: Trivia,
    pub contents: SeparatedList<Expr, Token![,]>,
}

#[derive(Debug, Print, Walk)]
pub struct ExprParen {
    pub t1: Trivia,
    pub expr: Box<Expr>,
    pub t2: Trivia,
}

#[derive(Debug, Print, Walk)]
pub struct ExprRepeat {
    pub t1: Trivia,
    pub elem: Box<Expr>,
    pub t2: Trivia,
    pub semi: Token![;],
    pub t3: Trivia,
    pub len: Box<Expr>,
    pub t4: Trivia,
}

impl Respace for Parens<CommaSepExprs> {
    fn respace(&mut self, _: &mut Spaces) {
        todo!()
    }
}
 
impl Respace for Brackets<CommaSepExprs> {
    fn respace(&mut self, _: &mut Spaces) {
        todo!()
    }
}

impl Respace for Parens<ExprParen> {
    fn respace(&mut self, _: &mut Spaces) {
        todo!()
    }
}

impl Respace for Brackets<ExprRepeat> {
    fn respace(&mut self, _: &mut Spaces) {
        todo!()
    }
}

#[derive(Debug, Print, Walk)]
pub struct ClosureArg {
    pub attrs: List<Attribute>,
    pub pat: Pat,
    pub ty: Option<(Trivia, Token![:], Trivia, Ty)>,
    pub comma: Option<(Trivia, Token![,])>,
}

#[derive(Debug, Print, Walk)]
pub struct Closure {
    pub bar1: Token![|],
    pub t1: Trivia,
    pub args: List<ClosureArg>,
    pub bar2: Token![|],
    pub ret: Option<(Trivia, FnRet)>,
    pub t2: Trivia,
    pub body: Box<Expr>,
}

impl Respace for Closure {
    fn respace(&mut self, _: &mut Spaces) {
        todo!()
    }
}

#[derive(Debug, Print, Walk)]
pub enum RangeLimits {
    HalfOpen(Token![..=]),
    Closed(Token![..]),
}

#[derive(Debug, Print, Walk)]
pub struct ExprRange {
    pub start: Option<(Box<Expr>, Trivia)>,
    pub limits: RangeLimits,
    pub end: Option<L<Box<Expr>>>,
}

impl Respace for ExprRange {
    fn respace(&mut self, _: &mut Spaces) {
        todo!()
    }
}

#[derive(Debug, Print, Walk, Respace)]
pub struct ExprCall {
    pub callee: Box<ExprKind>,
    #[sourcery(spaces = 1)]
    pub t0: Trivia,
    pub args: Parens<CommaSepExprs>,
}

#[derive(Debug, Print, Walk, Respace)]
pub struct Expr {
    pub attrs: List<Attribute>,
    pub kind: ExprKind,
}
