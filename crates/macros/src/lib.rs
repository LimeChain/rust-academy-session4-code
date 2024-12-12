#![allow(unused)]

use proc_macro::{Group, Ident, Literal, Punct, Spacing, TokenStream, TokenTree};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// 2. Procedural Macros

// 2.1 Function-like Macro
#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

// 2.2 Derive Macro
#[proc_macro_derive(Neshto, attributes(banica))]
pub fn derive_neshto(input: TokenStream) -> TokenStream {
    // Парсване на входящия TokenStream
    let input = parse_macro_input!(input as DeriveInput);

    // Извличане на името на структурата
    let name = &input.ident;

    // Генериране на код с quote!
    let expanded = quote! {
        impl #name {
            fn special_method() -> String {
                format!("Специален метод за {}", stringify!(#name))
            }
        }
    };

    // Конвертиране към TokenStream
    TokenStream::from(expanded)
}

// 2.3 Attribute-like Macro
#[proc_macro_attribute]
pub fn log_method(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Простичък attribute macro който добавя логване към метод
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let name = &input.sig.ident;
    let body = &input.block;

    let expanded = quote! {
        fn #name() {
            println!("Извикване на метод: {}", stringify!(#name));
            #body
        }
    };

    TokenStream::from(expanded)
}

// Бонус: Ръчно създаване на прост macro processor
#[proc_macro]
pub fn bulgar(input: TokenStream) -> TokenStream {
    let mut new_tokens = Vec::new();

    for token in input {
        let span = token.span();
        match token {
            TokenTree::Ident(ident) if ident.to_string() == "hello" => {
                // Заменя "hello" с "привет"
                new_tokens.push(TokenTree::Ident(Ident::new("привет", span)));
            }
            _ => new_tokens.push(token),
        }
    }

    TokenStream::from_iter(new_tokens)
}
