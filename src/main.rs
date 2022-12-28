use chrono::Datelike;
use scraper::{Html, Selector};
use std::process::exit;

#[tokio::main]
async fn main() {
    let res = reqwest::get("http://www.bemsorozo.hu/heti_menu.htm")
        .await
        .expect("error making http GET request")
        .text()
        .await
        .expect("error getting response text");

    let document = Html::parse_document(&res);
    let selector = Selector::parse("table.MsoTableGrid>tbody>tr").unwrap();
    let rows = document.select(&selector);

    let table: Vec<Vec<&str>> = rows
        .map(|row| {
            let selector = Selector::parse("td>p:first-child").unwrap();
            let cells = row.select(&selector);

            return cells
                .map(|cell| cell.text().next().unwrap().trim())
                .collect();
        })
        .collect();

    let weekday = chrono::offset::Local::now().weekday();
    let weekday_index = weekday.num_days_from_monday();

    if weekday_index > 4 {
        println!("Check back on a weekday!");
        exit(0);
    }

    let first_course = table[2][weekday_index as usize];
    let second_course = table[3][weekday_index as usize];
    println!("{}", weekday);
    println!(
        "\x1b[93m{:?}\x1b[0m",
        first_course.replace("\n", "").replace("  ", " ")
    );
    println!(
        "\x1b[93m{:?}\x1b[0m",
        second_course.replace("\n", "").replace("  ", " ")
    );
    /* println!("\x1b[93m{:?}\x1b[0m"); */
}
