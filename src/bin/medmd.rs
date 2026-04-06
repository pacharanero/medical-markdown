use std::io::Read;

fn main() {
    let mut input = String::new();

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] != "-" {
        input = std::fs::read_to_string(&args[1]).unwrap_or_else(|e| {
            eprintln!("Error reading {}: {e}", args[1]);
            std::process::exit(1);
        });
    } else {
        std::io::stdin().read_to_string(&mut input).unwrap_or_else(|e| {
            eprintln!("Error reading stdin: {e}");
            std::process::exit(1);
        });
    }

    let md = &mut markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(md);
    medical_markdown::add(md);

    let ast = md.parse(&input);

    let output_json = args.iter().any(|a| a == "--json");
    let output_html = args.iter().any(|a| a == "--html");
    let output_both = !output_json && !output_html;

    if output_html || output_both {
        println!("{}", ast.render());
    }

    if output_json || output_both {
        let data = medical_markdown::extract_structured_data(&ast);
        println!("{}", serde_json::to_string_pretty(&data).unwrap());
    }
}
