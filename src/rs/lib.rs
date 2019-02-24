use scraper::{Html, Selector};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn parse_html(input: &str) -> String {
    let fragment = Html::parse_fragment(input);

    let img_selector = Selector::parse("img").unwrap();
    // for element in fragment.select(&img_selector) {
    //     log(element.value().name());
    //     log(element.value().attr("src").unwrap());
    // }
    let element = fragment.select(&img_selector).next().unwrap();

    String::from(element.value().attr("src").unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_parse_a_fragment() {
        let data = "<a href=\"foo\"></a>";
        parse_html(data);
    }
}
