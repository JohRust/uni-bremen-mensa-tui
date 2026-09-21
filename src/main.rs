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

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let client = reqwest::Client::new();
    println!("Today is a good day to get fat on campus!");
    println!("Menu for {}", print_day_str(args.day_offset));
    let (mensa_menu, cafe_central_menu, gw2_menu) = tokio::join!(
        get_menu_studentenwerk(&client, "Mensa", args.day_offset, args.price_category),
        get_menu_studentenwerk(&client, "Cafe Central", args.day_offset, args.price_category),
        get_menu_studentenwerk(&client, "GW2", args.day_offset, args.price_category),
    );

    mensa_menu.unwrap().print();
    cafe_central_menu.unwrap().print();
    gw2_menu.unwrap().print();
}

fn print_day_str(day_offset: i64) -> String {
    let date_today = chrono::Local::now() + chrono::Duration::days(day_offset);
    // Return the day of week along with the full date
    date_today.format("%A, %Y-%m-%d").to_string()
}

/// Fetches the menu for the given location code and day offset.
/// 
/// # Arguments
/// * `location` - The location code for the menu. Can be "Mensa", "GW2", or "Cafe Central".
/// * `day_offset` - The number of days to offset from today.
/// * `price_category` - The price category to fetch.
/// 
/// # Returns
/// A `Result` containing the menu or an error.
async fn get_menu_studentenwerk(client: &reqwest::Client, location: &str, day_offset: i64, price_category: usize) -> Result<menu::Menu, Box<dyn std::error::Error>> {
    if location != "Mensa" && location != "GW2" && location != "Cafe Central" {
        return Err("Invalid location code".into());
    }
    let location_code = if location == "Mensa" { "300" } else if location == "GW2" { "340" } else { "3001" };

    let mut menu = menu::Menu::new(location.to_string());
    let url = "https://content.stw-bremen.de/api/kqlnocache";
    let date_today = chrono::Local::now() + chrono::Duration::days(day_offset);
    let target_day_str = date_today.format("%Y-%m-%d").to_string(); 
    let request_body = serde_json::json!({
        "action": "meals",
        "location": location_code,
        "date": target_day_str,
    });
    let request = client
        .post(url)
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64; rv:156.0) Gecko/20100101 Firefox/156.0")
        .header(reqwest::header::ACCEPT, "*/*")
        .header(reqwest::header::ACCEPT_LANGUAGE, "de,en-US;q=0.9,en;q=0.8")
        .header(reqwest::header::REFERER, "https://www.stw-bremen.de/")
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header("X-Language", "de")
        .header(reqwest::header::ORIGIN, "https://www.stw-bremen.de")
        .bearer_auth("1c4792d057ee90f4cd30c9720292f82989b07003f49424638f575efafd2379e9")
        .json(&request_body)
        .build()?;

    let res = client.execute(request).await?;
    // parse JSON body
    if res.status() != 200 {
        println!("Error: {}", res.status());
        return Ok(menu);
    }
    let body = &res.json::<serde_json::Value>().await?["result"];
    let meals = body.as_array().unwrap();
    for meal_json in meals {
        let meal = meal_json.as_object().unwrap();

        let meal_title = meal["title"].as_str().unwrap();
        let meal_counter = meal["counter"].as_str().unwrap();
        let menu_entry = menu::Meal {
            name: meal_title.trim().to_string(),
            price: meal["prices"][price_category]["price"].as_str().unwrap().trim().to_string(),
            counter: meal_counter.to_string(),
            attributes: menu::MealAttributes::from_mealadds(meal["mealadds"].as_str().unwrap_or(""))
        };
        menu.add_meal(menu_entry);
    }
    Ok(menu)
}
