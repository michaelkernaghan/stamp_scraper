use regex::Regex;

fn main() {
    let response = reqwest::blocking::get("http://www.fvhstamps.com/WeeklyAuctions/FvhWA.htm")
        .unwrap()
        .text()
        .unwrap();

    let document = scraper::Html::parse_document(&response);
    let title_selector = scraper::Selector::parse("span").unwrap();

    let titles = document.select(&title_selector).map(|x| x.inner_html());

    let re = Regex::new(r">(.*?)<").unwrap();

    titles.for_each(|item| {
        if item.contains("jpg")
            && !item.contains("CATALOGUE")
            && !item.contains("Catalogue")
            && !item.contains("BLOCK")
            && !item.contains("Block")
            && !item.contains("CANADA")
            && !item.contains("Booklets")
            && !item.contains("BOOKLETS")
            && !item.contains("SPECIMEN")
            && !item.contains("cover")
            && !item.contains("Souvenir")
        {
            let item = item.replace("\n", "");
            for cap in re.captures_iter(&item) {
                let interesting = &cap[1];

                if Regex::new(r"\s18\d{2}.|\s190\d.|\s191\d.|\s192\d.|\s193\d.|\s194\d.")
                    .unwrap()
                    .is_match(interesting)
                {
                    println!("{}", interesting);
                }
            }
        }
    });
}
