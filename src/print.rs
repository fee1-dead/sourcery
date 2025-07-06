use crate::{ast::{File, Item, ItemMod, Module}, conv_span, Ident, Trivia, Trivium};

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

impl Print for ItemMod {
    fn print(&self, orig_src: &str, dest: &mut String) {
        let ItemMod { kw, t1, name, t2, semi, content }
        = self;
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
