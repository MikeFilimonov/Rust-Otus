#[no_mangle]
pub extern "C" fn nine() -> i32 {
    9
}

#[repr(C)]
pub struct SmartOutlet {
    is_on: bool,
    consumption: f32,
}

impl SmartOutlet {
    pub fn new() -> Self {
        SmartOutlet {
            is_on: false,
            consumption: 0f32,
        }
    }

    pub fn toggle(&mut self) {
        self.consumption = match self.is_on {
            true => 0f32,
            false => 220.1f32,
        };
        self.is_on = !self.is_on;
    }

    pub fn get_state(&self) {
        println!(
            "The outlet is on: {0}, consumes {1} W",
            self.is_on, self.consumption
        )
    }
}

#[no_mangle]
pub extern "C" fn smart_outlet_new() -> *mut SmartOutlet {
    Box::into_raw(Box::new(SmartOutlet::new()))
}

#[no_mangle]
pub unsafe extern "C" fn smart_outlet_dealloc(ptr: *mut SmartOutlet) {
    if ptr.is_null() {
        return;
    }
    Box::from_raw(ptr);
}

#[no_mangle]
pub unsafe extern "C" fn smart_outlet_toggle(ptr: *mut SmartOutlet) {
    let smart_outlet = {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    smart_outlet.toggle();
}

#[no_mangle]
pub unsafe extern "C" fn smart_outlet_get_state(ptr: *const SmartOutlet) {
    let smart_outlet = {
        assert!(!ptr.is_null());
        &*ptr
    };
    smart_outlet.get_state()
}
