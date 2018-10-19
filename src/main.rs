use rss::Channel;
use std::fs::File;
use chrono::*;
use std::io::BufReader;
use std::io::prelude::*;
extern crate rss;
extern crate chrono;



// this is the rss parsed post to save to the inbox
pub struct RssPost  <'a> {
    // pub feed: Option<&'a str>,
    pub feed: String,
    pub title: Option<&'a str>,
    pub link: Option<&'a str>,
    pub description: Option<&'a str>,
    pub date: Option<Date<Utc>>,
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

// function that parses the feed
fn parse_channel(chan: rss::Channel, feed_name: String) -> bool {
    // let chan_items = chan.into_items;
    for i in chan.items() {
        println!{"{:?}", i};
        let rsspost = RssPost {
            title: i.title(),
            feed: feed_name.to_owned(),
            link: i.link(),
            description: i.description(),
            date: None
        };
        println!{""};
        println!{"{:?}", rsspost.title};
        println!{"{:?}", rsspost.link};
    }
    return true;
}

// main
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

