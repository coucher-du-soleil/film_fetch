use select::document::Document;
use select::node::Node;
use select::predicate::Name;
use reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Replace these keywords with your desired search terms
    let keywords = ["action", "adventure", "sci-fi"];

    // Build the IMDb search URL
    let search_url = format!(
        "https://www.imdb.com/search/keyword/?keywords={}",
        keywords.join("+")
    );

    // Send an HTTP GET request to the search URL
    let response = reqwest::get(&search_url).await?;

    // Read the response body as bytes
    let body = response.bytes().await?;

    // Convert the response body bytes to a string
    let body_str = String::from_utf8_lossy(&body);

    // Parse the HTML document
    let document = Document::from(body_str.as_ref());

    // Find movie titles and print them
    for node in document.find(Name("h3")) {
        if let Some(title) = extract_movie_title(&node) {
            println!("Movie: {}", title);
        }
    }

    Ok(())
}

fn extract_movie_title(node: &Node) -> Option<String> {
    let title_node = node.find(Name("a")).next()?;
    let title_text = title_node.text();
    Some(title_text.trim().to_string())
}
