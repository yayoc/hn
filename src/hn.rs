extern crate reqwest;

use serde::Deserialize;
use std::ops::Try;

pub fn get_top_stories(limit: usize) -> Result<Vec<HNItem>, Box<std::error::Error>> {
    let mut resp: Vec<i64> =
        reqwest::get("https://hacker-news.firebaseio.com/v0/topstories.json")?.json()?;
    let mut items: Vec<HNItem> = Vec::new();
    resp.drain(limit..);
    resp.iter().for_each(|id| match get_item(&id) {
        Ok(i) => items.push(i),
        Err(e) => println!("{:#?}", e),
    });
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
    url: String,
}

impl Try for HNItem {
    type Ok = ();
    type Error = ();

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn from_error(v: Self::Error) -> Self {
        unimplemented!()
    }

    fn from_ok(v: Self::Ok) -> Self {
        unimplemented!()
    }
}

fn get_item(id: &i64) -> Result<HNItem, Box<std::error::Error>> {
    let url = format!(
        "{}{}{}",
        "https://hacker-news.firebaseio.com/v0/item/", id, ".json"
    );
    let resp: HNItem = reqwest::get(url.as_str())?.json()?;
    Ok(resp)
}
