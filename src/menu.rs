/// A food with a name and a price.
pub struct Food {
    pub name: String,
    pub price: String,
}

/// A menu with a name and a list of foods.
pub struct Menu {
    name: String,
    foods: Vec<Food>,
}

impl Menu {
    /// Create a new menu with a given name.
    pub fn new(name: String) -> Menu {
        Menu {
            name,
            foods: Vec::new(),
        }
    }

    /// Add a food to the menu.
    /// Arguments:
    /// - `food`: The food to add.
    pub fn add_food(&mut self, food: Food) {
        self.foods.push(food);
    }

    /// Print the menu to the console.
    pub fn print(&self) {
        println!("{}", self.name);
        for food in &self.foods {
            println!("{}: {}", food.name, food.price);
        }
    }
}
