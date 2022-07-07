pub struct SmartOutlet {
    pub description: String,
    pub enabled: bool,
    pub consumption: f32,
}

impl SmartOutlet {
    fn _show_description(&self) {
        todo!()
    }

    fn _turn_on(&mut self) {
        todo!()
    }

    fn _turn_off(&mut self) {
        todo!()
    }

    fn _get_current_power_consumption(&self) -> f32 {
        todo!()
    }
}

pub struct SmartThermometer {
    pub current_temperature: f32,
}

impl SmartThermometer {
    fn _get_current_value(&self) -> f32 {
        todo!()
    }
}