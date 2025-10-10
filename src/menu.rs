static CUSTOM_ORDER: [&str; 4] = [
    "Ausgabe 1",
    "Ausgabe 2",
    "Ausgabe 3",
    "Auflauf/Gratin",
];

/// A meal with a name and a price.
pub struct Meal {
    pub name: String,
    pub price: String,
    pub counter: String
}

pub struct Counter {
    pub name: String,
    pub meals: Vec<Meal>,
}

/// A menu with a name and a list of meals.
pub struct Menu {
    name: String,
    counters: Vec<Counter>,
}


impl Counter {
    /// Create a new counter with a given name.
    pub fn print(&self) {
        println!("{}", self.name);
        for meal in &self.meals {
            println!("   {} - {}", meal.name, meal.price);
        }
    }
}

impl Menu {
    /// Create a new menu with a given name.
    pub fn new(name: String) -> Menu {
        Menu {
            name,
            counters: Vec::new(),
        }
    }

    /// Add a meal to the menu.
    /// Arguments:
    /// - `meal`: The meal to add.
    pub fn add_meal(&mut self, meal: Meal) {
        let counter = meal.counter.clone();
        if let Some(cat) = self.counters.iter_mut().find(|c| c.name == counter) {
            cat.meals.push(meal);
        } else {
            let meals = vec![meal];
            self.counters.push(Counter { name: counter, meals });
        }
    }

    /// Print the menu to the console.
    pub fn print(&self) {
        println!("\n=== {} ===", self.name);
        for cat_name in CUSTOM_ORDER {
            if let Some(cat) = self.counters.iter().find(|c| c.name == cat_name) {
                cat.print();
            }
        }
        for cat in &self.counters {
            if !CUSTOM_ORDER.contains(&cat.name.as_str()) {
                cat.print();
            }
        }
    }
}
