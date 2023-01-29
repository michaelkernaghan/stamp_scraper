use regex::Regex;

fn main() {
    let response = reqwest::blocking
        ::get("http://www.fvhstamps.com/WeeklyAuctions/FvhWA.htm")
        .unwrap()
        .text()
        .unwrap();

    let document = scraper::Html::parse_document(&response);
    let title_selector = scraper::Selector::parse("span").unwrap();

    let titles = document.select(&title_selector).map(|x| x.inner_html());

    let re = Regex::new(r">(.*?)<").unwrap();

    titles.for_each(|item| {
        if
            item.contains("jpg") &&
            !item.contains("CATALOGUE") &&
            !item.contains("Catalogue") &&
            !item.contains("BLOCK") &&
            !item.contains("Block") &&
            !item.contains("CANADA") &&
            !item.contains("Booklets") &&
            !item.contains("BOOKLETS") &&
            !item.contains("SPECIMEN") &&
            !item.contains("cover") &&
            !item.contains("Souvenir") &&
            !item.contains("CINDERELLAS") &&
            !item.contains("BANK NOTE") &&
            !item.contains("POSTCARDS") &&
            !item.contains("P.O.W. Mail") &&
            !item.contains("COVERS") &&
            !item.contains("JERSEY") &&
            !item.contains("Accum") &&
            !item.contains("Omnibus complete") &&
            !item.contains("BERLIN") &&
            !item.contains("LAW STAMPS") &&
            !item.contains("BRITISH COLUMBIA") &&
            !item.contains("ONTARIO") &&
            !item.contains("SAUDI ARABIA") &&
            !item.contains("ALBERTA") &&
            !item.contains("SASKATCHEWAN") &&
            !item.contains("ESSAYS") &&
            !item.contains("BANKNOTE") &&
            !item.contains(" Range of") &&
            !item.contains("POST  CARDS") &&
            !item.contains("Cover") &&
            !item.contains("Forgery") &&
            !item.contains("Fake") &&
            !item.contains("Law Stamp") &&
            !item.contains("SAUDI ARABIA") &&
            !item.contains("EAST GERMANY") &&
            !item.contains("GDR")
        {
            let item = item.replace("\n", "");
            let item = item.replace(",", "");
            for cap in re.captures_iter(&item) {
                let interesting = &cap[1];

                if
                    Regex::new(
                        r"\s18\d{2}.|\s19[0-3]\d\s.|\s19[0-3]\d-[0-3]\d.|19\d{2}-40.|1940\s."
                    )
                        .unwrap()
                        .is_match(interesting)
                {
                    println!("{}", interesting);
                }
            }
        }
    });
}