#[no_mangle]
pub extern fn nine() -> i32{
    9
}

#[repr(C)]
pub struct SmartOutlet {
    is_on: bool,
    consumption: f32,
}
