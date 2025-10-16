use smol_str::SmolStr;

pub trait Print {
    fn print(&self, dest: &mut String);
}

impl<T: Print> Print for &'_ T {
    fn print(&self, dest: &mut String) {
        T::print(*self, dest)
    }
}

impl<T: Print> Print for Box<T> {
    fn print(&self, dest: &mut String) {
        T::print(self, dest)
    }
}

impl Print for SmolStr {
    fn print(&self, dest: &mut String) {
        dest.push_str(self);
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

impl<T1: Print, T2: Print, T3: Print> Print for (T1, T2, T3) {
    fn print(&self, dest: &mut String) {
        let (a, b, c) = self;
        a.print(dest);
        b.print(dest);
        c.print(dest);
    }
}

impl<T1: Print, T2: Print, T3: Print, T4: Print> Print for (T1, T2, T3, T4) {
    fn print(&self, dest: &mut String) {
        let (a, b, c, d) = self;
        a.print(dest);
        b.print(dest);
        c.print(dest);
        d.print(dest);
    }
}

impl<T1: Print, T2: Print, T3: Print, T4: Print, T5: Print, T6: Print> Print for (T1, T2, T3, T4, T5, T6) {
    fn print(&self, dest: &mut String) {
        let (a, b, c, d, e, f) = self;
        a.print(dest);
        b.print(dest);
        c.print(dest);
        d.print(dest);
        e.print(dest);
        f.print(dest);
    }
}

impl<T: Print> Print for Vec<T> {
    fn print(&self, dest: &mut String) {
        self.iter().for_each(|x| x.print(dest));
    }
}
