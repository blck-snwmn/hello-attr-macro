use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, token::Token, Error, Item};

#[proc_macro_attribute]
pub fn hello(args: TokenStream, input: TokenStream) -> TokenStream {
    println!("Hello! attribute macro");
    println!("\targs=`{}`", args.to_string());
    println!("\tinput=`{}`", input.to_string());

    let mut output = input.clone();

    let input = parse_macro_input!(input as Item);

    if let Err(e) = parse_item(&input) {
        output.extend(TokenStream::from(e))
    }

    output
}

fn parse_item(item: &Item) -> Result<(), TokenStream2> {
    match item {
        Item::Enum(item) => {
            println!("Enum!");
            Ok(())
        }

        Item::Const(_) => Err(Error::new(item.span(), "unexpected tokens").to_compile_error()),
        Item::ExternCrate(_) => {
            Err(Error::new(item.span(), "unexpected tokens").to_compile_error())
        }
        Item::Fn(_) => {
            println!("Fn!");
            Ok(())
        }
        Item::ForeignMod(_) => Err(Error::new(item.span(), "unexpected tokens").to_compile_error()),
        Item::Impl(_) => Err(Error::new(item.span(), "unexpected tokens").to_compile_error()),
        Item::Macro(_) => Err(Error::new(item.span(), "unexpected tokens").to_compile_error()),
        Item::Macro2(_) => Err(Error::new(item.span(), "unexpected tokens").to_compile_error()),
        Item::Mod(_) => Err(Error::new(item.span(), "unexpected tokens").to_compile_error()),
        Item::Static(_) => Err(Error::new(item.span(), "unexpected tokens").to_compile_error()),
        Item::Struct(_) => {
            println!("Struct!");
            Ok(())
        }
        Item::Trait(_) => Err(Error::new(item.span(), "unexpected tokens").to_compile_error()),
        Item::TraitAlias(_) => Err(Error::new(item.span(), "unexpected tokens").to_compile_error()),
        Item::Type(_) => Err(Error::new(item.span(), "unexpected tokens").to_compile_error()),
        Item::Union(_) => Err(Error::new(item.span(), "unexpected tokens").to_compile_error()),
        Item::Use(_) => Err(Error::new(item.span(), "unexpected tokens").to_compile_error()),
        Item::Verbatim(_) => Err(Error::new(item.span(), "unexpected tokens").to_compile_error()),
        Item::__TestExhaustive(_) => {
            Err(Error::new(item.span(), "unexpected tokens").to_compile_error())
        }
    }
}
