use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let json = r#"
    {
        "article": "This is an article",
        "author": "John Doe",
        "paragraph": [
            {  
                "name": "paragraph1"
            },
            {  
                "name": "paragraph2"
            },
            {  
                "name": "paragraph3"
            }
        ]        
    }
    "#;

    let parsed: Article = read_json_typed(json);
    println!("The name of the paragraph is {}", parsed.paragraph[0].name);
}

fn read_json_typed(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed;
}
