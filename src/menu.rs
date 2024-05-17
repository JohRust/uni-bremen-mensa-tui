pub struct Food {
    pub name: String,
    pub price: String,
}

pub struct Menu {
    name: String,
    foods: Vec<Food>,
}

impl Menu {
    pub fn new(name: String) -> Menu {
        Menu {
            name,
            foods: Vec::new(),
        }
    }

    pub fn add_food(&mut self, food: Food) {
        self.foods.push(food);
    }

    pub fn print(&self) {
        println!("{}", self.name);
        for food in &self.foods {
            println!("{}: {}", food.name, food.price);
        }
    }
}
