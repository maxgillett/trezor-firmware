use core::slice;

use super::{ffi, obj::Obj};

pub type Func = ffi::mp_obj_fun_builtin_fixed_t;

impl Func {
    /// Convert a "static const" function to a MicroPython object.
    pub const fn as_obj(&'static self) -> Obj {
        // SAFETY:
        //  - We are an object struct with a base and a type.
        //  - 'static lifetime holds us in place.
        //  - MicroPython is smart enough not to mutate `mp_obj_fun_builtin_fixed_t`
        //    objects.
        unsafe { Obj::from_ptr(self as *const _ as *mut _) }
    }
}

// SAFETY: We are in a single-threaded environment.
unsafe impl Sync for Func {}

pub type FuncVar = ffi::mp_obj_fun_builtin_var_t;

impl FuncVar {
    /// Convert variable argument "static const" function to a MicroPython
    /// object.
    pub const fn as_obj(&'static self) -> Obj {
        // SAFETY:
        //  - We are an object struct with a base and a type.
        //  - 'static lifetime holds us in place.
        //  - MicroPython is smart enough not to mutate `mp_obj_fun_builtin_var_t`
        //    objects.
        unsafe { Obj::from_ptr(self as *const _ as *mut _) }
    }
}

// SAFETY: We are in a single-threaded environment.
unsafe impl Sync for FuncVar {}

/// Unpack arguments for variable argument function.
pub fn unpack_args<'a>(n_args: usize, args: *const Obj) -> &'a [Obj] {
    // SAFETY:
    //  - args must be valid and aligned: array of pointers coming from micropython
    //    should always be aligned.
    //  - args points to n_args initialized pointers to mp_obj_t: micropython
    //    ensures that.
    //  - Returned slice must not be mutated (except in UnsafeCell): we don't do
    //    that.
    //  - Size of args must not exceed isize::MAX: there's not enough physical
    //    memory on STM32.
    unsafe { slice::from_raw_parts(args, n_args) }
}
