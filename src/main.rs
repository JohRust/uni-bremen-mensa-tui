mod menu;
use scraper::{Html, Selector};
use reqwest;


fn main() {
    println!("Today is a good day to get fat on campus:");
    let menu = get_menu_mensa().unwrap();
    menu.print();
}

fn get_menu_mensa() -> Result<menu::Menu, Box<dyn std::error::Error>> {
    let mut menu = menu::Menu::new("Uni Mensa".to_string());
    let url = "https://www.stw-bremen.de/de/mensa/uni-mensa";
    let body = reqwest::blocking::get(url)?.text()?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse(".view-dom-id-1571de99bb5b3a18eee33f35c2dac288 > table:nth-child(2) > tbody:nth-child(2) > tr:nth-child(1) > td:nth-child(2)").unwrap();
    for element in document.select(&selector) {
        let name = element.text().collect::<Vec<_>>().join(" ");
        let price = element.value().attr("data-price").unwrap();
        let food = menu::Food {
            name,
            price: price.to_string(),
        };
        menu.add_food(food);
    }
    Ok(menu)
}
