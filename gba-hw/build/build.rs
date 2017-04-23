use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

extern crate syn;
extern crate gba_register_generator;

#[macro_use]
extern crate quote;

mod video;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("gen.rs");
    let mut f = File::create(&dest_path).unwrap();

    let video_tokens = video::tokens();

    f.write_all(format!(
        "{}",
        quote! {
            mod gen {
                pub mod video {
                    #video_tokens
                }
            }
        }).as_bytes()).unwrap();
}
