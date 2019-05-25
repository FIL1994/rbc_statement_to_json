use chrono::NaiveDate;
use serde::Serialize;
use std::fs::File;


#[derive(Debug, Serialize)]
struct StatementItem {
    account_type: String,
    account_number: String,
    transaction_date: NaiveDate,
    cheque_number: String,
    desc_1: String,
    desc_2: String,
    cad: String,
}

fn main() {
    let mut statements: Vec<StatementItem> = Vec::new();

    let file = File::open("statement.csv").unwrap();
    let mut rdr = csv::ReaderBuilder::new().flexible(true).from_reader(file);
    for result in rdr.records() {
        match result {
            Ok(record) => {
                let date_str = record.get(2).unwrap().to_string();
                let date_vec: Vec<&str> = date_str.split("/").collect();

                let date = NaiveDate::from_ymd(
                    date_vec[2].parse::<i32>().unwrap(),
                    date_vec[0].parse::<u32>().unwrap(),
                    date_vec[1].parse::<u32>().unwrap(),
                );

                let item = StatementItem {
                    account_type: record.get(0).unwrap().to_string(),
                    account_number: record.get(1).unwrap().to_string(),
                    transaction_date: date,
                    cheque_number: record.get(3).unwrap().to_string(),
                    desc_1: record.get(4).unwrap().to_string(),
                    desc_2: record.get(5).unwrap().to_string(),
                    cad: record.get(6).unwrap().to_string(),
                };

                statements.push(item);
            }
            Err(e) => println!("{:?}", e),
        }

    }

    println!("{}", serde_json::to_string(&statements).unwrap());
}
