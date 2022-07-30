// #[derive(Clone, Copy)]
struct BreakfastMenu {
    items: Vec<MenuItem>
}

impl BreakfastMenu {
    
    fn add_item(&mut self, name:String, description:String, is_veggy: bool, price: f32) {
        let item = MenuItem::new(name, description, is_veggy, price);
        self.items.push(item);
    }

    fn get_items(&self) -> &Vec<MenuItem>{
        &self.items
    }

    fn new()-> Self {
        Self{items: Vec::<MenuItem>::new()}
    }
}
struct MainMenu{
    
    max_length: usize,
    length: usize,
    items: [MenuItem; 6]
}

impl MainMenu {

    fn add_item(&mut self, name:String, description: String, is_veggy: bool, price: f32){

        if self.length < self.max_length {
            
            let item = MenuItem::new(name, description, is_veggy, price);
            self.items[self.length] = item;
            self.length += 1;

        }
        else{
            println!("Failed to add the item {} due to item limit.", &name)
        }    

    }

    fn new() -> Self {
        
        const MAX_LENGTH: usize = 6;
        let length = 0;
        let items = [MenuItem::default(); MAX_LENGTH];
        let max_length = MAX_LENGTH;
        Self{length, items, max_length}
    }

}

#[derive(Clone)]
struct MenuItem{
    
    name: String,
    description: String, 
    is_veggy: bool,
    price: f32

}
impl MenuItem{
    
    fn new(name:String, description:String, is_veggy: bool, price: f32) -> Self{
        let name = name.into();
        let description = description.into();
        Self{name, description, is_veggy, price}
    }
}

impl Default for MenuItem {
    fn default() -> Self {
        MenuItem{
        name: String::from(""),
        description: String::from(""),
        is_veggy: false,
        price: 0 as f32
    }
}

}


fn main() {
    println!("Hello, world!");
}

