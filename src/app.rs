use crate::commands::{args::RunArgs, run};
use dioxus::prelude::*;
use rfd::FileDialog;
use std::path::PathBuf;

#[component]
pub fn App() -> Element {
    // --- UI State ---
    let mut selected_dir = use_signal(|| None::<PathBuf>);
    let mut exclusions_input = use_signal(|| String::new());
    let mut use_clipboard = use_signal(|| true);
    let mut fast_mode = use_signal(|| false);
    let mut is_processing = use_signal(|| false);
    let mut status_message = use_signal(|| String::from("Waiting for input..."));

    // --- Handlers ---
    let pick_directory = move |_| {
        if let Some(folder) = FileDialog::new().pick_folder() {
            selected_dir.set(Some(folder));
            status_message.set(String::from("Directory selected. Ready to run!"));
        }
    };

    let run_treeclip = move |_| {
        if let Some(path) = selected_dir.read().clone() {
            is_processing.set(true);
            status_message.set(String::from("Processing... Check your terminal for logs."));

            // Parse exclusions from comma-separated string
            let exclude_list: Vec<String> = exclusions_input
                .read()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            let clipboard = *use_clipboard.read();
            let fast = *fast_mode.read();

            // Construct YOUR existing arguments struct!
            // Note: If your RunArgs in `src/commands/args.rs` has different field names,
            // simply adjust these lines to match your actual struct definition.
            let args = RunArgs {
                input_paths: vec![path],
                output_path: None,
                root: None,
                exclude: exclude_list,
                clipboard,
                fast_mode: fast,
                // Defaulting other flags to false/None so it matches standard execution
                editor: false,
                delete: false,
                stats: true,
                verbose: false,
                skip_hidden: false,
                raw: false,
                tree: false,
            };

            // Spawn a blocking background task so the UI doesn't freeze
            spawn(async move {
                let result = tokio::task::spawn_blocking(move || {
                    // Call your exact existing logic!
                    run::execute(args)
                })
                .await;

                is_processing.set(false);

                match result {
                    Ok(Ok(_)) => {
                        status_message.set(String::from("✅ Success! Check clipboard/output."))
                    }
                    Ok(Err(e)) => status_message.set(format!("❌ Error running core: {}", e)),
                    Err(e) => status_message.set(format!("❌ Thread error: {}", e)),
                }
            });
        }
    };

    // --- UI Layout ---
    rsx! {
        style {
            "
            body {{ font-family: sans-serif; background: #1e1e2e; color: #cdd6f4; padding: 20px; }}
            .card {{ background: #181825; padding: 20px; border-radius: 8px; max-width: 600px; margin: 0 auto; }}
            .form-group {{ margin-bottom: 15px; display: flex; flex-direction: column; }}
            label {{ font-weight: bold; margin-bottom: 5px; color: #a6e3a1; }}
            input[type='text'] {{ padding: 8px; background: #313244; color: white; border: 1px solid #45475a; border-radius: 4px; }}
            button {{ padding: 10px; border: none; border-radius: 4px; background: #89b4fa; color: #11111b; font-weight: bold; cursor: pointer; }}
            button:hover {{ background: #b4befe; }}
            button:disabled {{ background: #45475a; cursor: not-allowed; }}
            .status {{ margin-top: 15px; padding: 10px; background: #313244; border-left: 4px solid #89b4fa; border-radius: 4px; }}
            .checkbox-group {{ display: flex; align-items: center; gap: 8px; flex-direction: row; }}
            "
        }

        div { class: "card",
            h1 { "🌳 TreeClip GUI" }

            div { class: "form-group",
                label { "1. Target Directory" }
                button {
                    onclick: pick_directory,
                    disabled: *is_processing.read(),
                    "📂 Pick Directory"
                }
                if let Some(p) = selected_dir.read().as_ref() {
                    span { style: "margin-top: 8px; color: #f38ba8;", "{p.display()}" }
                }
            }

            div { class: "form-group",
                label { "2. Exclusions (comma separated)" }
                input {
                    r#type: "text",
                    value: "{exclusions_input}",
                    oninput: move |e| exclusions_input.set(e.value().clone()),
                    disabled: *is_processing.read(),
                    placeholder: "node_modules, target, .git"
                }
            }

            div { class: "form-group checkbox-group",
                input {
                    r#type: "checkbox",
                    checked: "{use_clipboard}",
                    onchange: move |e| use_clipboard.set(e.value().parse().unwrap_or(true)),
                    disabled: *is_processing.read(),
                }
                label { style: "margin: 0;", "Copy to Clipboard" }
            }

            div { class: "form-group checkbox-group",
                input {
                    r#type: "checkbox",
                    checked: "{fast_mode}",
                    onchange: move |e| fast_mode.set(e.value().parse().unwrap_or(false)),
                    disabled: *is_processing.read(),
                }
                label { style: "margin: 0;", "Fast Mode" }
            }

            button {
                style: "width: 100%; margin-top: 10px;",
                onclick: run_treeclip,
                disabled: selected_dir.read().is_none() || *is_processing.read(),
                if *is_processing.read() {
                    "⏳ Running..."
                } else {
                    "🚀 Execute Bundle"
                }
            }

            div { class: "status",
                strong { "Status: " }
                "{status_message}"
            }
        }
    }
}
