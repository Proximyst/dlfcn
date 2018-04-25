/// A safe Rust wrapper around the RTLD values defined in <dlfcn.h> in the C stdlib.
/// Create a new instance through [`RtldValue::new`].
pub struct RtldValue {
    pub(crate) main: RtldMain,
    pub(crate) ors:  Vec<RtldOr>,
}

impl RtldValue {
    /// Makes a new value with the main value as `main`, of type [`RtldMain`].
    pub fn new(main: RtldMain) -> Self {
        Self {
            main,
            ors: Vec::new(),
        }
    }

    /// Adds a [`RtldOr`] value to the list. It will be ORed on top of [`RtldMain`] upon calling
    /// [`RtldValue::to_libc`].
    pub fn with(mut self, or: RtldOr) -> Self {
        self.ors.push(or);
        self
    }

    /// Converts the RTLD value to a [`libc::c_int`] for use in libc related areas.
    pub fn to_libc(&self) -> ::libc::c_int {
        let mut ret = self.main.to_libc();
        for or in &self.ors {
            ret |= or.to_libc();
        }
        ret
    }
}

/// The RTLD main value to be used.
pub enum RtldMain {
    /// Resolve only binds which are needed upon request. If a symbol isn't ever requested, it
    /// won't be resolved.
    Lazy,

    /// If this is set or the environment variable LD_BIND_NOW is set to a non-empty string, all
    /// unresolved symbols will be resolved before the load returns.
    Now,
}

impl RtldMain {
    /// Maps the [`RtldMain`] value to a [`libc::c_int`].
    pub fn to_libc(&self) -> ::libc::c_int {
        match *self {
            RtldMain::Lazy => ::libc::RTLD_LAZY,
            RtldMain::Now  => ::libc::RTLD_NOW,
        }
    }
}

/// The RTLD OR values to be used with the OR operator (|).
pub enum RtldOr {
    /// The symbols defined in the library will be made available to all other subsequently loaded
    /// libraries. Be careful as this does not make them available to previously loaded ones.
    Global,

    /// This is the exact opposite of [`RtldOr::Global`], as it doesn't make any symbol available
    /// to other loaded libraries. This is the default value of these two.
    Local,

    /// (Requires glibc 2.2 or higher for C libraries to take it into use)
    ///
    /// Do not unload the library during close. This also means subsequent loads of this library
    /// will be ignored.
    NoDelete,

    /// (Requires glibc 2.2 or higher for C libraries to take it into use)
    ///
    /// Do not load the library during opening. This can be used to regain a handle together with
    /// [`RtldOr::Global`], though will in this crate not reopen the table, but rather populate another one.
    NoLoad,

    /// (Requires glibc 2.3.4 or higher for C libraries to take it into use)
    ///
    /// Put this libary's symbols ahead in the preference chain against that of the global scope.
    /// This means it will disregard any global symbol if there is one specifically defined in the
    /// library.
    DeepBind,
}

impl RtldOr {
    /// Maps the [`RtldOr`] value to a [`libc::c_int`].
    pub fn to_libc(&self) -> ::libc::c_int {
        match *self {
            RtldOr::Global   => ::libc::RTLD_GLOBAL,
            RtldOr::Local    => ::libc::RTLD_LOCAL,
            RtldOr::NoDelete => ::libc::RTLD_NODELETE,
            RtldOr::NoLoad   => ::libc::RTLD_NOLOAD,
            RtldOr::DeepBind => ::libc::RTLD_DEEPBIND,
        }
    }
}
