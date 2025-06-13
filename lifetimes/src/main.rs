fn next_language<'a>(languages: &'a[String], current: &str) -> &'a str {
    let mut found = false;
    for language in languages {
        if found == true {
            return language;
        }
        if language == current {
            found = true;
        }
    }
    languages.last().unwrap()
}

fn last_language(languages: &[String]) -> &str {
    languages.last().unwrap()
}

fn longest_language<'a>(lang_a: &'a str, lang_b: &'a str) -> &'a str {
    if lang_a.len() >= lang_b.len() {
        lang_a
    } else {
        lang_b
    }
 }

fn main() {
    let languages = vec![
        String::from("rust"),
        String::from("go"),
        String::from("typescript")
    ];

    let result = next_language(&languages, "go");
    let last_language = last_language(&languages);
    let longest_language = longest_language(&languages[1], &languages[0]);

    println!("{:#?}", result);
    println!("{}", last_language);
    println!("{}", longest_language);

}
