//! Posts are plain markdown files under `content/`, embedded at compile time.
//! They keep Zola's `+++` TOML front matter so the existing files stay as they are.

pub struct Post {
    pub slug: &'static str,
    pub title: String,
    pub date: String,
    pub body: String,
}

const RAW: &[(&str, &str)] = &[("2023-01-02", include_str!("../content/posts/2023-01-02.md"))];

pub const ABOUT: &str = include_str!("../content/about.md");

pub fn all() -> Vec<Post> {
    let mut posts: Vec<Post> = RAW.iter().map(|(slug, raw)| parse(slug, raw)).collect();
    posts.sort_by(|a, b| b.date.cmp(&a.date));
    posts
}

fn parse(slug: &'static str, raw: &str) -> Post {
    let (front, body) = raw
        .strip_prefix("+++")
        .and_then(|rest| rest.split_once("+++"))
        .unwrap_or(("", raw));
    let field = |key: &str| {
        front
            .lines()
            .find_map(|l| l.trim().strip_prefix(key)?.trim_start().strip_prefix('='))
            .map(|v| v.trim().trim_matches('"').to_owned())
    };
    Post {
        slug,
        title: field("title").unwrap_or_else(|| slug.to_owned()),
        date: field("date").unwrap_or_default(),
        body: body.trim().to_owned(),
    }
}
