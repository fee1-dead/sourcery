use crate::ast::{
    Attribute, AttributeInner, AttributeStyle, AttributeValue, BlockInner, Braces, Brackets, Const, Delimited, Expr, ExprKind, File, Fn, FnParam, FnRet, Ident, Item, ItemKind, List, Literal, Mod, Module, Parens, Pat, Path, PathSegment, Stmt, StmtKind, Trivia, TriviaN, Ty, TyAlias, TyArray, TySlice, VisRestricted, Visibility
};
use crate::parse::{TokenStream, TokenTree};

mod minify;

macro_rules! visit_default_noop {
    ($($visit:ident($Ty:ty);)*) => {
        $(fn $visit(&mut self, _value: &mut $Ty) {
            #[expect(non_local_definitions)]
            impl Visit for $Ty {
                fn visit<P: Pass + ?Sized>(&mut self, p: &mut P) {
                    p.$visit(self);
                }
            }
        })*
    };
}

macro_rules! visit_default_walk {
    ($($visit:ident($Ty:ty);)*) => {
        $(fn $visit(&mut self, value: &mut $Ty) {
            #[expect(non_local_definitions)]
            impl Visit for $Ty {
                fn visit<P: Pass + ?Sized>(&mut self, p: &mut P) {
                    p.$visit(self);
                }
            }
            Walk::walk(value, self);
        })*
    };
}

pub trait Pass {
    visit_default_noop! {
        visit_trivia(Trivia);
        visit_trivia_n(TriviaN);
        visit_ident(Ident);
        visit_literal(Literal);
    }

    visit_default_walk! {
        visit_file(File);
        visit_attr(Attribute);
        visit_attr_inner(AttributeInner);
        visit_attr_style(AttributeStyle);
        visit_attr_value(AttributeValue);
        visit_item(Item);
        visit_item_kind(ItemKind);
        visit_mod(Mod);
        visit_module(Module);
        visit_vis(Visibility);
        visit_vis_restricted(VisRestricted);
        visit_const(Const);
        visit_path(Path);
        visit_path_segment(PathSegment);
        visit_ty(Ty);
        visit_ty_slice(TySlice);
        visit_ty_array(TyArray);
        visit_expr(Expr);
        visit_expr_kind(ExprKind);
        visit_fn(Fn);
        visit_fn_param(FnParam);
        visit_fn_ret(FnRet);
        visit_block(BlockInner);
        visit_stmt(Stmt);
        visit_stmt_kind(StmtKind);
        visit_ty_alias(TyAlias);
        visit_pat(Pat);
        // only encountered inside macros and attributes
        visit_token_stream(TokenStream);
        visit_token_tree(TokenTree);
    }

    fn visit_list<T: Walk + Visit>(&mut self, l: &mut List<T>) {
        l.walk(self);
    }
}

pub trait Visit {
    fn visit<P: Pass + ?Sized>(&mut self, p: &mut P);
}

impl<T: Visit> Visit for Box<T> {
    fn visit<P: Pass + ?Sized>(&mut self, p: &mut P) {
        T::visit(self, p);
    }
}

impl<T: Walk + Visit> Visit for List<T> {
    fn visit<P: Pass + ?Sized>(&mut self, p: &mut P) {
        p.visit_list(self)
    }
}

impl<T: Visit> Visit for Brackets<T> {
    fn visit<P: Pass + ?Sized>(&mut self, p: &mut P) {
        self.0.visit(p)
    }
}

impl<T: Visit> Visit for Braces<T> {
    fn visit<P: Pass + ?Sized>(&mut self, p: &mut P) {
        self.0.visit(p)
    }
}

impl<T: Visit> Visit for Parens<T> {
    fn visit<P: Pass + ?Sized>(&mut self, p: &mut P) {
        self.0.visit(p)
    }
}

impl<T: Visit> Visit for Delimited<T> {
    fn visit<P: Pass + ?Sized>(&mut self, p: &mut P) {
        self.inner_mut().visit(p);
    }
}

impl<T1: Visit, T2: Visit> Visit for (T1, T2) {
    fn visit<P: Pass + ?Sized>(&mut self, p: &mut P) {
        let (a, b) = self;
        a.visit(p);
        b.visit(p);
    }
}

impl<T: Visit> Visit for Option<T> {
    fn visit<P: Pass + ?Sized>(&mut self, p: &mut P) {
        match self {
            Some(x) => x.visit(p),
            None => (),
        }
    }
}

pub trait Walk {
    fn walk<P: Pass + ?Sized>(&mut self, p: &mut P);
}


impl Walk for Path {
    fn walk<P: Pass + ?Sized>(&mut self, p: &mut P) {
        let Path {
            leading_colon,
            seg1,
            rest,
        } = self;
        if let Some((_, t)) = leading_colon {
            p.visit_trivia(t);
        }
        p.visit_path_segment(seg1);
        for (t1, _, t2, seg) in rest {
            p.visit_trivia(t1);
            p.visit_trivia(t2);
            p.visit_path_segment(seg);
        }
    }
}
