use std::fs;

fn main() {
    // Check if the input file exists, and create it if it doesn't
    let input_path = "input.md";
    if !input_file_exists(input_path) {
        fs::write(input_path, "").unwrap();
    }

    // Read the input file
    let input = fs::read_to_string(input_path).unwrap();

    // Split the input into lines
    let lines = input.lines();

    // Iterate over the lines and parse the Markdown syntax
    let mut html_output = String::new();
    let mut in_code_block = false;
    let mut in_blockquote = false;
    let mut in_list = false;
    let mut in_list_item = false;
    let mut list_item_prefix = "";
    for line in lines {
        let parsed_line = parse_markdown(
            line,
            &mut in_code_block,
            &mut in_blockquote,
            &mut in_list,
            &mut in_list_item,
            &mut list_item_prefix.to_string(),
        );
        let html_line = generate_html(
            parsed_line,
            in_code_block,
            in_blockquote,
            in_list,
            in_list_item,
            &list_item_prefix,
        );
        html_output.push_str(&html_line);
    }

    // Write the HTML code to an output file
    fs::write("output.html", html_output).unwrap();
}

fn input_file_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn parse_markdown(
    line: &str,
    in_code_block: &mut bool,
    in_blockquote: &mut bool,
    in_list: &mut bool,
    in_list_item: &mut bool,
    list_item_prefix: &mut String,
) -> String {
    let mut parsed_line = line.to_string();
    if line.starts_with("```") {
        *in_code_block = !*in_code_block;
        if *in_code_block {
            parsed_line = "<pre><code>".to_string();
        } else {
            parsed_line = "</code></pre>".to_string();
        }
    } else if !*in_code_block {
        let bold_regex = regex::Regex::new(r"\*\*(.*?)\*\*").unwrap();
        let italic_regex = regex::Regex::new(r"_(.*?)_").unwrap();
        let strike_regex = regex::Regex::new(r"~~(.*?)~~").unwrap();
        let heading_regex = regex::Regex::new(r"^(#{1,6})\s(.*)$").unwrap();
        let link_regex = regex::Regex::new(r"\[(.*?)\]\((.*?)\)").unwrap();
        let image_regex = regex::Regex::new(r"!\[(.*?)\]\((.*?)\)").unwrap();
        let code_regex = regex::Regex::new(r"`(.*?)`").unwrap();
        let blockquote_regex = regex::Regex::new(r"^>\s(.*)$").unwrap();
        let list_regex = regex::Regex::new(r"^(\s*)[-*]\s(.*)$").unwrap();
        let ordered_list_regex = regex::Regex::new(r"^(\s*)\d+\.\s(.*)$").unwrap();
        if *in_blockquote {
            parsed_line = blockquote_regex
                .replace_all(&parsed_line, "<p>$1</p>")
                .to_string();
        } else if *in_list {
            if ordered_list_regex.is_match(&parsed_line) {
                parsed_line = ordered_list_regex
                    .replace_all(&parsed_line, |caps: &regex::Captures| {
                        format!("<li>{}</li>", &caps[2])
                    })
                    .to_string();
            } else if list_regex.is_match(&parsed_line) {
                parsed_line = list_regex
                    .replace_all(&parsed_line, |caps: &regex::Captures| {
                        format!("<li>{}</li>", &caps[2])
                    })
                    .to_string();
            } else {
                *in_list = false;
                *in_list_item = false;
                parsed_line = "".to_string();
            }
        }
        if !*in_blockquote && !*in_list {
            parsed_line = bold_regex
                .replace_all(&parsed_line, "<b>$1</b>")
                .to_string();
            parsed_line = italic_regex
                .replace_all(&parsed_line, "<i>$1</i>")
                .to_string();
            parsed_line = strike_regex
                .replace_all(&parsed_line, "<s>$1</s>")
                .to_string();
            parsed_line = heading_regex
                .replace_all(&parsed_line, |caps: &regex::Captures| {
                    let level = caps[1].len();
                    format!("<h{0}>{1}</h{0}>", level, &caps[2])
                })
                .to_string();
            parsed_line = link_regex
                .replace_all(&parsed_line, |caps: &regex::Captures| {
                    format!("<a href=\"{}\">{}</a>", &caps[2], &caps[1])
                })
                .to_string();
            parsed_line = image_regex
                .replace_all(&parsed_line, |caps: &regex::Captures| {
                    format!("<img src=\"{}\" alt=\"{}\">", &caps[2], &caps[1])
                })
                .to_string();
            parsed_line = code_regex
                .replace_all(&parsed_line, "<code>$1</code>")
                .to_string();
            if blockquote_regex.is_match(&parsed_line) {
                *in_blockquote = true;
                parsed_line = blockquote_regex
                    .replace_all(&parsed_line, "<blockquote><p>$1</p>")
                    .to_string();
            } else if list_regex.is_match(&parsed_line) {
                *in_list = true;
                *in_list_item = true;
                parsed_line = list_regex
                    .replace_all(&parsed_line, "<ul><li>$2</li>")
                    .to_string();
                *list_item_prefix = format!("{}<li>", &list_item_prefix);
            } else if ordered_list_regex.is_match(&parsed_line) {
                *in_list = true;
                *in_list_item = true;
                parsed_line = ordered_list_regex
                    .replace_all(&parsed_line, "<ol><li>$2</li>")
                    .to_string();
                *list_item_prefix = format!("{}<li>", &list_item_prefix);
            } else {
                parsed_line = format!("<p>{}</p>", parsed_line);
            }
        }
        if *in_list_item {
            if !list_regex.is_match(&parsed_line) && !ordered_list_regex.is_match(&parsed_line) {
                parsed_line = format!("{}{}", list_item_prefix, parsed_line);
            }
        }
    }
    parsed_line
}

fn generate_html(
    parsed_line: String,
    in_code_block: bool,
    in_blockquote: bool,
    in_list: bool,
    in_list_item: bool,
    list_item_prefix: &str,
) -> String {
    // TODO: Implement HTML generation logic
    // For example, you could simply wrap the parsed line in a paragraph tag
    // Here's an example implementation that wraps the parsed line in a paragraph tag:
    if in_code_block {
        format!("{}\n", parsed_line)
    } else if in_blockquote {
        if parsed_line == "" {
            "</blockquote>\n".to_string()
        } else {
            format!("{}\n", parsed_line)
        }
    } else if in_list {
        if in_list_item {
            format!("{}{}\n", list_item_prefix, parsed_line)
        } else {
            let new_prefix = format!("{}<li>", list_item_prefix);
            format!("{}{}\n", new_prefix, parsed_line)
        }
    } else {
        format!("{}\n", parsed_line)
    }
}
