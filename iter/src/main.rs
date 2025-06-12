fn print_elements(elements: &[String]) {
    elements
        .iter()
        .map(|el| format!("{} {}", el, el))
        .for_each(|el| println!("{}", el));
}

fn shorten_strings(elements: &mut [String]) {
    elements
        .iter_mut()
        .for_each(|el| el.truncate(1));
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements
        .iter()
        .map(|el| el.to_uppercase())
        .collect::<Vec<String>>()
}

fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|el| vec_b.push(el));
}

fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
        .iter()
        .map(|el| el.chars().map(|c| c.to_string()).collect())
        .collect::<Vec<Vec<String>>>()
}

fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String {
    elements
        .iter()
        .find(|el| el.contains(search))
        .map_or(String::from(fallback), |el| el.to_string())
}

fn main() {
    let colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue")
    ];

    let uppercase_colors = to_uppercase(&colors);
    print_elements(&uppercase_colors);

    let mut destination = vec![];
    move_elements(uppercase_colors, &mut destination);
    shorten_strings(&mut destination);
    print_elements(&destination);

    let exploded = explode(&colors);
    println!("{:#?}", exploded);

    let found_color = find_color_or(&colors, "re", "NOPE");
    println!("{}", found_color)
}
