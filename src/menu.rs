static CUSTOM_ORDER: [&str; 4] = [
    "Ausgabe 1",
    "Ausgabe 2",
    "Ausgabe 3",
    "Auflauf/Gratin",
];

pub struct MealAttributes {
    pub vegan: bool,
    pub vegetarian: bool
}

/// A meal with a name and a price.
pub struct Meal {
    pub name: String,
    pub price: String,
    pub counter: String,
    pub attributes: MealAttributes,
}

/// A counter with a name and a list of meals.
pub struct Counter {
    pub name: String,
    pub meals: Vec<Meal>,
}

/// A menu with a name and a list of counters.
/// Each Menu corresponds to a location (Mensa or GW2).
/// Each Counter corresponds to a counter in the Mensa (e.g. "Ausgabe 1").
/// Each Meal corresponds to a meal served at that counter.
pub struct Menu {
    name: String,
    counters: Vec<Counter>,
}

impl MealAttributes {
    pub fn from_mealadds(mealadds: &str) -> MealAttributes {
        let vegan = mealadds.contains("VG");
        let vegetarian = vegan || mealadds.contains("VT");
        MealAttributes { vegan, vegetarian }
    }
}

impl Meal {
    pub fn print(&self) {
        let attr = if self.attributes.vegan {
            " 🌱"
        } else if self.attributes.vegetarian {
            " 🥛🥚"
        } else {
            ""
        };
        println!("   {} - {}{}", self.name, self.price, attr);
    }
}

impl Counter {
    /// Create a new counter with a given name.
    pub fn print(&self) {
        println!("{}", self.name);
        for meal in &self.meals {
            meal.print();
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
        } else { // counter does not exist yet
            let meals = vec![meal];
            self.counters.push(Counter { name: counter, meals });
        }
    }

    /// Print the menu to the console.
    pub fn print(&self) {
        println!("\n=== {} ===", self.name);
        // First print the counters listed the custom order, then the rest
        for cat_name in CUSTOM_ORDER {
            if let Some(cat) = self.counters.iter().find(|c| c.name == cat_name) {
                cat.print();
            }
        }
        // Print the rest
        for cat in &self.counters {
            if !CUSTOM_ORDER.contains(&cat.name.as_str()) {
                cat.print();
            }
        }
    }
}
