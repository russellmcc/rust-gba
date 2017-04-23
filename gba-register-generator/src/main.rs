#[macro_use]
extern crate quote;

#[macro_use]
extern crate derive_builder;

#[macro_use]
extern crate error_chain;

extern crate syn;

mod errors {
    error_chain!{}
}
use errors::*;

mod register;
mod generate;

use syn::Ident;

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let items = generate::generate_register(
        &(register::RegisterBuilder::default()
          .title(Ident::new("TestRegister"))
          .doc("A register for testing")
          .repr(Ident::new("u16"))
          .fields(vec![
              register::FieldBuilder::default()
                  .name(Ident::new("int_field"))
                  .doc("A field that represents an integer.")
                  .start(0usize)
                  .size(3usize)
                  .format(register::Format::Type{
                      type_ident: Ident::new("u8"),
                      as_bits: None,
                      from_bits: None,
                  })
                  .access(register::Access::ReadWrite)
                  .build()?,
              register::FieldBuilder::default()
                  .name(Ident::new("enum_field"))
                  .doc("A field that is an enum")
                  .start(3usize)
                  .size(3usize)
                  .access(register::Access::ReadWrite)
                  .format(register::Format::Enum{
                      title: Ident::new("TestEnum"),
                      variants: vec![
                          register::VariantBuilder::default()
                              .title("TestOne")
                              .doc("Foo Is a really cool bar.")
                              .quick_get(Some(Ident::new("test_one")))
                              .quick_set(Some(Ident::new("set_test_one")))
                              .build()?,
                          register::VariantBuilder::default()
                              .title("TestTwo")
                              .doc("BORING")
                              .quick_get(Some(Ident::new("test_two")))
                              .quick_set(Some(Ident::new("set_test_two")))
                              .value(Some(7))
                              .build()?
                      ]
                  })
                  .build()?,
              register::FieldBuilder::default()
                  .name(Ident::new("bool_is_enabled"))
                  .doc("A basic field that is an enum")
                  .start(6usize)
                  .size(1usize)
                  .access(register::Access::ReadWrite)
                  .format(register::Format::Bool {
                      quick_set_true: Some(Ident::new("set_bool_enabled")),
                      quick_set_false: Some(Ident::new("set_bool_disabled")),
                  })
                  .build()?
          ])
          .build()?)
    ).chain_err(|| {
        "Could not generate test enum"
    })?;
    println!("{}",
             quote! {
                 #items
             });
    Ok(())
}
