use std::fs;

use serde::{Deserialize, Serialize};

use crate::EmailType::{Fak, FakCc, Statment, StatmwentCc};

fn main() {
    let temp = ResultData::new(Fak("5545".to_string(), "mk".to_string()));
    println!("{:?}", serde_json::to_string(&temp).unwrap());

    let file = fs::File::open("input.json").expect("need the file");

    let json_list: Vec<InputData> =
        serde_json::from_reader(file).expect("file should be proper JSON");

    let mut res: Vec<ResultData> = Vec::new();

    for json in json_list {
        if !json.faktura_credit_nota_to.is_empty() {
            let gg = ResultData::new(Fak(temp.customer_no.clone(), json.faktura_credit_nota_to));
            // println!("{:?}", serde_json::to_string(&gg).unwrap());
            res.push(gg);
        }
        if !json.faktura_credit_nota_cc.is_empty() {
            let gg = ResultData::new(FakCc(temp.customer_no.clone(), json.faktura_credit_nota_cc));
            // println!("{:?}", serde_json::to_string(&gg).unwrap());
            res.push(gg);
        }
        if !json.statment_to.is_empty() {
            let gg = ResultData::new(Statment(temp.customer_no.clone(), json.statment_to));
            // println!("{:?}", serde_json::to_string(&gg).unwrap());
            res.push(gg);
        }
    }

    println!("{:?}", res);

    let res_file = fs::File::create("result.json").expect("need that file okay");

    serde_json::to_writer_pretty(res_file, &res).expect("failed to create json file");
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
}

#[derive(Serialize)]
pub enum EmailType {
    Fak(String, String),
    FakCc(String, String),
    Statment(String, String),
    StatmwentCc(String, String),
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
            StatmwentCc(customer_no, email) => Self {
                customer_no,
                email_type: 4,
                email,
            },
        }
    }
}
