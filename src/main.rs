use rss::Channel;
use std::fs::File;
use std::io::BufReader;
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
    pub id: i32,
    pub title: String,
    pub url: String,
}

// this is the function that will save the parsed feed to the inbox
// I should put in parameters the struct RssPost
fn send_to_mailbox(feed: String, title: String, link: String, summary: String, pub_parsed: String) -> bool {
    return true;
}

// function that reads a file in my directory with all the rss feeds
fn read_file(curr_file: String) -> File {
    let file = File::open(curr_file).unwrap();
    return file;
}

// the function that will get the new information from the feed
fn feed_getter() {

}

// duh ! dis a main yo !
fn main() {
    println!("Hello, world!");  // what else ?
    let opened_file = read_file("/home/seventh/.rsst/rss.txt".to_string());
}
