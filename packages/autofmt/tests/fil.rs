#[test]
fn formats_file_properly() {
    let src = include_str!("./samples/thing.rsx");

    let formatted = dioxus_autofmt::fmt_file(src);
    let out = dioxus_autofmt::apply_formats(src, formatted);

    println!("{}", out);
}

#[test]
fn already_formatted_file_properly() {
    let src = include_str!("./samples/pre.rsx");

    let formatted = dioxus_autofmt::fmt_file(src);

    dbg!(&formatted);

    let out = dioxus_autofmt::apply_formats(src, formatted);

    println!("{}", out);
}

#[test]
fn comment_case_work() {
    let src = include_str!("./samples/comments.rsx");

    let formatted = dioxus_autofmt::fmt_file(src);
    let out = dioxus_autofmt::apply_formats(src, formatted);

    println!("{}", out);
}

#[test]
fn comment_attr_case_work() {
    let src = include_str!("./samples/attrs.rsx");

    let formatted = dioxus_autofmt::fmt_file(src);
    let out = dioxus_autofmt::apply_formats(src, formatted);

    println!("{}", out);
}