use std::{env, fs};

use serde::{Deserialize, Serialize};

use crate::EmailType::{Fak, FakCc, Statment, StatmentCc};

fn main() {
    let file = fs::File::open("input.json").expect("need the file");

    let json_list: Vec<InputData> =
        serde_json::from_reader(file).expect("file should be proper JSON");

    let mut result_list: Vec<ResultData> = Vec::new();

    for json in json_list {
        if !json.faktura_credit_nota_to.is_empty() {
            let faktura_to =
                ResultData::new(Fak(json.customer_no.clone(), json.faktura_credit_nota_to));
            result_list.push(faktura_to);
        }
        if !json.faktura_credit_nota_cc.is_empty() {
            let faktura_cc =
                ResultData::new(FakCc(json.customer_no.clone(), json.faktura_credit_nota_cc));
            result_list.push(faktura_cc);
        }
        if !json.statment_to.is_empty() {
            let statment_to = ResultData::new(Statment(json.customer_no.clone(), json.statment_to));
            result_list.push(statment_to);
        }
        if !json.statment_cc.is_empty() {
            let statment_cc =
                ResultData::new(StatmentCc(json.customer_no.clone(), json.statment_cc));
            result_list.push(statment_cc);
        }
    }

    let args: Vec<String> = env::args().collect();

    let has = |s: &str| args.iter().any(|arg| arg == s);

    if has("print") || has("-p") || has("--print") {
        println!("{:?}", result_list);
    }

    if has("debug") || has("-d") || has("--debug") {
        dbg!(&result_list);
    }

    if !has("--no-result") && !has("--no-json") && !has("-no") && !has("--no-output") {
        let res_file = fs::File::create("result.json").expect("need that file okay");

        serde_json::to_writer_pretty(res_file, &result_list).expect("failed to create json file");
    }

    println!("completed....")
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResultData {
    pub customer_no: String,
    pub email_type: u8,
    pub email: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InputData {
    pub customer_no: String,
    pub name: String,
    pub faktura_credit_nota_to: String,
    pub faktura_credit_nota_cc: String,
    pub statment_to: String,
    pub statment_cc: String,
}

#[derive(Serialize)]
pub enum EmailType {
    Fak(String, String),
    FakCc(String, String),
    Statment(String, String),
    StatmentCc(String, String),
}

impl ResultData {
    pub fn new(email_type: EmailType) -> Self {
        match email_type {
            Fak(customer_no, email) => Self {
                customer_no,
                email_type: 1,
                email,
            },
            FakCc(customer_no, email) => Self {
                customer_no,
                email_type: 2,
                email,
            },
            Statment(customer_no, email) => Self {
                customer_no,
                email_type: 3,
                email,
            },
            StatmentCc(customer_no, email) => Self {
                customer_no,
                email_type: 4,
                email,
            },
        }
    }
}
