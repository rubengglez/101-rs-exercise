//! Adapted from https://github.com/ferrous-systems/teaching-material/blob/main/assignments/serde-lifetimes.adoc

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// pretend that we call an API and get a JSON String back
fn fetch_data() -> String {
    String::from(
        r#"
            {
                "id": 1,
                "title": "Hello, Rust"
            }
        "#,
    )
}

fn fetch_data2() -> String {
    String::from(
        r#"
            {
                "id": 1,
                "title": "Hello, Rust from post2"
            }
        "#,
    )
}

fn fetch_data3() -> String {
    String::from(
        r#"
            {
                "id": 1,
                "title": "Hello, Rust"
            }
        "#,
    )
}

#[derive(Debug, Deserialize, Serialize)]
struct BlogPost {
    id: u32,
    title: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct BlogPost2<'a> {
    id: u32,
    title: &'a str,
}

#[derive(Debug, Deserialize, Serialize)]
struct BlogPost3<'a> {
    id: u32,
    #[serde(borrow)]
    title: Cow<'a, str>,
}

fn main() -> anyhow::Result<()> {
    let post: BlogPost = {
        let data = fetch_data();
        let v: BlogPost = serde_json::from_str(&data)?;

        v
    };
    println!("deserialized = {:?}", post);

    let post_json: String = serde_json::to_string(&post)?;
    println!("serialized = {:?}", post_json);

    // Data needs to be placed here because if not it will be
    // destroyed once the scope ends. Since we are using references
    // from the JSON object, we need data to live the whole scope.
    let data = fetch_data2();
    let post2: BlogPost2 = {
        let v: BlogPost2 = serde_json::from_str(&data)?;

        v
    };
    println!("deserialized = {:?}", post2);

    let post2_json: String = serde_json::to_string(&post2)?;
    println!("serialized = {:?}", post2_json);

    let data = fetch_data3();
    let post3: BlogPost3 = {
        let v: BlogPost3 = serde_json::from_str(&data)?;

        v
    };
    println!("is borrowed = {}", matches!(post3.title, Cow::Borrowed(_)));
    println!("deserialized = {:?}", post3);

    let post3_json: String = serde_json::to_string(&post3)?;
    println!("serialized = {:?}", post3_json);

    Ok(())
}
