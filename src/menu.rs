use std::collections::HashMap;

static CUSTOM_ORDER: [&str; 4] = [
    "Ausgabe 1",
    "Ausgabe 2",
    "Ausgabe 3",
    "Auflauf/Gratin",
];

/// A food with a name and a price.
pub struct Food {
    pub name: String,
    pub price: String,
}

pub struct Category {
    pub name: String,
    pub foods: HashMap<String, Food>,
}

/// A menu with a name and a list of foods.
pub struct Menu {
    name: String,
    categories: Vec<Category>,
}


impl Category {
    /// Create a new category with a given name.
    pub fn print(&self) {
        println!("{}", self.name);
        let mut food_list: Vec<&Food> = self.foods.values().collect();
        food_list.sort_by(|a, b| a.name.cmp(&b.name));
        for food in food_list {
            println!("    {} - {}", food.name, food.price);
        }
    }
}

impl Menu {
    /// Create a new menu with a given name.
    pub fn new(name: String) -> Menu {
        Menu {
            name,
            categories: Vec::new(),
        }
    }

    /// Add a food to the menu.
    /// Arguments:
    /// - `food`: The food to add.
    pub fn add_food(&mut self, food: Food) {
        // Split the food name by the first colon
        let parts: Vec<&str> = food.name.splitn(2, ':').collect();
        
        let mut category: String = "Uncategorized".to_string();
        let mut food_name: String = food.name.clone();
        if parts.len() == 2 {
            category = parts[0].trim().to_string();
            food_name = parts[1].trim().to_string();
        }
        if let Some(cat) = self.categories.iter_mut().find(|c| c.name == category) {
            cat.foods.insert(food_name.clone(), Food { name: food_name, price: food.price });
        } else {
            let mut foods = HashMap::new();
            foods.insert(food_name.clone(), Food { name: food_name, price: food.price });
            self.categories.push(Category { name: category, foods });
        }
    }

    /// Print the menu to the console.
    pub fn print(&self) {
        println!("\n=== {} ===", self.name);
        for cat_name in CUSTOM_ORDER {
            if let Some(cat) = self.categories.iter().find(|c| c.name == cat_name) {
                cat.print();
            }
        }
        for cat in &self.categories {
            if !CUSTOM_ORDER.contains(&cat.name.as_str()) {
                cat.print();
            }
        }
    }
}
