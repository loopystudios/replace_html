fn main() {
    let html = r#"
    <style>
        p {
            margin: 25px
        }
    </style>
    <p>
        This was injected by Rust!
    </p>
    "#;

    if let Err(e) = replace_html::replace_body(html) {
        eprintln!("Failed to replace HTML: {e}");
    };
}
