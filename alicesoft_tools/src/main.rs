use std::collections::BTreeMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::future::Future;
use std::io::Write;
use std::io::{self, BufRead, BufReader, BufWriter, Error, ErrorKind, Read};
use std::path::Path;
use std::string::String;
use std::{thread, time};
use tokio::prelude::*;

#[tokio::main]
async fn main() {
    //thread::sleep(time::Duration::from_secs(55));
    let strg = read_file("out.txt").unwrap();

    //println!("{}", strg.messages[&4114]);

    //write_temp_file(&strg, "temp.txt").unwrap();

    //println!("{}", strg.messages[&4113]);

    run_translation(&strg).await;

    //write_temp_file(&trans, "translated.txt").unwrap();
}

async fn run_translation(strg: &MessageSet) {
    let rm = RequestManager::new();
    //let mut trans = MessageSet::new();

    let v_str: Vec<String> = strg.messages.values().cloned().collect();
    //let v_i = v_str.iter();

    let mut len_counter = 0;
    let mut finished = false;
    let mut offset = 0;
    let mut cn = 0;
    let mut haha = 0;

    for i in &v_str {
        len_counter += i.len();
        cn += 1;

        if len_counter > 9500 {
            let mut trans = MessageSet::new();
            thread::sleep(time::Duration::from_secs_f64(0.1));
            println!("{} request!", cn + offset);

            let res = rm
                .post_translate(v_str[offset..(offset + cn)].to_vec())
                .await;

            let mut index = 1;
            for c in res.translations {
                trans
                    .messages
                    .entry((offset + index) as u32)
                    .or_insert(match c.text {
                        Some(text) => text,
                        None => "".to_owned(),
                    });
                index += 1;
            }

            offset += cn;
            cn = 0;
            len_counter = 0;
            write_temp_file(&trans, "translated.txt").unwrap();
        }
    }

    // if file finished, but not all submitted to translation
    if offset < v_str.len() && v_str.len() - offset < 300 {
        let mut trans = MessageSet::new();
        thread::sleep(time::Duration::from_secs_f64(0.1));
        println!("{} request!", cn + offset);

        let res = rm
            .post_translate(v_str[offset..(v_str.len() - 1)].to_vec())
            .await;

        let mut index = 1;
        for c in res.translations {
            trans
                .messages
                .entry((offset + index) as u32)
                .or_insert(match c.text {
                    Some(text) => text,
                    None => "".to_owned(),
                });
            index += 1;
        }

        offset += cn;
        cn = 0;
        len_counter = 0;
        write_temp_file(&trans, "translated.txt").unwrap();
    }

    // for i in &res.translations {
    //     println!("{}", i.text);
    // }
    // println!("size: {}", res.translations.len());
}

struct MessageSet {
    messages: BTreeMap<u32, String>,
}

impl MessageSet {
    fn new() -> MessageSet {
        MessageSet {
            messages: BTreeMap::new(),
        }
    }
}

