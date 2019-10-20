extern crate reqwest;

use serde::Deserialize;
use std::io::Read;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::LockResult;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::thread;

pub fn get_top_stories(limit: usize) -> Result<Vec<HNItem>, Box<std::error::Error>> {
    let mut vec: Vec<i64> =
        reqwest::get("https://hacker-news.firebaseio.com/v0/topstories.json")?.json()?;
    let mut items = Vec::new();

    let lock: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let mut handles: Vec<thread::JoinHandle<HNItem>> = Vec::new();
    for i in 0..limit {
        let mut lock2 = lock.clone();
        let vec2 = vec.clone();
        handles.push(thread::spawn(move || {
            let url = format!(
                "https://hacker-news.firebaseio.com/v0/item/{}.json",
                vec2[i],
            );
            let item: HNItem = reqwest::get(url.as_str()).unwrap().json().unwrap();
            item
        }));
    }

    for handle in handles.into_iter() {
        let item = handle.join().unwrap();
        items.push(item);
    }
    Ok(items)
}

#[derive(Deserialize, Debug)]
pub struct HNItem {
    id: i64,
    by: String,
    descendants: i64,
    kids: Option<Vec<i64>>,
    score: i64,
    time: i64,
    title: String,
    r#type: String,
    url: Option<String>,
}
