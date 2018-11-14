#[macro_use]
mod obsmodule {
    macro_rules! obs_declare_module {
        ($name:expr, $description:expr) => {
            #[macro_use]
            extern crate lazy_static;

            use std::ptr;
            use std::sync::{Mutex};
            use std::os::raw::c_void;

            mod libobs;

            const MODULE_NAME: &str = concat!($name, "\0");
            const MODULE_DESCRIPTION: &str = concat!($description, "\0");

            struct OBSRustModule {
                pointer: Option<*const c_void>
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

            #[no_mangle]
            pub extern fn obs_module_set_pointer(module_ptr: *const c_void) -> ()
            {
                THIS_MODULE.lock().unwrap().pointer = Option::Some(module_ptr);
            }

            #[no_mangle]
            pub extern fn obs_current_module() -> *const c_void
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
            pub extern fn obs_module_ver() -> u32
            {
                (
                    (libobs::LIBOBS_API_MAJOR_VER << 24) |
                    (libobs::LIBOBS_API_MINOR_VER << 16) |
                    (libobs::LIBOBS_API_PATCH_VER)
                )
            }
        };
    }

    macro_rules! obs_module_author {
        ($author:expr) => {
            const MODULE_AUTHOR: &str = concat!($author, "\0");

            #[no_mangle]
            pub extern fn obs_module_author() -> *const u8
            {
                MODULE_AUTHOR.as_ptr()
            }
        };
    }
}
