use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::error::Error;

#[derive(Debug)]
pub struct Submission {
    pub title: String,
    pub author: String,
    pub link: String,
}

pub fn fetch_hacker_news() -> Result<Vec<Submission>, Box<dyn Error>> {
    let url = "https://news.ycombinator.com/";
    let response = get(url)?.text()?;
    let document = Html::parse_document(&response);

    let row_selector = Selector::parse("tr.athing")?;
    let mut submissions = Vec::new();

    for row in document.select(&row_selector) {
        // Select the title link
        let title_selector = Selector::parse("td.title > span.titleline > a")?;
        if let Some(title_element) = row.select(&title_selector).next() {
            let title = title_element.inner_html();
            let link = title_element.value().attr("href").unwrap_or("").to_string();
            // Get the next sibling row for the author
            // if let Some(subtext_row) = row.next_sibling() {
            //     // Ensure the sibling is a <tr> element
            //     if subtext_row.is_element() {
            //         let subtext_selector = Selector::parse("td.subtext > a.hnuser")?;
            //         // Use the `select` method on the subtext row
            //         let author_element = subtext_row.select(&subtext_selector).next();
            //         let author = author_element.map_or("No Author".to_string(), |e| e.inner_html());

            //         submissions.push(Submission {
            //             title,
            //             author,
            //             link,
            //         });
            //     }
            // }
            submissions.push(Submission {
                title,
                author: "".to_string(),
                link,
            });
        }
    }

    Ok(submissions)
}
