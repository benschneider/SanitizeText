use phf::phf_map;
use std::fs;
use std::path::Path;

pub static REPMAP: phf::Map<char, (char, &'static str)> = phf_map! {
    '\u{00A0}' => (' ', "Non-breaking space – used to prevent line breaks."),
    '\u{00AD}' => ('-', "Soft hyphen – optional hyphenation point."),
    '\u{034F}' => (' ', "Combining grapheme joiner – invisible formatting."),
    '\u{061C}' => (' ', "Arabic letter mark – bidirectional formatting."),
    '\u{1680}' => (' ', "⚠️ Ogham space mark – rare legacy space."),
    '\u{180E}' => (' ', "Mongolian vowel separator – deprecated formatting."),
    '\u{2000}' => (' ', "En quad – fixed-width space."),
    '\u{2001}' => (' ', "Em quad – fixed-width space."),
    '\u{2002}' => (' ', "En space – typographic space."),
    '\u{2003}' => (' ', "Em space – typographic space."),
    '\u{2004}' => (' ', "Three-per-em space – typographic space."),
    '\u{2005}' => (' ', "Four-per-em space – typographic space."),
    '\u{2006}' => (' ', "Six-per-em space – typographic space."),
    '\u{2007}' => (' ', "Figure space – numeric alignment."),
    '\u{2008}' => (' ', "Punctuation space – typographic space."),
    '\u{2009}' => (' ', "Thin space – narrow typographic space."),
    '\u{200A}' => (' ', "Hair space – extremely narrow space."),
    '\u{200B}' => (' ', "Zero-width space – invisible separator."),
    '\u{200C}' => (' ', "Zero-width non-joiner – formatting control."),
    '\u{200D}' => (' ', "Zero-width joiner – formatting control."),
    '\u{200E}' => (' ', "⚠️ Left-to-right mark – invisible directional formatting."),
    '\u{200F}' => (' ', "⚠️ Right-to-left mark – invisible directional formatting."),
    '\u{2028}' => (' ', "⚠️ Line separator – unusual newline character."),
    '\u{2029}' => (' ', "⚠️ Paragraph separator – unusual newline character."),
    '\u{202A}' => (' ', "Left-to-right embedding – bidi formatting."),
    '\u{202B}' => (' ', "Right-to-left embedding – bidi formatting."),
    '\u{202C}' => (' ', "Pop directional formatting – bidi control."),
    '\u{202D}' => (' ', "Left-to-right override – bidi control."),
    '\u{202E}' => (' ', "Right-to-left override – bidi control."),
    '\u{202F}' => (' ', "Narrow no-break space – narrow fixed space."),
    '\u{2039}' => ('<', "Left-pointing angle quote – often used in French."),
    '\u{203A}' => ('>', "Right-pointing angle quote – often used in French."),
    '\u{2043}' => ('-', "Hyphen bullet – list item symbol."),
    '\u{205F}' => (' ', "Medium mathematical space – math typography."),
    '\u{2060}' => (' ', "Word joiner – prevents line breaks."),
    '\u{2061}' => (' ', "Function application – invisible formatting."),
    '\u{2062}' => (' ', "Invisible times – math formatting."),
    '\u{2063}' => (' ', "Invisible separator – math formatting."),
    '\u{2064}' => (' ', "Invisible plus – math formatting."),
    '\u{2066}' => (' ', "Left-to-right isolate – bidi formatting."),
    '\u{2067}' => (' ', "Right-to-left isolate – bidi formatting."),
    '\u{2068}' => (' ', "First strong isolate – bidi formatting."),
    '\u{2069}' => (' ', "Pop directional isolate – bidi formatting."),
    '\u{2010}' => ('-', "Hyphen – standard dash."),
    '\u{2011}' => ('-', "Non-breaking hyphen – no line break."),
    '\u{2014}' => ('-', "Em dash – punctuation."),
    '\u{2018}' => ('\'', "Left single quote – punctuation."),
    '\u{2019}' => ('\'', "Right single quote – punctuation."),
    '\u{201C}' => ('\"', "Left double quote – punctuation."),
    '\u{201D}' => ('\"', "Right double quote – punctuation."),
    '\u{2212}' => ('-', "Minus sign – math symbol."),
    '\u{FEFF}' => (' ', "Byte order mark – invisible character."),
    '\u{2E3F}' => ('-', "Three-em dash – very long dash used in text."),
    '\u{2E3A}' => ('-', "Two-em dash – used for extended punctuation."),
    '\u{2012}' => ('-', "Figure dash – similar to hyphen."),
    '\u{2013}' => ('-', "En dash – typically used for ranges."),
    '\u{17B4}' => (' ', "Khmer vowel inherent AQ – invisible character."),
    '\u{17B5}' => (' ', "Khmer vowel inherent AA – invisible character."),
    '\u{FE63}' => ('-', "Small hyphen-minus – punctuation variant."),
    '\u{FF0D}' => ('-', "Fullwidth hyphen-minus – fullwidth form of '-'."),
    '\u{2170}' => ('i', "Small Roman numeral one – can resemble 'i'."),
    '\u{A7AE}' => ('I', "Latin capital letter I with stroke – resembles 'I'."),
    '\u{2113}' => ('l', "Script small l – resembles lowercase 'l'."),
    '\u{206A}' => (' ', "Inhibit symmetric swapping – deprecated formatting."),
    '\u{206B}' => (' ', "Activate symmetric swapping – deprecated formatting."),
    '\u{206C}' => (' ', "Inhibit Arabic form shaping – deprecated formatting."),
    '\u{206D}' => (' ', "Activate Arabic form shaping – deprecated formatting."),
    '\u{206E}' => (' ', "National digit shapes – deprecated formatting."),
    '\u{206F}' => (' ', "Nominal digit shapes – deprecated formatting."),
    '\u{0430}' => ('a', "⚠️ Cyrillic small a – visually identical to Latin 'a'. Common in phishing."),
    '\u{0456}' => ('i', "⚠️ Cyrillic small Ukrainian i – looks like Latin 'i'."),
    '\u{0406}' => ('I', "⚠️ Cyrillic capital Ukrainian I – looks like Latin 'I'."),
    '\u{0441}' => ('c', "⚠️ Cyrillic small es – resembles Latin 'c'."),
    '\u{0445}' => ('x', "⚠️ Cyrillic small ha – resembles Latin 'x'."),
    '\u{3000}' => (' ', "⚠️ Ideographic space – large invisible character used in East Asian text."),
    '\u{FFFC}' => (' ', "⚠️ Object replacement character – invisible placeholder."),
    //'\u{0009}' => ('    ', "⚠️ Tab character – replaced with 4 spaces. Consider making this configurable."),
    '\u{000B}' => (' ', "⚠️ Vertical tab – legacy formatting."),
    '\u{000C}' => (' ', "⚠️ Form feed – legacy formatting."),
    '\u{001C}' => (' ', "⚠️ File separator – non-printable control."),
    '\u{001D}' => (' ', "⚠️ Group separator – non-printable control."),
    '\u{001E}' => (' ', "⚠️ Record separator – non-printable control."),
    '\u{001F}' => (' ', "⚠️ Unit separator – non-printable control."),
    '\u{007F}' => (' ', "⚠️ Delete character – may be used to obscure content."),
    // Stylized text characters flagged with soft 🧠 warnings (do not change the character)
    '\u{1D4C1}' => ('𝓁', "🧠 Mathematical script small l – may resemble normal 'l'."),
    '\u{1D540}' => ('𝕀', "🧠 Double-struck capital I – may be confused with normal 'I'."),
    '\u{1D400}' => ('𝐀', "🧠 Bold capital A – stylized character."),
    '\u{1D41A}' => ('𝐚', "🧠 Bold small a – stylized character."),
    '\u{1D4B6}' => ('𝒶', "🧠 Script small a – stylized character."),
    '\u{1D5BA}' => ('𝗮', "🧠 Sans-serif bold small a – stylized character."),
    '\u{1D622}' => ('𝘢', "🧠 Sans-serif italic small a – stylized character."),
    '\u{1D68A}' => ('𝚊', "🧠 Monospace small a – stylized character."),
};

