use rss::Channel;
use std::io::BufReader;
use std::io::prelude::*;
use html2runes::markdown::convert_string;
use std::fs::File;
use std::path::Path;
use chrono::{DateTime, Utc};
// use core::hash::Hash;
use std::error::Error;

extern crate core;
extern crate clap;
extern crate rss;
extern crate chrono;
extern crate reqwest;
extern crate html2runes;
static MAILDIR: &'static str = "/home/seventh/Mail/RSS/INBOX/new/";
// this is the rss parsed post to save to the inbox
pub struct RssPost  <'a> {
    // pub feed: Option<&'a str>,
    pub feed: String,
    pub title: Option<&'a str>,
    pub link: Option<&'a str>,
    pub description: Option<&'a str>,
    pub date: Option<reqwest::header::HeaderValue>,
    pub body: Option<String>,
    pub filename: Option<String>,
    pub idhash: Option<String>,
}

// this struct is the output of each feed
pub struct Feed {
    pub id: u32,
    pub feed_name: String,
    pub url: String
}

// function that reads a file in my directory with all the rss feeds
fn read_file(curr_file: String) -> File {
    let file = File::open(curr_file).unwrap();
    return file;
}

// the function that will get the new information from the feed
fn feed_getter(feed: Feed) {
    let channel = Channel::from_url(&feed.url);
    match channel {
        Err(e) => {
            println!("this feed failed to retrieve: {} {}", feed.url, e)
        },
        Ok(channel) => {
            let res_parse = parse_channel(channel, feed.feed_name);
            match res_parse {
                true => return,
                false => {
                    println!("something went bad");
                    return;
                },
            };
        },
    };
}

fn create_filename() -> String {
    let n: DateTime<Utc> = Utc::now();
    let dt = n.format("%Y%m%d%H%M%S%f").to_string();
    let mut fname = String::from(MAILDIR);
    fname.push_str(&dt);
    fname.push_str("_uid.txt");
    return fname;
}

// function that parses the feed
fn parse_channel(chan: rss::Channel, feed_name: String) -> bool {
    // let chan_items = chan.into_items;
    for i in chan.items() {
        let mut rsspost = RssPost {
            title: i.title(),
            feed: feed_name.to_owned(),
            link: i.link(),
            description: i.description(),
            date: None,
            body: None,
            filename: None,
            idhash: None
        };
        let body = reqwest::get((&rsspost.link).to_owned().unwrap()).unwrap();
        rsspost.date = Some(body.headers().get("date").unwrap().to_owned());
        rsspost.body = Some(convert_string(&(reqwest::get((body.url()).to_owned()).unwrap().text()).unwrap()));
        rsspost.filename = Some(create_filename());

        // check if in DB or not
        save_to_file(rsspost);

        // add to DB if not in DB
    }
    return true;
}

fn save_to_file(rsspost: RssPost) -> bool {
    let cfn = rsspost.filename.unwrap().to_string();
    let newfn = Path::new(&cfn);

    let display = newfn.display();

    let mut file = match File::create(&newfn) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };

    match file.write_all(rsspost.body.unwrap().as_bytes()) {
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


    for (i, line) in opened_file.lines().enumerate() {    // Nor
        let mut curr_line = line.unwrap();
        let mut split = curr_line.split(' ');
        let feed = Feed {
            id: i as u32,
            url: split.next().unwrap().to_owned(),
            feed_name: split.next().unwrap().to_owned(),
        };
        feed_getter(feed);
    };

}

