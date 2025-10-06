use crate::ast::{
    Attribute, Const, File, Ident, Item, ItemKind, List, Mod, Module, Path, PathSegment, Trivia,
    TriviaN, VisRestricted, Visibility,
};

mod minify;

macro_rules! visit_default_walk {
    ($($visit:ident($ty:ty);)*) => {
        $(fn $visit(&mut self, value: &mut $ty) { value.walk(self); })*
    };
}

pub trait Pass {
    fn visit_trivia(&mut self, _t: &mut Trivia) {}
    fn visit_trivia_n(&mut self, _t: &mut TriviaN) {}
    fn visit_ident(&mut self, _i: &mut Ident) {}

    visit_default_walk! {
        visit_file(File);
        visit_attr(Attribute);
        visit_item(Item);
        visit_item_kind(ItemKind);
        visit_mod(Mod);
        visit_module(Module);
        visit_vis(Visibility);
        visit_vis_restricted(VisRestricted);
        visit_const(Const);
        visit_path(Path);
        visit_path_segment(PathSegment);
    }

    fn visit_list<T: Walk + Visit>(&mut self, l: &mut List<T>) {
        l.walk(self);
    }
}

pub trait Visit {
    fn visit<P: Pass + ?Sized>(&mut self, p: &mut P);
}

macro_rules! impl_visit {
    ($Ty:ident($method:ident)) => {
        impl Visit for $Ty {
            fn visit<P: Pass + ?Sized>(&mut self, p: &mut P) {
                p.$method(self);
            }
        }
    };
}

impl_visit!(Attribute(visit_attr));
impl_visit!(Item(visit_item));

impl Visit for Option<(Visibility, Trivia)> {
    fn visit<P: Pass + ?Sized>(&mut self, p: &mut P) {
        if let Some((vis, t)) = self {
            p.visit_vis(vis);
            p.visit_trivia(t);
        }
    }
}

pub trait Walk {
    fn walk<P: Pass + ?Sized>(&mut self, p: &mut P);
}

impl Walk for File {
    fn walk<P: Pass + ?Sized>(&mut self, p: &mut P) {
        p.visit_module(&mut self.module);
    }
}

impl Walk for Attribute {
    fn walk<P: Pass + ?Sized>(&mut self, p: &mut P) {
        p.visit_trivia(&mut self.t1);
    }
}

impl Walk for Item {
    fn walk<P: Pass + ?Sized>(&mut self, p: &mut P) {
        p.visit_list(&mut self.attrs);
        p.visit_item_kind(&mut self.kind);
    }
}

impl Walk for ItemKind {
    fn walk<P: Pass + ?Sized>(&mut self, p: &mut P) {
        match self {
            ItemKind::Mod(m) => p.visit_mod(m),
            ItemKind::Const(c) => p.visit_const(c),
            ItemKind::Fn(f) => todo!(),
            ItemKind::TyAlias(_) => todo!(),
        }
    }
}

impl Walk for Mod {
    fn walk<P: Pass + ?Sized>(&mut self, p: &mut P) {
        let Mod {
            vis,
            kw: _,
            t1,
            name,
            t2,
            semi: _,
            content,
        } = self;
        vis.visit(p);
        p.visit_trivia(t1);
        p.visit_ident(name);
        p.visit_trivia(t2);
        if let Some(module) = content {
            p.visit_module(&mut module.0);
        }
    }
}

impl Walk for Const {
    fn walk<P: Pass + ?Sized>(&mut self, p: &mut P) {
        let Const {
            vis,
            kw: _,
            t1,
            name,
            t2,
            colon: _,
            t3,
            ty,
            t4,
            eq: _,
            t5,
            expr,
            t6,
            semi: _,
        } = self;
        vis.visit(p);
        p.visit_trivia(t1);
        p.visit_ident(name);
        p.visit_trivia(t2);
        p.visit_trivia(t3);
        todo!()
    }
}

impl Walk for Visibility {
    fn walk<P: Pass + ?Sized>(&mut self, p: &mut P) {
        match self {
            Visibility::Public { pub_: _ } => {}
            Visibility::Restricted {
                pub_: _,
                t1,
                parens,
            } => {
                p.visit_trivia(t1);
                p.visit_vis_restricted(&mut parens.0);
            }
        }
    }
}

impl Walk for VisRestricted {
    fn walk<P: Pass + ?Sized>(&mut self, p: &mut P) {
        let VisRestricted { t2, in_, path, t3 } = self;
        p.visit_trivia(t2);
        if let Some((_, t)) = in_ {
            p.visit_trivia_n(t);
        }
        p.visit_path(path);
        p.visit_trivia(t3);
    }
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

impl Walk for PathSegment {
    fn walk<P: Pass + ?Sized>(&mut self, p: &mut P) {
        let PathSegment { ident } = self;
        p.visit_ident(ident);
    }
}

impl Walk for Module {
    fn walk<P: Pass + ?Sized>(&mut self, p: &mut P) {
        p.visit_trivia(&mut self.t1);
        p.visit_list(&mut self.attrs);
        p.visit_list(&mut self.items);
    }
}
