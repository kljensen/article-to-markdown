extern crate readability;
use htmd::HtmlToMarkdown;
use readability::extractor;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <URL>", args[0]);
        std::process::exit(1);
    }
    let url = &args[1];
    let converter = HtmlToMarkdown::builder()
        .skip_tags(vec!["script", "style"])
        .build();

    match extractor::scrape(url) {
        Ok(product) => match converter.convert(&product.content) {
            Ok(md) => {
                println!("# {}\n\n{}", product.title, md);
            }
            Err(_) => println!("error occured"),
        },
        Err(_) => println!("error occured"),
    }
}
