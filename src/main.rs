use dioxus::{document::Title, prelude::*};

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    let mut input_text = use_signal(|| String::new());
    let mut output_text = use_signal(|| String::new());
    let mut explanation_list = use_signal(|| Vec::<String>::new());
    let mut removed_count = use_signal(|| 0);

    let removal_summary = if removed_count() > 0 {
        rsx! {
            Fragment {
                h3 { "Removed {removed_count()} invisible characters" }
                ul {
                    for desc in explanation_list().iter() {
                        li { "{desc}" }
                    }
                }
            }
        }
    } else {
        rsx!(Fragment {})
    };

    rsx! {
        Title{ "🧼 Unicode Sanitizer" }
            div {
                style: "
                    max-width: 800px;
                    margin: auto;
                    padding: 20px;
                    font-family: 'Segoe UI', sans-serif;
                    background-color: #1e1e1e;
                    color: #dcdcdc;
                    border-radius: 12px;
                    box-shadow: 0 0 12px rgba(0,0,0,0.5);
                ",
                h1 { "🧼 Unicode Sanitizer" }

                textarea {
                    rows: "10",
                    cols: "80",
                    value: "{input_text()}",
                    oninput: move |e| input_text.set(e.value().clone()),
                    placeholder: "Paste your text here...",
                    style: "
                        background-color: #2d2d2d;
                        color: #ffffff;
                        border: 1px solid #444;
                        border-radius: 6px;
                        padding: 8px;
                    ",
                }

                div {
                    button {
                        onclick: move |_| {
                            let (cleaned, count, found) = sanitizetext::clean_invisible(&input_text());
                            output_text.set(cleaned);
                            removed_count.set(count);
                            explanation_list.set(found);
                        },
                        "🧹 Clean"
                    }
                }
               
               {removal_summary}
               
                h2 { "Cleaned Output" }
                pre {
                    style: "
                        background-color: #2a2a2a;
                        color: #cceeff;
                        padding: 10px;
                        border-radius: 6px;
                        overflow-x: auto;
                        white-space: pre-wrap;
                    ",
                    "{output_text()}"
                }

 
            }
        
    }
}
