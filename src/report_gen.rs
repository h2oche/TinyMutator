extern crate typed_html;
use std::{
    fs::{File, OpenOptions},
    convert::{TryFrom, TryInto},
};
use typed_html::{
    html,
    dom::DOMTree,
    types::{Class, SpacedSet},
};

pub fn make_report(path: &String) -> String {
    let mut doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>"Hello Kitty"</title>
            </head>
            <body>
                <h1>"Hello Kitty"</h1>
                <p class="official">
                    "She is not a cat. She is a human girl."
                </p>
                { (0..3).map(|_| html!(
                    <p class="emphasis">
                        "Her name is Kitty White."
                    </p>
                )) }
                <p class="citation-needed">
                    "We still don't know how she eats."
                </p>
            </body>
        </html>
    );
    return doc.to_string();
}