use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::error::Error;

#[derive(Debug)]
pub struct Submission {
    pub title: String,
    pub link: String,
}
// Struct to hold page data
pub struct Page {
    pub submissions: Vec<Submission>,
    pub current_page: usize,
    pub total_pages: usize,
}

pub fn fetch_hacker_news(page: usize) -> Result<Page, Box<dyn Error>> {
    // Construct the URL with pagination
    let url = format!("https://news.ycombinator.com/news?p={}", page);
    let response = get(&url)?.text()?;
    let document = Html::parse_document(&response);

    let row_selector = Selector::parse("tr.athing")?;
    let mut submissions = Vec::new();

    for row in document.select(&row_selector) {
        // Select the title link
        let title_selector = Selector::parse("td.title > span.titleline > a")?;
        if let Some(title_element) = row.select(&title_selector).next() {
            let title = title_element.inner_html();
            let link = title_element.value().attr("href").unwrap_or("").to_string();
            submissions.push(Submission { title, link });
        }
    }

    // Here you would determine the total number of pages based on your logic
    // For example, you could set a fixed number of total pages
    let total_pages = 10; // Placeholder for total pages; implement logic to determine this if needed

    Ok(Page {
        submissions,
        current_page: page,
        total_pages,
    })
}