pub fn clean_invisible(input: &str) -> (String, usize, Vec<String>) {
    use std::collections::HashMap;

    let (cleaned, count, counts) = input.chars().fold(
        (String::new(), 0usize, HashMap::<&'static str, usize>::new()),
        |(mut acc, mut count, mut map), c| {
            match c {
                // First check if it's in the REPMAP
                c if REPMAP.contains_key(&c) => {
                    let (rep, desc) = REPMAP[&c];
                    acc.push(rep);
                    count += 1;
                    *map.entry(desc).or_insert(0) += 1;
                }
                
                c if ('\u{E0000}'..='\u{E0FFF}').contains(&c) => {
                    let code_point = c as u32 - 0xE0000;
                    if let Some(mapped_c) = char::from_u32(code_point) {
                        acc.push(mapped_c);
                    } else {
                        acc.push(' ');
                    }
                    count += 1;
                    *map.entry("Private Use Area (E0000-E0FFF) – non-standard Invisible characters").or_insert(0) += 1;
                }
                _ => acc.push(c),
            }
            (acc, count, map)
        },
    );

    // Group by emoji prefix (icon)
    let mut grouped: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    for (&desc, n) in &counts {
        // Extract emoji or icon prefix (icon) carefully
        let icon = {
            let mut chars = desc.chars();
            let first = chars.next();
            let second = chars.next();
            if let (Some(f), Some(s)) = (first, second) {
                let mut prefix = String::new();
                prefix.push(f);
                if !s.is_alphanumeric() && !s.is_ascii() {
                    prefix.push(s);
                } else if f == '🧠' || f == '⚠' || f == '🚫' || f == '❌' {
                    prefix.push(s);
                }
                prefix
            } else if let Some(f) = first {
                f.to_string()
            } else {
                "".to_string()
            }
        };
        grouped.entry(icon).or_default().push((desc.to_string(), *n));
    }

    let mut found = Vec::new();
    let mut keys: Vec<_> = grouped.keys().cloned().collect();
    keys.sort();
    for key in keys {
        if let Some(items) = grouped.get(&key) {
            found.push(format!("{} Group:", key));
            let mut sorted_items = items.clone();
            sorted_items.sort();
            for (desc, n) in sorted_items {
                found.push(format!("  • {} (×{})", desc, n));
            }
        }
    }

    (cleaned, count, found)
}

pub fn process_file(input_path: &str, output_path: Option<&str>) {
    let input_text = fs::read_to_string(input_path).expect("Failed to read input file");
    let (cleaned, count, found) = clean_invisible(&input_text);

    if let Some(path) = output_path {
        fs::write(path, &cleaned).expect("Failed to write to output file");
        println!("✅ Cleaned output written to {}", path);
    } else {
        println!("Cleaned text:\n{}", cleaned);
    }

    eprintln!("\nRemoved {} invisible characters.", count);
    for desc in found {
        eprintln!("• {}", desc);
    }
}

pub fn process_directory(input_path: &str, output_dir: Option<&str>) {
    let in_dir = Path::new(input_path);
    let out_dir = output_dir
        .map(Path::new)
        .map(Path::to_path_buf)
        .unwrap_or_else(|| {
            in_dir.with_file_name(format!(
                "{}{}",
                in_dir.file_name().unwrap().to_string_lossy(),
                "_cleaned"
            ))
        });
    fs::create_dir_all(&out_dir).expect("Failed to create output directory");

    for entry in fs::read_dir(in_dir).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.is_file() {
            let content =
                fs::read_to_string(&path).expect("Failed to read file");
            let (cleaned, count, _) = clean_invisible(&content);
            let file_name = path.file_name().unwrap().to_string_lossy();
            let out_file = out_dir.join(&*file_name);
            fs::write(&out_file, &cleaned).expect("Failed to write cleaned file");
            eprintln!("Processed {} → removed {} chars", file_name, count);
        }
    }
}

pub fn print_help() {
    println!("🧼 Unicode Sanitizer CLI Help

Usage:
  SanitizeText <input_file_or_folder> [--output <output_path_or_dir>]

Options:
  -h, --help       Show this help message.
  -o, --output     Specify an output file (for single file) or output directory (for folder).
");
}
