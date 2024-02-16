// TODO: consider using futures::future::join_all for async iteration

use std::env;

use calgary_central_library::{AsyncClient, Availability, Client, Room};
use chrono::{DateTime, NaiveDate};

// let's set up the sequence of steps we want the browser to take
#[tokio::main]
async fn main() {
    let host =
        env::var("CHROMEDRIVER_HOST").expect("Environment variable CHROMEDRIVER_HOST is not set");
    let port =
        env::var("CHROMEDRIVER_PORT").expect("Environment variable CHROMEDRIVER_PORT is not set");
    let port = port
        .parse::<u16>()
        .expect("Failed to parse CHROMEDRIVER_PORT as a u16");
    println!("Connecting to WebDriver at http://{host}:{port}...");
    let c = Client::new((&host, port)).await.unwrap();

    let now: DateTime<chrono::Local> = chrono::Local::now();
    let today: NaiveDate = now.date_naive();

    let mut s = String::new();

    println!("For how many days ahead would you like to check the availability?");
    let days_ahead = {
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim()
            .parse::<u8>()
            .expect("Failed to parse the number of days ahead")
    };

    s.clear();

    let mut day = today;
    for _ in 0..days_ahead {
        day = day.succ_opt().unwrap();
    }

    println!("Checking availability for {day:?}");

    println!("What's the expected number of attendees? (Default: 10)");
    let attendance: u8 = 'attendance: {
        std::io::stdin().read_line(&mut s).unwrap();
        if s.trim().is_empty() {
            break 'attendance 10;
        }
        s.trim()
            .parse::<u8>()
            .expect("Failed to parse the number of attendees")
    };

    let available_rooms: Vec<(Room, Availability)> =
        c.available_rooms(day, attendance).await.unwrap();
    for (room, availability) in available_rooms {
        println!("{room:?}");
        println!("{availability}");
    }

    c.close().await.unwrap();
}
