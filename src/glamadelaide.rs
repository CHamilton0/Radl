use reqwest::Client;
use select::document::Document;
use select::predicate::{Class, Name};
use rust_bert::pipelines::summarization::SummarizationModel;

#[tokio::main]
pub async fn get_this_week(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    // Create model once before fetching pages
    let model = tokio::task::block_in_place(|| SummarizationModel::new(Default::default()))?;

    // Loop through all the GlamAdelaide pages
    let mut page_number = 1;
    loop {
        let resp = client
            .get(
                "https://glamadelaide.com.au/category/whats_on_in_adelaide/whats-on-this-week/page/"
                    .to_string() + &page_number.to_string(),
            )
            .send()
            .await?;

        if !(resp.status().is_success()) {
            // When the page fails to load, we have loaded all the content
            break;
        };

        let body = resp.text().await?;
        let document = Document::from(body.as_str());

        // Get each post this week
        for post_node in document.find(Class("infinite-post")) {
            let trimmed: String = post_node
                .text()
                .trim()
                .split_once("\n")
                .iter()
                .collect::<Vec<_>>()[0]
                .1
                .chars()
                .filter(|c| *c != '\t')
                .collect();
            let mut lines = trimmed.lines().filter(|l| !l.is_empty());
            
            // Get the title and body from the article
            let title = lines.next().unwrap();
            let body = lines.next().unwrap();
            println!("{title}");
            println!("{body}");

            let output = tokio::task::block_in_place(|| model.summarize(&[body]));
            println!("{:?}", output);

            // Get the first link element in the post node and get the link from it
            if let Some(href) = post_node.find(Name("a")).next().unwrap().attr("href") {
                println!("{}", href);
            }

            println!();
        }

        page_number += 1;
    }

    Ok(())
}
