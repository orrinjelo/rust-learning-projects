use rustop::opts;
use serde_json;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::process;
use std::collections::HashMap;
use reqwest;

/// Utility function to read lines from a file
/// Opens and reads a file, returns a vector of strings 
///  wrapped in a Result
/// 
/// # Arguments
/// filename - String filename path
///
/// # Returns
/// Result of a Vector of Strings
fn lines_from_file(filename: String) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

// #[derive(Debug, Deserialize, Serialize)]
// struct ConversionRate {
//     code: String,
//     value: f32,
// }

#[derive(Debug, Deserialize, Serialize)]
struct ApiCall {
    success: Option<bool>,
    timestamp: Option<u64>,
    base: Option<String>,
    date: Option<String>,
    rates: Option<HashMap<String, f32>>,
    error: Option<ErrorType>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ErrorType {
    code: String,
    message: String
}

#[derive(Debug, Deserialize, Serialize)]
struct Secret {
    api_key: String
}

fn main() {
    let (args, _rest) = opts! {
        synopsis "Look up currency conversion rates.";
        opt amount:f32=1.0, desc:"Amount to convert.";
        opt secrets:String="secrets.json".to_string(), desc:"Secrets file";
        param symbol:String, desc:"Currency symbol.";
    }.parse_or_exit();

    // Load secrets
    let secrets: Secret = serde_json::from_str( &
        match lines_from_file(args.secrets) {
            Ok(x) => x.join("\n"),
            Err(x) => {
                println!("Error: {}", x);
                process::exit(1);
            }
        }
    ).unwrap();

    // Get API request
    let res = reqwest::blocking::get(
        format!(
            "http://api.exchangeratesapi.io/v1/latest?access_key={}&symbols=USD,GBP,EUR,AUD,CAD,PLN,MXN,JPY,{}", 
            secrets.api_key,
            args.symbol.to_uppercase()
        )
    ).unwrap();

    let json_money : ApiCall = serde_json::from_str(&res.text().unwrap()).unwrap();
    // println!("{:?}", json_money);

    // Report errors if necessary
    match json_money.error {
        Some(x) => {
            println!("Error {}: {}", x.code, x.message);
        },
        None => {}
    }

    // Do some look ups and some maths.
    match json_money.rates {
        Some(x) => {
            let divisor: f32;
            if !x.contains_key(&args.symbol.to_uppercase()) {
                println!("I'm not sure what {} is, but for 1 {}:", args.symbol.to_uppercase(), json_money.base.unwrap());
                divisor = 1.;
            } else {
                println!("{} {} is: ", args.amount, args.symbol.to_uppercase());
                divisor = x[&args.symbol.to_uppercase()];
            }
            for curr in x.keys() {
                println!("\t{}: {}", curr, x[curr]/divisor*args.amount);
            }

        },
        None => {}
    }
}
