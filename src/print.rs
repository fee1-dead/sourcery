use crate::{
    Ident, Trivia, Trivium,
    ast::{File, Item, ItemMod, Module, Path, PathSegment, VisRestricted, Visibility},
    conv_span,
};

pub trait Print {
    fn print(&self, orig_src: &str, dest: &mut String);
}

impl<T: Print> Print for &'_ T {
    fn print(&self, orig_src: &str, dest: &mut String) {
        T::print(*self, orig_src, dest)
    }
}

impl Print for Ident {
    fn print(&self, orig_src: &str, dest: &mut String) {
        dest.push_str(&orig_src[conv_span(self.0.clone())]);
    }
}

impl Print for Trivium {
    fn print(&self, orig_src: &str, dest: &mut String) {
        dest.push_str(&orig_src[conv_span(self.span.clone())]);
    }
}

impl Print for Trivia {
    fn print(&self, orig_src: &str, dest: &mut String) {
        self.list.iter().for_each(|t| t.print(orig_src, dest));
    }
}

impl<T: Print> Print for Option<T> {
    fn print(&self, orig_src: &str, dest: &mut String) {
        if let Some(x) = self {
            x.print(orig_src, dest)
        }
    }
}

impl<T1: Print, T2: Print> Print for (T1, T2) {
    fn print(&self, orig_src: &str, dest: &mut String) {
        let (a, b) = self;
        a.print(orig_src, dest);
        b.print(orig_src, dest);
    }
}

impl Print for PathSegment {
    fn print(&self, orig_src: &str, dest: &mut String) {
        let PathSegment { ident } = self;
        ident.print(orig_src, dest);
    }
}

impl Print for Path {
    fn print(&self, orig_src: &str, dest: &mut String) {
        let Path {
            leading_colon,
            seg1,
            rest,
        } = self;
        leading_colon.print(orig_src, dest);
        seg1.print(orig_src, dest);
        rest.iter().for_each(|(t1, cc, t2, ident)| {
            t1.print(orig_src, dest);
            cc.print(orig_src, dest);
            t2.print(orig_src, dest);
            ident.print(orig_src, dest);
        });
    }
}

impl Print for VisRestricted {
    fn print(&self, orig_src: &str, dest: &mut String) {
        let VisRestricted { t2, in_, path, t3 } = self;
        t2.print(orig_src, dest);
        in_.print(orig_src, dest);
        path.print(orig_src, dest);
        t3.print(orig_src, dest);
    }
}

impl Print for Visibility {
    fn print(&self, orig_src: &str, dest: &mut String) {
        match self {
            Visibility::Public(p) => p.print(orig_src, dest),
            Visibility::Restricted { pub_, t1, parens } => {
                pub_.print(orig_src, dest);
                t1.print(orig_src, dest);
                parens.print(orig_src, dest);
            }
        }
    }
}

impl Print for ItemMod {
    fn print(&self, orig_src: &str, dest: &mut String) {
        let ItemMod {
            vis,
            kw,
            t1,
            name,
            t2,
            semi,
            content,
        } = self;
        if let Some((vis, t0)) = vis {
            vis.print(orig_src, dest);
            t0.print(orig_src, dest);
        }
        kw.print(orig_src, dest);
        t1.print(orig_src, dest);
        name.print(orig_src, dest);
        t2.print(orig_src, dest);
        semi.print(orig_src, dest);
        content.print(orig_src, dest);
    }
}

impl Print for Item {
    fn print(&self, orig_src: &str, dest: &mut String) {
        match self {
            Item::Mod(im) => im.print(orig_src, dest),
        }
    }
}

impl Print for Module {
    fn print(&self, orig_src: &str, dest: &mut String) {
        let Module { t1, items, tlast } = self;
        t1.print(orig_src, dest);
        items.print(orig_src, dest);
        tlast.print(orig_src, dest);
    }
}

impl Print for File {
    fn print(&self, orig_src: &str, dest: &mut String) {
        let File { module } = self;
        module.print(orig_src, dest);
    }
}
