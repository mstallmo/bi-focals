#[macro_use]
extern crate html5ever;

use html5ever::driver::ParseOpts;
use html5ever::rcdom::{Handle, NodeData, RcDom};
use html5ever::tendril::TendrilSink;
use html5ever::{parse_fragment, serialize, QualName};
use std::str;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn walk(indent: usize, handle: &mut Handle) {
    let node = handle;
    // FIXME: don't allocate
    // print!("{}", repeat(" ").take(indent).collect::<String>());
    match node.data {
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            assert!(name.ns == ns!(html));
            if name.local == local_name!("img") {
                log(format!("<{}", name.local).as_str());
                for attr in attrs.borrow().iter() {
                    assert!(attr.name.ns == ns!());
                    log(format!(" {}=\"{}\"", attr.name.local, attr.value).as_str());
                }
                log(">");
            }
        }
        _ => (),
    }

    for child in node.children.borrow().iter() {
        walk(indent + 4, &mut child.clone());
    }
}

#[wasm_bindgen]
pub fn parse_html(input: &str) -> String {
    let mut fragment = parse_fragment(
        RcDom::default(),
        ParseOpts::default(),
        QualName::new(None, ns!(html), local_name!("body")),
        vec![],
    )
    .from_utf8()
    .read_from(&mut input.as_bytes())
    .unwrap();

    walk(0, &mut fragment.document);

    if !fragment.errors.is_empty() {
        log("\nParse errors:");
        for err in fragment.errors.into_iter() {
            log(format!("   {}", err).as_str());
        }
    } else {
        log("parsed html!");
    }

    let mut serialized_output = vec![];
    serialize(
        &mut serialized_output,
        &fragment.document.children.borrow()[0],
        Default::default(),
    )
    .ok()
    .expect("serialization failed");

    str::from_utf8(&serialized_output).unwrap().to_owned()
    // String::from("TEST")
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
