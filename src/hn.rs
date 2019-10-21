extern crate reqwest;
extern crate num_cpus;

use serde::Deserialize;
use std::io::Read;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::LockResult;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::thread;

fn next(cursor: &mut Arc<Mutex<usize>>) -> usize {
    let result: LockResult<MutexGuard<usize>> = cursor.lock();
    let mut guard: MutexGuard<usize> = result.unwrap();
    let mut temp = guard.deref_mut();
    *temp = *temp+1;
    return *temp;
}

pub fn get_top_stories(limit: usize) -> Result<Vec<Story>, Box<std::error::Error>> {
    let mut vec: Vec<i64> =
        reqwest::get("https://hacker-news.firebaseio.com/v0/topstories.json")?.json()?;

    vec.truncate(limit);
    let lock: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let mut handles: Vec<thread::JoinHandle<Vec<Story>>> = Vec::new();
    let lock: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));

    for i in 0..num_cpus::get() {
        let mut lock2 = lock.clone();
        let vec2 = vec.clone();
        handles.push(thread::spawn(move || {
            let mut stories = Vec::new();
            loop {
                let cursor = next(&mut lock2);

                if cursor >= vec2.len() {
                    break;
                }
                let url = format!(
                    "https://hacker-news.firebaseio.com/v0/item/{}.json",
                    vec2[i],
                );
                let story: Story = reqwest::get(url.as_str()).unwrap().json().unwrap();
                stories.push(story);
            };
            stories
        }));
    }

    let mut ret = Vec::new();
    for handle in handles.into_iter() {
        let mut res = handle.join().unwrap();
        ret.append(&mut res);
    }
    Ok(ret)
}

#[derive(Deserialize, Debug)]
pub struct Story {
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
