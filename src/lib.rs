use proc_macro::TokenStream;

#[proc_macro]
pub fn jstr(item: TokenStream) -> TokenStream {
    let mut string = String::from("[");
    string.push_str(
        &item
            .to_string()
            .trim_start_matches("r#")
            .trim_end_matches("#"),
    );
    string.push_str("].join(\"\")");
    string
        .replace("{", "\", ")
        .replace("}", ", \"")
        .replace("\"\", ", "")
        .parse()
        .unwrap()
}
