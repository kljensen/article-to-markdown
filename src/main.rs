extern crate readability;
use curl::easy::{Easy, List};
use htmd::HtmlToMarkdown;
use readability::extractor;
use url::Url;

fn fetch_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut easy = Easy::new();
    
    // Set the URL
    easy.url(url)?;
    
    // Configure basic settings
    easy.follow_location(true)?;
    easy.max_redirections(10)?;
    easy.ssl_verify_peer(true)?;
    easy.http_version(curl::easy::HttpVersion::V11)?;
    
    // Set up headers
    let mut headers = List::new();
    headers.append("Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")?;
    headers.append("User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.0.1 Safari/605.1.15")?;
    headers.append("Accept-Language: en-US,en;q=0.9")?;
    headers.append("Connection: keep-alive")?;
    easy.http_headers(headers)?;

    // Create a buffer to store the response body
    let mut response_body = Vec::new();
    
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            response_body.extend_from_slice(data);
            Ok(data.len())
        })?;
        
        transfer.perform()?;
    }
    
    // Check if the request was successful
    let status_code = easy.response_code()?;
    if status_code != 200 {
        return Err(format!("HTTP request failed with status code: {}", status_code).into());
    }

    // Convert response body to string
    Ok(String::from_utf8_lossy(&response_body).to_string())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <URL>", args[0]);
        std::process::exit(1);
    }
    
    let url_str = &args[1];
    let url = match Url::parse(url_str) {
        Ok(url) => url,
        Err(e) => {
            eprintln!("Error parsing URL: {}", e);
            std::process::exit(1);
        }
    };

    let converter = HtmlToMarkdown::builder()
        .skip_tags(vec!["script", "style"])
        .build();

    match fetch_url(url_str) {
        Ok(html_content) => {
            // Use the readability extractor with the fetched HTML content
            let mut reader = std::io::Cursor::new(html_content);
            match extractor::extract(&mut reader, &url) {
                Ok(product) => match converter.convert(&product.content) {
                    Ok(md) => {
                        println!("# {}\n\n{}", product.title, md);
                    }
                    Err(e) => eprintln!("Error converting to markdown: {}", e),
                },
                Err(e) => eprintln!("Error extracting content: {}", e),
            }
        }
        Err(e) => eprintln!("Error fetching URL: {}", e),
    }
}