use wasm_bindgen::prelude::*;
use web_sys::Document;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn main() {
    let doc = Document::new().unwrap();
    let img_elements = doc.get_elements_by_tag_name("img");

    let element = img_elements.item(0).unwrap();

    log(&element.get_attribute("src").unwrap());

    // Document::get_elements_by_tag_name("img");
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_should_parse_a_fragment() {
//         let data = "<a href=\"foo\"></a>";
//         parse_html(data);
//     }
// }
