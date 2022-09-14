use libloading::{Library, Symbol};
use std::{path::Path, sync::Arc};

#[cfg(target_os = "linux")]
pub fn lib_pth() -> &'static Path {
    Path::new("target/release/libffi_lib.so")
}

#[cfg(target_os = "windows")]
pub fn lib_pth() -> &'static Path {
    Path::new("target/release/libffi_lib.dll")
}
#[cfg(target_os = "macos")]
pub fn lib_pth() -> &'static Path {
    Path::new("target/release/libffi_lib.dylib")
}

#[repr(C)]
struct SmartOutlet {
    is_on: bool,
    consumption: f32,
}

type NewSmartOutletFn = unsafe extern "C" fn() -> *mut SmartOutlet;
type ToggleSmartOutletFn = unsafe extern "C" fn(*mut SmartOutlet);
type GetSmartOutletStateFn = unsafe extern "C" fn(*const SmartOutlet) -> (bool, f32);

fn lib() -> Library {
    unsafe { Library::new(lib_pth()).unwrap() }
}

fn new_smart_outlet(lib: Arc<Library>) -> *mut SmartOutlet {
    unsafe {
        let fnc: Symbol<NewSmartOutletFn> = lib.get(b"smart_outlet_new").unwrap();
        fnc()
    }
}

fn toggle_state(lib: Arc<Library>, smart_outlet: *mut SmartOutlet) {
    unsafe {
        let fnc: Symbol<ToggleSmartOutletFn> = lib.get(b"smart_outlet_toggle").unwrap();
        fnc(smart_outlet)
    }
}

fn get_state(lib: Arc<Library>, smart_outlet: *const SmartOutlet) -> (bool, f32) {
    unsafe {
        let fnc: Symbol<GetSmartOutletStateFn> = lib.get(b"smart_outlet_get_state").unwrap();
        fnc(smart_outlet)
    }
}

#[derive(Clone)]
struct Lib {
    lib: Arc<Library>,
}

fn main() {
    let lib = lib();
    let arclib = Lib { lib: Arc::new(lib) };
    let smart_outlet = new_smart_outlet(arclib.clone().lib);
    toggle_state(arclib.clone().lib, smart_outlet);
    get_state(arclib.clone().lib, smart_outlet);
}
