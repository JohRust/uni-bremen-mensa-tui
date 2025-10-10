mod menu;
use reqwest::{self};
use chrono;
use clap::Parser;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[arg(short, long, default_value = "0")]
     price_category: usize,
    #[arg(short, long, default_value = "0")]
     day_offset: i64,
}

fn main() {
    let args = Cli::parse();

    println!("Today is a good day to get fat on campus!");
    let mensa_menu = get_menu_studentenwerk("Mensa", args.day_offset, args.price_category).unwrap();
    mensa_menu.print();
    let gw2_menu = get_menu_studentenwerk("GW2", args.day_offset, args.price_category).unwrap();
    gw2_menu.print();
}

/// Fetches the menu for the given location code and day offset.
/// 
/// # Arguments
/// * `location` - The location code for the menu. Can be "Mensa" or "GW2".
/// * `day_offset` - The number of days to offset from today.
/// * `price_category` - The price category to fetch.
/// 
/// # Returns
/// A `Result` containing the menu or an error.
fn get_menu_studentenwerk(location: &str, day_offset: i64, price_category: usize) -> Result<menu::Menu, Box<dyn std::error::Error>> {
    if location != "Mensa" && location != "GW2" {
        return Err("Invalid location code".into());
    }
    let location_code = if location == "Mensa" { "300" } else { "340" };

    let mut menu = menu::Menu::new(location.to_string());
    let url = "https://content.stw-bremen.de/api/kql";
    let date_today = chrono::Local::now() + chrono::Duration::days(day_offset);
    let target_day_str = date_today.format("%Y-%m-%d").to_string(); 
    let client = reqwest::blocking::Client::new();
    let mut req = client.post(url);
    // This token is from the official website and does not need to be kept secret.
    req = req.bearer_auth("SiERWGuZbj/Ud0AqSp21cDX/GIUJqnKG!MgkW-Zg7QzCO0NT1YjkO-N1Bc1aUssM");
    req = req.json(&serde_json::json!({
        "query": format!("page(\'meals\').children.filterBy(\'location\', \'{location_code}\').filterBy(\'date\', \'{target_day_str}\')"),
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
            price: meal["prices"][price_category]["price"].as_str().unwrap().trim().to_string(),
        };
        menu.add_food(menu_entry);
    }
    Ok(menu)
}
