extern crate libc;

use libc::{c_void, c_int};

#[repr(C)]
pub struct JNINativeInterface {
    reserved0: *mut c_void,
    reserved1: *mut c_void,
    reserved2: *mut c_void,
    reserved3: *mut c_void,
    // much more actually in here for practical JNI code, but not
    // relevant for this very simple example...
}

pub type JNIEnv = *const JNINativeInterface;

#[allow(non_snake_case)]
#[no_mangle]
pub extern fn Java_Adder_add(_jre: *mut JNIEnv, _class: *const c_void, v1: c_int, v2: c_int) -> c_int {
    v1 + v2
}
