extern crate libc;
#[macro_use]
extern crate lazy_static;
use std::ptr;
use std::sync::{Mutex};
use std::os::raw::c_void;

#[allow(non_camel_case_types)]
type obs_module_t = *const c_void;

struct OBSRustModule {
    pointer: Option<obs_module_t>
}
unsafe impl Sync for OBSRustModule {}
unsafe impl Send for OBSRustModule {}

lazy_static! {
    static ref THIS_MODULE: Mutex<OBSRustModule> = {
        Mutex::new(OBSRustModule {
            pointer: Option::None
        })
    };
}

const MODULE_NAME: &str = "obs-module-rust\0";
const MODULE_DESCRIPTION: &str = "Example of an OBS module written in Rust\0";
const MODULE_AUTHOR: &str = "Stéphane Lepin\0";

#[no_mangle]
pub extern fn obs_module_set_pointer(module_ptr: obs_module_t) -> ()
{
    THIS_MODULE.lock().unwrap().pointer = Option::Some(module_ptr);
}

#[no_mangle]
pub extern fn obs_current_module() -> obs_module_t
{
    THIS_MODULE.lock().unwrap().pointer.unwrap_or(ptr::null())
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