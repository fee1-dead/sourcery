use crate::ast::{
    File, Ident, Item, ItemMod, Module, Path, PathSegment, Trivia, Trivium, VisRestricted,
    Visibility,
};

pub trait Print {
    fn print(&self, dest: &mut String);
}

impl<T: Print> Print for &'_ T {
    fn print(&self, dest: &mut String) {
        T::print(*self, dest)
    }
}

impl Print for Ident {
    fn print(&self, dest: &mut String) {
        dest.push_str(&self.0);
    }
}

impl Print for Trivium {
    fn print(&self, dest: &mut String) {
        dest.push_str(self.snippet());
    }
}

impl Print for Trivia {
    fn print(&self, dest: &mut String) {
        self.list.iter().for_each(|t| t.print(dest));
    }
}

impl<T: Print> Print for Option<T> {
    fn print(&self, dest: &mut String) {
        if let Some(x) = self {
            x.print(dest)
        }
    }
}

impl<T1: Print, T2: Print> Print for (T1, T2) {
    fn print(&self, dest: &mut String) {
        let (a, b) = self;
        a.print(dest);
        b.print(dest);
    }
}

impl Print for PathSegment {
    fn print(&self, dest: &mut String) {
        let PathSegment { ident } = self;
        ident.print(dest);
    }
}

impl Print for Path {
    fn print(&self, dest: &mut String) {
        let Path {
            leading_colon,
            seg1,
            rest,
        } = self;
        leading_colon.print(dest);
        seg1.print(dest);
        rest.iter().for_each(|(t1, cc, t2, ident)| {
            t1.print(dest);
            cc.print(dest);
            t2.print(dest);
            ident.print(dest);
        });
    }
}

impl Print for VisRestricted {
    fn print(&self, dest: &mut String) {
        let VisRestricted { t2, in_, path, t3 } = self;
        t2.print(dest);
        in_.print(dest);
        path.print(dest);
        t3.print(dest);
    }
}

impl Print for Visibility {
    fn print(&self, dest: &mut String) {
        match self {
            Visibility::Public(p) => p.print(dest),
            Visibility::Restricted { pub_, t1, parens } => {
                pub_.print(dest);
                t1.print(dest);
                parens.print(dest);
            }
        }
    }
}

impl Print for ItemMod {
    fn print(&self, dest: &mut String) {
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
            vis.print(dest);
            t0.print(dest);
        }
        kw.print(dest);
        t1.print(dest);
        name.print(dest);
        t2.print(dest);
        semi.print(dest);
        content.print(dest);
    }
}

impl Print for Item {
    fn print(&self, dest: &mut String) {
        match self {
            Item::Mod(im) => im.print(dest),
        }
    }
}

impl Print for Module {
    fn print(&self, dest: &mut String) {
        let Module { t1, items, tlast } = self;
        t1.print(dest);
        items.print(dest);
        tlast.print(dest);
    }
}

impl Print for File {
    fn print(&self, dest: &mut String) {
        let File { module } = self;
        module.print(dest);
    }
}
