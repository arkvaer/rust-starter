use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
}

pub fn test_summary() {
    let post = Post {
        title: "Rust语言简介".to_string(),
        author: "Waver".to_string(),
        content: "Rust棒极了".to_string(),
    };
    let weibo = Weibo {
        username: "waver".to_string(),
        content: "好像微博没Tweet好用".to_string(),
    };
    println!("{}", post.summarize());
    println!("{}", weibo.summarize());
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}

fn return_summariazble() -> impl Summary {
    Weibo {
        username: String::from("waver"),
        content: String::from("Rust 真牛逼"),
    }
}

pub fn test_prelude() {
    let a: i32 = 10;
    let b: u16 = 100;

    let b_ = b.try_into().unwrap();

    if a < b_ {
        println!("Ten is less than one hundred.");
    }
}
