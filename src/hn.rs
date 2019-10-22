extern crate num_cpus;
extern crate reqwest;

#[cfg(test)]
use mockito;
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
    *temp = *temp + 1;
    return *temp;
}

pub fn get_top_stories(num: usize) -> Result<Vec<Story>, Box<std::error::Error>> {
    #[cfg(not(test))]
    let hn_url = "https://hacker-news.firebaseio.com";
    #[cfg(test)]
    let hn_url = &mockito::server_url();

    let top_stories_url = format!("{}{}", hn_url, "/v0/topstories.json");
    let mut vec: Vec<i64> = reqwest::get(top_stories_url.as_str())?.json()?;
    vec = vec[0..num].to_vec();

    let lock: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let mut handles: Vec<thread::JoinHandle<Vec<Story>>> = Vec::new();
    let lock: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));

    for i in 0..num_cpus::get() {
        let mut lock2 = lock.clone();
        let vec2 = vec.clone();
        let hn_url2 = hn_url.clone();
        handles.push(thread::spawn(move || {
            let mut stories = Vec::new();
            loop {
                let cursor = next(&mut lock2);

                if cursor > vec2.len() {
                    break;
                }

                let story_url = format!("{}/v0/item/{}.json", hn_url2, vec2[i],);
                let story: Story = reqwest::get(story_url.as_str()).unwrap().json().unwrap();
                stories.push(story);
            }
            stories
        }));
    }

    let mut stories = Vec::new();
    for handle in handles.into_iter() {
        let mut res = handle.join().unwrap();
        stories.append(&mut res);
    }
    Ok(stories)
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

#[cfg(test)]
mod tests {
    use crate::hn::get_top_stories;
    use mockito::mock;

    #[test]
    fn test_get_get_top_stories() {
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
}
