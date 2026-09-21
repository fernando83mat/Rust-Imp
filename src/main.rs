// use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Review {
    name : String,
    location : String,
    #[serde(rename = "Date")]
    date : String,
    #[serde(rename = "Rating")]
    rating : String,
    #[serde(rename = "Review")]
    review : String,
    #[serde(rename = "Image_Links", alias = "Image Links")]
    image_links : String,
}

fn main() {
    //  Leer el csv de un archivo
    let mut csv_reader = csv::Reader::from_path("reviews_data.csv").unwrap();
    let reviews_rows = csv_reader
        .deserialize()
        .map(|r| r.unwrap())
        .collect::<Vec<Review>>();  
   // reviews_rows.iter().for_each(|r| println!("{:?}", r));

   println!("Read {} rows", reviews_rows.len());
}