fn read_file(filename: &str) -> io::Result<MessageSet> {
    let lines = read_lines(filename)?;

    let mut storage = MessageSet::new();

    for line in lines {
        if let Ok(line) = line {
            if line.starts_with(";m[") {
                let v1: Vec<&str> = line.split(|c: char| c == ']' || c == '[').collect();
                if v1.len() != 3 {
                    panic!("Suppose '[' or ']' is in text");
                }
                let num: u32 = v1[1].parse().unwrap();

                let v2: Vec<&str> = v1[2].split(|c: char| c == '"').collect();
                let mut text = String::new();
                if v2.len() != 3 {
                    //panic!("Suppose '\"' is in text");
                    for i in 1..v2.len() - 1 {
                        text.push_str(v2[i]);
                    }
                } else {
                    text = v2[1].to_owned();
                }

                storage.messages.entry(num).or_insert(text);
            }
        }
    }
    Ok(storage)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn write_temp_file(strg: &MessageSet, filename: &str) -> io::Result<()> {
    //let mut f = File::create(filename)?;
    let mut f = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(filename)
        .unwrap();

    let mut f = BufWriter::new(f);
    f.write("\n".as_bytes())?;
    for (num, text) in &strg.messages {
        let line = format!("m[{}] = \"{}\"\n", num, text);
        f.write_all(line.as_bytes())?;
    }

    Ok(())
}

use bytes::Bytes;
use hyper::client::HttpConnector;
use hyper::{body::HttpBody as _, Client};
use hyper::{Body, Method, Request};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use std::fs;
use std::str;
#[derive(Serialize, Deserialize)]
struct Config {
    token: String,
    folder: String,
}
#[derive(Serialize, Deserialize)]
struct Urls {
    base_url: String,
    translate: String,
}

#[derive(Serialize, Deserialize)]
struct TranslateRequest {
    folder_id: String,
    texts: Vec<String>,
    targetLanguageCode: String,
    sourceLanguageCode: String,
}

#[derive(Serialize, Deserialize)]
struct TranslateResponse {
    translations: Vec<Translation>,
}
#[derive(Serialize, Deserialize)]
struct Translation {
    text: Option<String>,
    detectedLanguageCode: Option<String>,
}

pub struct RequestManager {
    token: String,
    folder: String,
    urls: Urls,
    client: Client<HttpsConnector<HttpConnector>, Body>,
}

impl RequestManager {
    // Warning IO operations
    pub fn new() -> RequestManager {
        let path = Path::new("config/config.json");
        let contents = fs::read_to_string(path).expect("Error opening config.json");
        let config: Config =
            serde_json::from_str(&contents).expect("Error while parsing config.json");
        let path = Path::new("config/urls.json");
        let contents = fs::read_to_string(path).expect("Error opening urls.json");
        let urls: Urls = serde_json::from_str(&contents).expect("Error while parsing urls.json");

        let connector = HttpsConnector::new();

        RequestManager {
            token: config.token,
            folder: config.folder,
            urls: urls,
            client: Client::builder().build::<_, hyper::Body>(connector),
        }
    }

    // Writes response body from get request to storage
    async fn get_from_url_to_storage(&self, url: &hyper::Uri) {
        let req = Request::builder()
            .method(Method::GET)
            .uri(url)
            .header("Authorization", &self.token)
            .body(Body::from(r#"{"library":"hyper"}"#))
            .expect("Error while constructing request");

        let mut res = self.client.request(req).await.unwrap();

        if !res.status().is_success() {
            println!("Error occured. \n{}", res.status());
        }

        let mut response_data: std::vec::Vec<Bytes> = Vec::new();

        // Stream the body, moving each chunk to vector as we get it
        while let Some(next) = res.data().await {
            //    response_data.push(next.unwrap());
        }
    }

    async fn post_translate(&self, texts: Vec<String>) -> TranslateResponse {
        let tr = TranslateRequest {
            folder_id: self.folder.clone(),
            texts: texts,
            targetLanguageCode: "en".to_owned(),
            sourceLanguageCode: "ja".to_owned(),
        };

        let body = serde_json::to_string(&tr).unwrap();
        let req = Request::builder()
            .method("POST")
            .uri(self.urls.base_url.clone() + &self.urls.translate.clone())
            .header("Authorization", &self.token)
            .body(Body::from(body))
            .expect("Error, while constructing request");

        let mut res = self.client.request(req).await.unwrap();

        if !res.status().is_success() {
            println!("Error occured. \n{}", res.status());
            panic!("status not success!");
        }

        let mut response_data: std::vec::Vec<Bytes> = Vec::new();

        // Stream the body, moving each chunk to vector as we get it
        while let Some(next) = res.data().await {
            response_data.push(next.unwrap());
        }

        let mut full_v: Vec<u8> = Vec::new();
        for i in response_data {
            full_v.extend(i.iter());
        }

        //let mut in_str = String::new();
        //in_str += str::from_utf8(&full_v).unwrap();

        let resp: TranslateResponse = serde_json::from_slice(&full_v).unwrap();

        resp
    }
}
