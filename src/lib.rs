extern crate libc;
use std::ptr;

#[allow(non_camel_case_types)]
pub type obs_module_t = *mut ();

pub struct OBSRustModule {
    pointer: obs_module_t
}
unsafe impl Sync for OBSRustModule {}

static mut THIS_MODULE: OBSRustModule = OBSRustModule { pointer: ptr::null_mut() };

#[no_mangle]
pub extern fn obs_module_set_pointer(module_ptr: obs_module_t) -> ()
{
    unsafe {
        THIS_MODULE.pointer = module_ptr;
    }
}

#[no_mangle]
pub extern fn obs_current_module() -> obs_module_t
{
    unsafe {
        THIS_MODULE.pointer
    }
}

#[no_mangle]
pub extern fn obs_module_ver() -> u32
{
    101010 // TODO use LIBOBS_API_VER from libobs
}

#[no_mangle]
pub extern fn obs_module_load() -> bool
{
    println!("Rust module loaded!");
    true
}

#[no_mangle]
pub extern fn obs_module_unload() -> ()
{
    println!("Rust module unloaded!");
}