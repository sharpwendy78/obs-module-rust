const MODULE_NAME: &str = "obs-module-rust\0";
const MODULE_DESCRIPTION: &str = "Example of an OBS module written in Rust\0";
const MODULE_AUTHOR: &str = "Stéphane Lepin\0";

include!("obs-module.rs");

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