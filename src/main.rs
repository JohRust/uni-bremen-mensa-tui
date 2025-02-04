mod menu;
use reqwest::{self};
use chrono;

fn main() {
    println!("Today is a good day to get fat on campus:");
    let menu = get_menu_mensa().unwrap();
    menu.print();
}

fn get_menu_mensa() -> Result<menu::Menu, Box<dyn std::error::Error>> {
    let mut menu = menu::Menu::new("Uni Mensa".to_string());
    let url = "https://content.stw-bremen.de/api/kql";
    let date_today = chrono::Local::now().format("%Y-%m-%d").to_string(); 
    let client = reqwest::blocking::Client::new();
    let mut req = client.post(url);
    req = req.bearer_auth("SiERWGuZbj/Ud0AqSp21cDX/GIUJqnKG!MgkW-Zg7QzCO0NT1YjkO-N1Bc1aUssM");
    req = req.json(&serde_json::json!({
        "query": format!("page(\'meals\').children.filterBy(\'location\', \'300\').filterBy(\'date\', \'{date_today}\')"),
        "select":{
            "title":true,
            "ingredients":"page.ingredients.toObject",
            "prices":"page.prices.toObject",
            "counter":true,
            "date":true,
            "mealadds":true,
            "mark":true,
            "kombicategory":true,
            "categories":"page.categories.split"
        }
    }));
    let res = req.send()?;
    // parse JSON body
    if res.status() != 200 {
        println!("Error: {}", res.status());
        return Ok(menu);
    }
    let body = &res.json::<serde_json::Value>()?["result"];
    let meals = body.as_array().unwrap();
    for meal_json in meals {
        let meal = meal_json.as_object().unwrap();

        let meal_title = meal["title"].as_str().unwrap();
        let meal_counter = meal["counter"].as_str().unwrap();
        let menu_entry = menu::Food {
            name: format!("{meal_counter}: {meal_title}"),
            price: meal["prices"][2]["price"].as_str().unwrap().trim().to_string(),
        };
        menu.add_food(menu_entry);
    }
    //let meals = body["result"].as_array().unwrap();
    Ok(menu)
}
