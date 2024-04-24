static CODE_FRAGMENT: &'static str = r#"// This is the main function.
fn main() {
    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    println!("Hello World!");
}"#;


fn main() {
    let total_num_of_lines = CODE_FRAGMENT.lines().count();
    let lines_containing = CODE_FRAGMENT.lines().filter(|lines| !lines.trim().is_empty() && !lines.trim().starts_with("\n")).count();
    let empty_line = CODE_FRAGMENT.lines().filter(|line| line.trim().is_empty()).count();

    println!("Lines in total: {}", total_num_of_lines);
    println!("Lines containing code: {}", lines_containing);
    println!("Empty lines: {}", empty_line);
}
