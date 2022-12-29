use chrono::{DateTime, Datelike, Local, Weekday};
use scraper::{ElementRef, Html, Selector};
use std::{ops::Add, process::exit};

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

    let date = chrono::offset::Local::now();
    let weekday_index = date.weekday().num_days_from_monday();

    if weekday_index > 4 {
        println!("Check back on a weekday!");
        exit(0);
    }

    let menus: Vec<Menu> = table[2]
        .iter()
        .zip(&table[3])
        .map(|(first_course, second_course)| Menu {
            date,
            first_course: (first_course.to_string()),
            second_course: second_course.to_string(),
        })
        .collect();

    menus.iter().for_each(|menu| menu.display());
}

struct Menu {
    date: DateTime<Local>,
    first_course: String,
    second_course: String,
}
impl Menu {
    fn display(&self) {
        println!(
            "\x1b[93m{} ({})\x1b[0m",
            match self.date.weekday() {
                Weekday::Mon => "Hétfő",
                Weekday::Tue => "Kedd",
                Weekday::Wed => "Szerda",
                Weekday::Thu => "Csütörtök",
                Weekday::Fri => "Péntek",
                _ => "",
            },
            self.date.format("%Y.%m.%d")
        );
        println!(
            "1. {}",
            self.first_course /* .replace("\n", "").replace("  ", " ") */
        );
        println!(
            "2. {}",
            self.second_course /* .replace("\n", "").replace("  ", " ") */
        );
    }
}
