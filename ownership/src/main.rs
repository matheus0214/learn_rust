fn main() {
    let mut s = String::from("hello");

    let len = calculate_length(&s);

    println!("{s} {len}");

    change(&mut s);

    println!("{s}");

    let mut v = String::from("this is something to test");

    let w = first_word(&v);

    println!("{w}");

    let w = first_word(&v[..6]);

    println!("{w}");

    println!("{}", first_word("Literal string"));

    v.clear();

    let a = [1, 2, 3, 4, 5];

    let sl = &a[..2];

    assert_eq!(sl, &[1, 2])
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("value");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
