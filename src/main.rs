use rss::Channel;
use std::io::prelude::*;
use html2runes::markdown::convert_string;
use std::fs::File;
use std::path::Path;
use chrono::{DateTime, Utc};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::error::Error;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::OpenOptions;

extern crate core;
extern crate clap;
extern crate rss;
extern crate chrono;
extern crate reqwest;
extern crate html2runes;
static MAILDIR: &'static str = "/home/seventh/Mail/RSS/INBOX/new/";

pub struct RssNewsEntry  <'a> {
    pub feed: String,
    pub title: Option<&'a str>,
    pub link: Option<&'a str>,
    pub date: Option<reqwest::header::HeaderValue>,
    pub body: Option<&'a str>,
    pub filename: Option<String>,
    pub idhash: Option<u64>,
}

impl <'a> Hash for RssNewsEntry <'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.feed.hash(state);
        self.title.hash(state);
        self.link.hash(state);
    }
}

pub struct Feed {
    pub id: u32,
    pub feed_name: String,
    pub url: String
}

fn read_file(curr_file: String) -> File {
    let file = File::open(curr_file).unwrap();
    return file;
}

fn feed_getter(feed: Feed) -> bool {
    let channel = Channel::from_url(&feed.url);
    match channel {
        Err(e) => {
            println!("this feed failed to retrieve: {} {}", feed.url, e);
            return false;
        },
        Ok(channel) => {
            parse_channel(channel, feed.feed_name);
            return true;
        },
    };
}

fn create_filename(uid: u64) -> String {
    let n: DateTime<Utc> = Utc::now();
    let dt = n.format("%Y%m%d%H%M%S%f").to_string();
    let mut fname = String::from(MAILDIR);
    fname.push_str(&dt);
    fname.push_str(&uid.to_owned().to_string());
    return fname;
}

fn create_feedmail<'a>(title: &'a str, feed: &'a str, url: &'a str,
                       datetime: reqwest::header::HeaderValue, body: &'a str) -> RssNewsEntry<'a> {

    RssNewsEntry {
        feed: feed.to_string(),
        title: Some(title),
        link: Some(url),
        date: Some(datetime),
        body: Some(body),
        filename: None,
        idhash: None
    }
}

fn parse_channel(chan: rss::Channel, feed_name: String) {
    for i in chan.items() {
        let body = reqwest::get((&i.link()).to_owned().unwrap()).unwrap();
        let dt = body.headers().get("date").unwrap().to_owned();

        let bodytxt = convert_string(&(reqwest::get((body.url()).to_owned()).unwrap().text()).unwrap());

        let mut fmail = create_feedmail(
            i.title().unwrap(),
            &feed_name,
            i.link().unwrap(),
            dt,
            &bodytxt,
            );

        let mut hasher = DefaultHasher::new();
        fmail.hash(&mut hasher);

        fmail.idhash = Some(hasher.finish());
        fmail.filename = Some(create_filename(fmail.idhash.unwrap()));

        match check_if_db(fmail.idhash) {
            false => {
                save_to_db(fmail.idhash);
                save_to_file(fmail);
            },
            true => println!{"in DB!"},
        }
    }
}

fn save_to_db(hash: Option<u64>) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("/home/seventh/.rust_rss.txt")
        .unwrap();

    writeln!(file, "{}", hash.unwrap().to_owned().to_string());
}

fn check_if_db(hash: Option<u64>) -> bool {
    let f = File::open("/home/seventh/.rust_rss.txt").unwrap();
    let file = BufReader::new(&f);
    for (_num, line) in file.lines().enumerate() {
        let l = line.unwrap();
        if hash.unwrap().to_owned().to_string() == l {
            println!{"in truth: {}", l};
            return true;
        }
    }
    return false;
}

fn save_to_file(fname: RssNewsEntry) -> bool {
    let cfn = fname.filename.unwrap().to_string();
    let newfn = Path::new(&cfn);

    let display = newfn.display();

    let mut file = match File::create(&newfn) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };

    match file.write_all(fname.body.unwrap().as_bytes()) {
        Err(_) => {
            panic!("couldn't write to {}: {}");
        },
        Ok(_) => println!("successfully wrote "),
    }
    return true;
}

fn main() {
    let opened_file = read_file("./rss.txt".to_string());
    let opened_file = BufReader::new(opened_file);


    for (i, line) in opened_file.lines().enumerate() {
        let mut curr_line = line.unwrap();
        let mut split = curr_line.split(' ');
        let feed = Feed {
            id: i as u32,
            url: split.next().unwrap().to_owned(),
            feed_name: split.next().unwrap().to_owned(),
        };
        let f = feed.url.to_owned();
        match feed_getter(feed) {
            true => {
                println!{"good: {:?}", f};
            },
            false => {
                println!{"this went bad: {:?}", f};
            }
        };
    };
}
