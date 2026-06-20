use std::io::Read;
use std::path::Path;

fn main() {
    let mut input = String::new();

    let args: Vec<String> = std::env::args().collect();

    // Find the input file (first positional arg that isn't a flag)
    let input_arg = args.iter().skip(1).find(|a| !a.starts_with("--"));

    match input_arg {
        Some(path) if path != "-" => {
            input = std::fs::read_to_string(path).unwrap_or_else(|e| {
                eprintln!("Error reading {path}: {e}");
                std::process::exit(1);
            });
        }
        _ => {
            std::io::stdin()
                .read_to_string(&mut input)
                .unwrap_or_else(|e| {
                    eprintln!("Error reading stdin: {e}");
                    std::process::exit(1);
                });
        }
    }

    // Check for --codes <path.json> flag
    let codes_path = args
        .windows(2)
        .find(|w| w[0] == "--codes")
        .map(|w| w[1].clone());

    let md = &mut markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(md);

    if let Some(path) = codes_path {
        let mut registry = medical_markdown::CodeRegistry::default();
        let custom =
            medical_markdown::CodeRegistry::from_json(Path::new(&path)).unwrap_or_else(|e| {
                eprintln!("Error loading codes from {path}: {e}");
                std::process::exit(1);
            });
        registry.merge(&custom);
        medical_markdown::add_with_registry(md, registry);
    } else {
        medical_markdown::add(md);
    }

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
