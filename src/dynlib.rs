use ::libc::c_void;
use ::std::collections::HashMap;
use ::std::ffi::CString;

/// A handle wrapper with a VTable of symbols found in the library as they're requested.
pub struct Library {
    pub(crate) handle: *mut c_void,
    pub(crate) name:   String,
    pub(crate) table:  HashMap<String, *mut c_void>,
}

impl Library {
    /// Returns an unsafe reference to the handle of the library.
    /// Usually, the [`sym`] and [`drop`] methods are enough, but you're free to use it for
    /// anything else, provided you can dereference it.
    pub unsafe fn handle(&self) -> &*mut c_void {
        &self.handle
    }

    /// Returns an unsafe reference to the VTable of the library.
    /// Usually, the [`sym`] method is enough, though you're free to use it for what you may want
    /// to use it for.
    pub unsafe fn table(&self) -> &HashMap<String, *mut c_void> {
        &self.table
    }

    /// Returns the path of the dynamic library as given to [`open`] or [`new`].
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Requests and finds a symbol in the library.
    /// So long [`RtldOr::Local`] wasn't specified, it will also look at other loaded symbols which
    /// are either in the program itself or were loaded with [`RtldOr::Global`].
    pub fn sym<T>(&mut self, name: String) -> Option<Box<T>> {
        if self.table.contains_key(&name) {
            let gotten = self.table.get(&name);
            unsafe {
                let gotten = Box::from_raw(*gotten.unwrap() as *mut T);
                return Some(gotten)
            }
        }
        if self.handle.is_null() {
            return None
        }
        let name_c = CString::new(name.as_str());
        if name_c.is_err() {
            return None
        }
        let name_c = name_c.unwrap();
        let sym: Box<T>;
        unsafe {
            let sym_c: *mut ::libc::c_void;
            if name.is_empty() {
                let name_c: *const ::libc::c_char = ::std::ptr::null();
                sym_c = ::libc::dlsym(self.handle, name_c);
            } else {
                sym_c = ::libc::dlsym(self.handle, name_c.as_ptr());
            }
            sym = Box::from_raw(sym_c as *mut T);
            self.table.insert(name, sym_c);
        }
        Some(sym)
    }

    /// Makes a safe instance using [`new`] then applies the closure given to it.
    /// It returns true if it was successful, or false if anything went wrong.
    pub fn open(name: String, flags: ::RtldValue, closure: fn(lib: Library)) -> bool {
        let cstr = CString::new(name.as_str());
        if cstr.is_err() {
            return false
        }
        let cstr = cstr.unwrap();
        unsafe {
            let lib = Self::new(cstr, flags);
            if lib.is_none() {
                return false
            }
            closure(lib.unwrap());
        }
        true
    }

    /// Creates a new instance of the library wanted. There are no checks other than
    /// handle checks upon loading it. It is the user's responsibility to pass a valid CString
    /// instance and valid flags.
    pub unsafe fn new(name_c: CString, flags: ::RtldValue) -> Option<Self> {
        let handle = ::libc::dlopen(name_c.as_ptr(), flags.to_libc());
        if handle.is_null() {
            return None
        }

        Some(Self {
            handle,
            name:  name_c.into_string().unwrap(), // dlopen expects UTF-8, so this will never fail
            table: HashMap::new(),
        })
    }
}

impl ::std::ops::Drop for Library {
    /// Closes the handle then drops the entire library.
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe {
                ::libc::dlclose(self.handle);
            }
        }
    }
}
