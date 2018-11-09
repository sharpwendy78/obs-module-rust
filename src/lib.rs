extern crate libc;
use std::ptr;
use std::os::raw::c_void;

#[allow(non_camel_case_types)]
type obs_module_t = *mut c_void;

struct OBSRustModule {
    pointer: obs_module_t
}
unsafe impl Sync for OBSRustModule {}

const MODULE_NAME: &str = "obs-module-rust\0";
const MODULE_DESCRIPTION: &str = "Example of an OBS module written in Rust\0";
const MODULE_AUTHOR: &str = "Stéphane Lepin\0";

static mut THIS_MODULE: OBSRustModule = OBSRustModule {
    pointer: ptr::null_mut()
};

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
pub extern fn obs_module_name() -> *const u8
{
    MODULE_NAME.as_ptr()
}

#[no_mangle]
pub extern fn obs_module_description() -> *const u8
{
    MODULE_DESCRIPTION.as_ptr()
}

#[no_mangle]
pub extern fn obs_module_author() -> *const u8
{
    MODULE_AUTHOR.as_ptr()
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