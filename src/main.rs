use regex::Regex;

fn main() {
     let response = reqwest::blocking::get(
         "http://www.fvhstamps.com/WeeklyAuctions/FvhWA.htm",
     )
     .unwrap()
     .text()
     .unwrap();

     let document = scraper::Html::parse_document(&response);
     let title_selector = scraper::Selector::parse("span").unwrap();

     let titles = document.select(&title_selector).map(|x| x.inner_html());

    let re = Regex::new(r">(.*?)<").unwrap();

     titles
         .for_each(|item| {
             if item.contains("jpg") 
                && !item.contains("CATALOGUE")
                && !item.contains("Catalogue")
                && !item.contains("BLOCK")
                && !item.contains("Block")
                && !item.contains("UNITED STATES")
                && !item.contains("CANADA")
                && !item.contains("Booklets")
                && !item.contains("BOOKLETS")
                && !item.contains("SPECIMEN")
                && !item.contains("cover")
                && !item.contains("Souvenir")
                
             {
                let item = item.replace("\n", "");
                 for cap in re.captures_iter(&item) {
                     let catch = &cap[1];
                     println!("{}", catch);
                     println!("");
                 }
                 
             }
         }
     );
}