extern crate num_cpus;
extern crate reqwest;

use chrono::Utc;
use futures::future::join_all;
use futures::Future;
use futures::Stream;
use reqwest::r#async::Client;
use serde::Deserialize;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::channel;

use crate::time;
#[cfg(test)]
use mockito;

lazy_static! {
    pub static ref CLIENT: Client = Client::new();
}

#[derive(Deserialize, Debug)]
pub struct Story {
    pub id: i64,
    by: String,
    descendants: i64,
    kids: Option<Vec<i64>>,
    score: i64,
    time: i64,
    pub title: String,
    r#type: String,
    pub url: Option<String>,
}

impl Story {
    pub fn title_label(&self) -> String {
        let relative_date = time::get_relative_time(self.time, Utc::now().timestamp());
        format!("{} ({})", self.title.as_str(), relative_date)
    }
}

/// Fetch top stories on Hacker News,
/// Using /v0/topstories.json and /v0/item/{:id}.json endpoints.
///
/// https://github.com/HackerNews/API
///
/// # Examples
/// ```
/// let stories = match fetch_top_stories(10) {
///     Ok(res) => res,
///     Err(e) => println!("{:#?}", e)
/// }
/// ```
pub fn fetch_top_stories(num: usize) -> Result<Vec<Story>, reqwest::Error> {
    #[cfg(not(test))]
    let hn_url = "https://hacker-news.firebaseio.com";
    #[cfg(test)]
    let hn_url = &mockito::server_url();

    let top_stories_url = format!("{}{}", hn_url, "/v0/topstories.json");
    let mut vec: Vec<i64> = reqwest::get(top_stories_url.as_str())?.json()?;
    vec = if vec.len() <= num {
        vec
    } else {
        vec[0..num].to_vec()
    };
    fetch_stories(vec)
}

fn fetch_stories(ids: Vec<i64>) -> Result<Vec<Story>, reqwest::Error> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let mut core = Runtime::new().unwrap();
    let (tx, rx) = channel(ids.len());

    let all = ids.into_iter().enumerate().map(move |(i, id)| {
        let mut tx = tx.clone();
        fetch_story(id)
            .then(move |x| tx.try_send((i, x)))
            .map(|_| ())
            .map_err(|e| println!("{:?}", e))
    });
    core.spawn(join_all(all).map(|_| ()));
    let mut stories = Vec::new();
    match rx.take(100 as u64).collect().wait() {
        Ok(mut x) => {
            x.sort_by(|a, b| {
                let (i1, _) = a;
                let (i2, _) = b;
                i1.cmp(i2)
            });
            for s in x {
                let (_, story) = s;
                match story {
                    Ok(st) => stories.push(st),
                    _ => {}
                }
            }
        }
        Err(e) => eprintln!("{:?}", e),
    };
    Ok(stories)
}

fn fetch_story(id: i64) -> impl Future<Item = Story, Error = reqwest::Error> {
    #[cfg(not(test))]
    let hn_url = "https://hacker-news.firebaseio.com";
    #[cfg(test)]
    let hn_url = &mockito::server_url();

    let url = format!("{}/v0/item/{}.json", hn_url, id);
    CLIENT
        .get(url.as_str())
        .send()
        .and_then(move |mut res| res.json())
}

#[cfg(test)]
mod tests {
    use crate::hn::get_top_stories;
    use mockito::mock;

    #[test]
    fn test_get_get_top_stories1() {
        let _m1 = mock("GET", "/v0/topstories.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("[1]")
            .create();

        let _m2 = mock("GET", "/v0/item/1.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("{\"by\":\"pg\",\"descendants\":15,\"id\":1,\"kids\":[15,234509,487171,454426,454424,454410,82729],\"score\":57,\"time\":1160418111,\"title\":\"Y Combinator\",\"type\":\"story\",\"url\":\"http://ycombinator.com\"}")
            .create();

        assert!(
            get_top_stories(1).is_ok(),
            "get_top_stories should return top stories"
        );
        let stories = get_top_stories(1);
        let story = &stories.unwrap()[0];
        assert_eq!(story.by, String::from("pg"));
        assert_eq!(story.id, 1);
    }

    #[test]
    fn test_get_get_top_stories2() {
        let _m1 = mock("GET", "/v0/topstories.json").with_status(500).create();

        assert!(
            get_top_stories(1).is_err(),
            "get_top_stories should return an error"
        );
    }

    #[test]
    fn test_get_get_top_stories3() {
        let _m1 = mock("GET", "/v0/topstories.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("[1,2]")
            .create();

        let _m2 = mock("GET", "/v0/item/1.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("{\"by\":\"pg\",\"descendants\":15,\"id\":1,\"kids\":[15,234509,487171,454426,454424,454410,82729],\"score\":57,\"time\":1160418111,\"title\":\"Y Combinator\",\"type\":\"story\",\"url\":\"http://ycombinator.com\"}")
            .create();

        let _m3 = mock("GET", "/v0/item/2.json").with_status(500).create();

        assert!(
            get_top_stories(5).is_ok(),
            "get_top_stories should return stories."
        );
        let stories = get_top_stories(1);
        let story = &stories.unwrap()[0];
        assert_eq!(story.by, String::from("pg"));
        assert_eq!(story.id, 1);
    }

    #[test]
    fn test_get_get_top_stories4() {
        let _m1 = mock("GET", "/v0/topstories.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("[]")
            .create();
        assert!(
            get_top_stories(5).is_ok(),
            "get_top_stories should return stories."
        );
        let stories = get_top_stories(1);
        assert_eq!(stories.unwrap().len(), 0);
    }
}
