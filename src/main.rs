extern crate hyper;
extern crate hyper_native_tls;
extern crate regex;

use std::io::Read;

use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

use regex::Regex;


fn main() {
    let html = get_html_for("https://www.reddit.com/r/rust/");
    let links = get_links_from(html);

    for link in links {
        println!("{}", link);
    }
}

fn get_html_for(url: &str) -> String {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let http_client = Client::with_connector(connector);

    let mut response = http_client.get(url).send().unwrap();

    let mut html = String::new();
    response
        .read_to_string(&mut html)
        .expect("Couldn't read html for some reason!");

    html
}

fn get_links_from(html: String) -> Vec<String> {
    Regex::new("(?s:.)<a[^>]*href=[\"'](http[^\"']*)[\"']")
        .unwrap()
        .captures_iter(html.as_str())
        .map(|cap| cap[1].to_string())
        .collect()
}
