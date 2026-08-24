//! help - Stateless content for the help overlay.
//!
//! Pure data: lines of `(key, description)` tuples that the renderer formats
//! into the help popup. No state, no rendering, no I/O.

/// One line of help: a keybind + its description.
#[derive(Debug, Clone, Copy)]
pub struct HelpLine {
    pub key: &'static str,
    pub desc: &'static str,
}

/// The whole help content, grouped into logical sections.
pub fn sections() -> &'static [(&'static str, &'static [HelpLine])] {
    &[
        (
            "Navigation",
            &[
                HelpLine { key: "↑ / k",    desc: "Move cursor up" },
                HelpLine { key: "↓ / j",    desc: "Move cursor down" },
                HelpLine { key: "→ / l",    desc: "Expand directory" },
                HelpLine { key: "← / h",    desc: "Collapse directory" },
                HelpLine { key: "Tab",      desc: "Switch between Tree / Options" },
                HelpLine { key: "PageUp",   desc: "Jump up by a page" },
                HelpLine { key: "PageDown", desc: "Jump down by a page" },
                HelpLine { key: "Home",     desc: "Jump to top" },
                HelpLine { key: "End",      desc: "Jump to bottom" },
                HelpLine { key: "g",        desc: "Top of tree" },
                HelpLine { key: "G",        desc: "Bottom of tree" },
            ],
        ),
        (
            "Selection",
            &[
                HelpLine { key: "Space", desc: "Toggle include (file / dir + descendants)" },
                HelpLine { key: "x",     desc: "Toggle exclude (file / dir + descendants)" },
                HelpLine { key: "a",     desc: "Select everything" },
                HelpLine { key: "A",     desc: "Deselect everything" },
                HelpLine { key: "X",     desc: "Open exclude-glob input box" },
                HelpLine { key: "I",     desc: "Open include-glob input box" },
            ],
        ),
        (
            "Tree",
            &[
                HelpLine { key: "*", desc: "Expand all directories" },
                HelpLine { key: "_", desc: "Collapse all directories" },
            ],
        ),
        (
            "Options Panel",
            &[
                HelpLine { key: "Space",     desc: "Toggle highlighted option" },
                HelpLine { key: "↑ / ↓",      desc: "Move between options" },
                HelpLine { key: "o",         desc: "Edit output path" },
                HelpLine { key: "Enter",     desc: "Edit output path (when Options panel is focused)" },
            ],
        ),
        (
            "Actions",
            &[
                HelpLine { key: "Enter", desc: "Run TreeClip with current selection" },
                HelpLine { key: "r",     desc: "Run TreeClip with current selection" },
                HelpLine { key: "?",     desc: "Toggle this help" },
                HelpLine { key: "q",     desc: "Quit without running" },
                HelpLine { key: "Esc",   desc: "Cancel popup / quit" },
            ],
        ),
    ]
}

/// A one-line summary of the most useful keybinds - shown in the bottom help bar.
pub fn footer_hints() -> &'static [(&'static str, &'static str)] {
    &[
        ("?", "help"),
        ("Space", "select"),
        ("x", "exclude"),
        ("I", "include-glob"),
        ("X", "exclude-glob"),
        ("*", "expand all"),
        ("_", "collapse all"),
        ("Tab", "panel"),
        ("r/Enter", "run"),
        ("q", "quit"),
    ]
}
