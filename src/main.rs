#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
struct Count {
    lines_in_total: usize,
    lines_containing_code: usize,
    empty_lines: usize,
    code_symbols: usize,
}

impl Count {

    #[allow(dead_code)]
    pub fn calculate(text: &str) -> Count {
        let mut count = Count {
            lines_in_total: 0,
            lines_containing_code: 0,
            empty_lines: 0,
            code_symbols: 0,
        };

        for line in text.lines() {
            count.lines_in_total += 1;


            if line.trim().is_empty() {
                count.empty_lines += 1;

                if line.ends_with("\n"){
                    count.code_symbols -= 1;
                }
            } else {
                count.lines_containing_code += 1;

                // include the whitespace and line breaker
                count.code_symbols += line.chars().count();

                // exclude the the line breacker /n
                if line.ends_with("\n"){
                    count.code_symbols -= 1;
                }
                 
            }
        }

        count.code_symbols += count.lines_in_total - 1; 
        count
    }
}

fn main() {
   
    let text = r#" 
    candidate : hello mam i have submited my assignment
 
    hr : ok we will get back to you 
 
    candidate : still waiting for the good reseponce 
    candidate : i hope i will get the chance to show my skills 
    "#;

   println!("{}",text.chars().count());

    let result = Count::calculate(text);
    println!("Lines in total: {}", result.lines_in_total);
    println!("Lines containing code: {}", result.lines_containing_code);
    println!("Empty lines: {}", result.empty_lines);
    println!("Number of characters of code: {}", result.code_symbols);
}



