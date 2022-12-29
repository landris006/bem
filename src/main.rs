use chrono::{Datelike, Local, NaiveDate, Weekday};
use select::{
    document::Document,
    predicate::{Class, Name, Predicate},
};
use std::{env, process::exit};

#[tokio::main]
async fn main() {
    let url = "http://www.bemsorozo.hu/heti_menu.htm";
    let res = reqwest::get(url)
        .await
        .expect("error making http GET request")
        .text()
        .await
        .expect("error getting response text");

    let document = Document::from(res.as_str());
    let rows = document.find(Class("MsoTableGrid").descendant(Name("tbody").child(Name("tr"))));

    let table: Vec<Vec<String>> = rows
        .map(|row| {
            row.find(Name("td"))
                .map(|cell| {
                    (*cell
                        .find(Name("p"))
                        .next()
                        .expect("unexpected table structure")
                        .text()
                        .trim())
                    .to_string()
                })
                .collect()
        })
        .collect();

    let date = Local::now();

    let menus: Vec<Menu> = table[2]
        .iter()
        .zip(&table[3])
        .enumerate()
        .map(|(index, (first_course, second_course))| Menu {
            date: NaiveDate::from_isoywd_opt(
                date.year(),
                date.iso_week().week(),
                match index {
                    0 => Weekday::Mon,
                    1 => Weekday::Tue,
                    2 => Weekday::Wed,
                    3 => Weekday::Thu,
                    4 => Weekday::Fri,
                    _ => panic!("unexpected table structure"),
                },
            )
            .expect("invalid date"),
            first_course: first_course.clone(),
            second_course: second_course.clone(),
        })
        .collect();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        let day_index = date.weekday().num_days_from_monday();
        if day_index > 4 {
            println!("No daily menu on weekends");
            exit(0);
        }

        menus[day_index as usize].display();
        exit(0);
    }

    match args[1].as_str() {
        "all" => menus.iter().enumerate().for_each(|(index, menu)| {
            menu.display();

            if index >= menus.len() - 1 {
                return;
            }
            println!("");
            println!("");
        }),
        "web" => {
            open::that(url).expect("could not open website");
            exit(0);
        }
        _ => {
            println!("No such argument");
        }
    }
}

struct Menu {
    date: NaiveDate,
    first_course: String,
    second_course: String,
}
impl Menu {
    fn display(&self) {
        let header = format!(
            "{} ({})",
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
        println!("\x1b[93m{}\x1b[0m", header);
        println!("\x1b[93m{}\x1b[0m", "‾".repeat(header.chars().count()));
        println!(
            "1. {}",
            self.first_course.replace("\n", "").replace("  ", " ")
        );
        println!(
            "2. {}",
            self.second_course.replace("\n", "").replace("  ", " ")
        );
    }
}
