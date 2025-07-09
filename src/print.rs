use smol_str::SmolStr;

#[macro_export]
macro_rules! TrivialPrint {
    (@gen_match([$($existing:tt)*], $self:ident, $dest:ident$(,)?)) => {
        match $self { $($existing)* }
    };
    // don't have macro_metavar_expr.
    (@gen_match([$($existing:tt)*], $self:ident, $dest:ident,
        $var_name:ident($vis:vis $FieldTy:ty$(,)?)$(;)?
    $(,$($tt:tt)*)?)) => {
        crate::TrivialPrint!(@gen_match([$($existing)* $var_name( a ) => {
            crate::print::Print::print(a, $dest);
        }], $self, $dest, $($($tt)*)?))
    };
    (@gen_match([$($existing:tt)*], $self:ident, $dest:ident,
        $var_name:ident($vis:vis $FieldTy:ty, $vis2:vis $FieldTy2:ty$(,)?)$(;)?
    $(,$($tt:tt)*)?)) => {
        crate::TrivialPrint!(@gen_match([$($existing)* $var_name( a, b ) => {
            crate::print::Print::print(a, $dest);
            crate::print::Print::print(b, $dest);
        }], $self, $dest, $($($tt)*)?))
    };
    (@gen_match([$($existing:tt)*], $self:ident, $dest:ident,
        $var_name:ident {
            $($field_vis:vis $field_name:ident: $FieldTy:ty),*$(,)?
        }
    $(,$($tt:tt)*)?)) => {
        crate::TrivialPrint!(@gen_match([$($existing)* $var_name { $($field_name),* } => {
            $(crate::print::Print::print($field_name, $dest);)*
        }], $self, $dest, $($($tt)*)?))
    };
    (@gen_match([$($existing:tt)*], $self:ident, $dest:ident, $var_name:ident$(,$($tt:tt)*)?)) => {
        crate::TrivialPrint!(@gen_match([$($existing)* $var_name => {}], $self, $dest, $($($tt)*)?))
    };
    ($(#[derive_args(where($($where:tt)*))])? $vis:vis struct $name:ident $(<$($GenTy:ident),*$(,)?>)? $($tt:tt)*) => {
        impl$(<$($GenTy),*>)? crate::print::Print for $name $(<$($GenTy),*>)? $(where $($where)*)? {
            fn print(&self, dest: &mut String) {
                crate::TrivialPrint!(@gen_match([], self, dest, $name $($tt)*))
            }
        }
    };
    ($(#[derive_args(where($($where:tt)*))])? $vis:vis enum $name:ident $(<$($GenTy:ident),*$(,)?>)? {
        $($tt:tt)*
    }) => {
        impl$(<$($GenTy),*>)? crate::print::Print for $name $(<$($GenTy),*>)? $(where $($where)*)? {
            fn print(&self, dest: &mut String) {
                use $name::*;
                crate::TrivialPrint!(@gen_match([], self, dest, $($tt)*))
            }
        }
    };
}

pub trait Print {
    fn print(&self, dest: &mut String);
}

impl<T: Print> Print for &'_ T {
    fn print(&self, dest: &mut String) {
        T::print(*self, dest)
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

impl<T: Print> Print for Vec<T> {
    fn print(&self, dest: &mut String) {
        self.iter().for_each(|x| x.print(dest));
    }
}
