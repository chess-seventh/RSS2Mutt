use std::fmt;
use rss::Channel;
use std::fs::File;
// use std::io::{self, BufReader};
use std::io::BufReader;
use std::io::prelude::*;
use std::io;
extern crate rss;


// this is the rss parsed post to save to the inbox
pub struct RssPost {
    pub author: Option<String>,
    pub title: Option<String>,
    pub feed: Option<String>,
    pub link: Option<String>,
    pub description: Option<String>,
}


// this struct is the output of each feed
pub struct Feed {
    pub id: u32,
    pub url: String
}

// this is the function that will save the parsed feed to the inbox
// I should put in parameters the struct RssPost
// fn send_to_mailbox(feed: String, title: String, link: String, summary: String, pub_parsed: String) -> bool {
//     return true;
// }

// function that reads a file in my directory with all the rss feeds
fn read_file(curr_file: String) -> File {
    let file = File::open(curr_file).unwrap();
    return file;
}

// the function that will get the new information from the feed
fn feed_getter(feed: Feed) {
    let mut channel = Ok(Channel::from_url(&feed.url).unwrap());
//    let mut channel = Channel::from_url(&feed.url)?;
  match channel {
      //Err(e) => return Err(e),
      Err(e) => {
          println!("this feed failed to retrieve: {}", feed.url)
      },
      _ => println!("{:?}", channel),
  };

}

// duh ! dis a main yo !
fn main() {
    let opened_file = read_file("./rss.txt".to_string());
    let opened_file = BufReader::new(opened_file);

    let mut i = 0;

    for line in opened_file.lines() {
        //println!("{}", line.unwrap());
        let feed = Feed {
            id: i,
            url: line.unwrap(),
        };
        feed_getter(feed);
        i = i + 1;
    }



}

