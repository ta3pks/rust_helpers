use std::fmt::Display;

pub trait DisplayExt: Display {
    /// println!("{msg}: {self}");
    fn println(&self, msg: &'static str) {
        println!("{msg}: {self}");
    }
    /// eprintln!("{msg}: {self}");
    fn eprintln(&self, msg: &'static str) {
        eprintln!("{msg}: {self}");
    }
    /// println!("{msg}: {self}"); only in debug builds
    fn println_dbg(&self, msg: &'static str) {
        #[cfg(debug_assertions)]
        println!("{msg}: {self}");
    }
    /// eprintln!("{msg}: {self}"); only in debug builds
    fn eprintln_dbg(&self, msg: &'static str) {
        #[cfg(debug_assertions)]
        eprintln!("{msg}: {self}");
    }
}

impl<T: Display> DisplayExt for T {}

pub trait DebugExt: std::fmt::Debug {
    /// pritnln!("{msg}: {self:?}");
    fn dprintln(&self, msg: &'static str) {
        println!("{msg}: {self:?}");
    }
    /// eprintln!("{msg}: {self:?}");
    fn deprintln(&self, msg: &'static str) {
        #[cfg(debug_assertions)]
        eprintln!("{msg}: {self:?}");
    }
    /// println!("{msg}: {self:?}"); only in debug builds
    fn dprintln_dbg(&self, msg: &'static str) {
        #[cfg(debug_assertions)]
        println!("{msg}: {self:?}");
    }
    /// eprintln!("{msg}: {self:?}"); only in debug builds
    fn deprintln_dbg(&self, msg: &'static str) {
        #[cfg(debug_assertions)]
        eprintln!("{msg}: {self:?}");
    }

    /// println!("{msg}: {self:#?}"); pretty println
    fn dpprintln(&self, msg: &'static str) {
        println!("{msg}: {self:#?}");
    }
    /// eprintln!("{msg}: {self:#?}"); pretty eprintln
    fn depprintln(&self, msg: &'static str) {
        #[cfg(debug_assertions)]
        eprintln!("{msg}: {self:#?}");
    }

    /// println!("{msg}: {self:#?}"); pretty println only in debug builds
    fn dpprintln_dbg(&self, msg: &'static str) {
        #[cfg(debug_assertions)]
        println!("{msg}: {self:#?}");
    }
    /// eprintln!("{msg}: {self:#?}"); pretty eprintln only in debug builds
    fn depprintln_dbg(&self, msg: &'static str) {
        #[cfg(debug_assertions)]
        eprintln!("{msg}: {self:#?}");
    }
    /// dbg!(self)
    fn dbg(&self) -> &Self {
        dbg!(self)
    }
    /// dbg!(self); only in debug builds
    fn dbg_dbg(&self) -> &Self {
        #[cfg(debug_assertions)]
        dbg!(self);
        self
    }
    fn dbg_tagged(&self, tag: &str) -> &Self {
        dbg!(tag, self);
        self
    }
    fn dbg_tagged_dbg(&self, tag: &str) -> &Self {
        #[cfg(debug_assertions)]
        dbg!(tag, self);
        self
    }
}

impl<T: std::fmt::Debug> DebugExt for T {}
