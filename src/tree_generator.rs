use colored::*;
use std::collections::BTreeMap;
use std::fmt::Write;
use std::path::Path;

type Tree = BTreeMap<String, Node>;

#[derive(Debug, PartialEq)]
struct Node {
    status: Option<String>,
    children: Option<Tree>,
}

impl Node {
    fn new_file(status: Option<String>) -> Self {
        Node {
            status,
            children: None,
        }
    }

    fn new_directory() -> Self {
        Node {
            status: None,
            children: Some(BTreeMap::new()),
        }
    }
}

#[derive(Debug, PartialEq)]
enum LineEntry {
    File(String, Option<String>),
    Directory(String),
    Connector(String),
    Indent(String),
}

pub struct Options {
    pub compact: bool,
    pub color: bool,
}

pub fn generate_tree_from_paths(
    paths_with_status: &Vec<(String, String)>,
    options: &Options,
) -> String {
    let mut root = Tree::new();
    for (path_str, status) in paths_with_status {
        if !path_str.trim().is_empty() {
            let status_opt = if status.is_empty() {
                None
            } else {
                Some(status.clone())
            };
            add_path_to_tree(&mut root, Path::new(path_str), status_opt);
        }
    }

    let entries = format_tree_as_entries(&root, "", options.compact);
    let mut result = String::new();
    for entry in entries {
        match entry {
            LineEntry::File(s, status) => {
                let colored_s = if options.color {
                    apply_color(&s, status.as_deref())
                } else {
                    s.normal().to_string()
                };
                write!(&mut result, "{}\n", colored_s)
            }
            LineEntry::Directory(s) => write!(
                &mut result,
                "{}\n",
                if options.color {
                    s.blue().to_string()
                } else {
                    s
                }
            ),
            LineEntry::Connector(s) | LineEntry::Indent(s) => write!(
                &mut result,
                "{}",
                if options.color {
                    s.bright_black().to_string()
                } else {
                    s
                }
            ),
            // LineEntry::Indent(s) => write!(&mut result, "{}", s),
        }
        .unwrap();
    }
    result
}

fn apply_color(s: &str, status: Option<&str>) -> String {
    match status {
        Some("M") => s.yellow().to_string(),
        Some("A") => s.green().to_string(),
        Some("D") => s.red().to_string(),
        Some("R") => s.cyan().to_string(),
        Some("C") => s.magenta().to_string(),
        Some("U") => s.red().bold().to_string(),
        Some("??") => s.bright_black().to_string(),
        _ => s.normal().to_string(),
    }
}

fn add_path_to_tree(tree: &mut Tree, path: &Path, status: Option<String>) {
    let mut current_tree = tree;

    let components: Vec<_> = path
        .components()
        .filter_map(|c| {
            let name = c.as_os_str().to_string_lossy().into_owned();
            if name == "/" || name.ends_with(":\\") {
                None
            } else {
                Some(name)
            }
        })
        .collect();

    if components.is_empty() {
        return;
    }

    let last_index = components.len() - 1;
    for (i, component_name) in components.into_iter().enumerate() {
        if i == last_index {
            current_tree
                .entry(component_name)
                .or_insert_with(|| Node::new_file(status.clone()));
            continue;
        }
        let entry = current_tree
            .entry(component_name)
            .or_insert_with(Node::new_directory);

        if let Some(subtree) = &mut entry.children {
            current_tree = subtree;
        } else {
            break;
        }
    }
}

/// Recursively builds a vector of LineEntry structs representing the tree structure.
fn format_tree_as_entries(tree: &Tree, prefix: &str, compact: bool) -> Vec<LineEntry> {
    let mut entries = Vec::new();
    let mut iter = tree.iter().peekable();
    while let Some((name, node)) = iter.next() {
        let mut compacted_name = name.clone();
        let mut node_to_print = node;

        if compact {
            while let Some(current_subtree) = &node_to_print.children {
                if current_subtree.len() != 1 {
                    break;
                }
                let (child_name, child_node) = current_subtree.iter().next().unwrap();
                if child_node.children.is_some() {
                    compacted_name.push('/');
                    compacted_name.push_str(child_name);
                    node_to_print = child_node;
                    continue;
                }
                break;
            }
        }

        let is_last = iter.peek().is_none();
        let connector = if is_last { "└── " } else { "├── " };

        entries.push(LineEntry::Indent(prefix.to_string()));
        entries.push(LineEntry::Connector(connector.to_string()));
        entries.push(if node_to_print.children.is_some() {
            LineEntry::Directory(compacted_name)
        } else {
            LineEntry::File(compacted_name, node_to_print.status.clone())
        });

        if let Some(subtree) = &node_to_print.children {
            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            entries.extend(format_tree_as_entries(subtree, &new_prefix, compact));
        }
    }
    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_paths_with_status(paths: &[&str]) -> Vec<(String, String)> {
        paths
            .iter()
            .map(|&s| (s.to_string(), String::new()))
            .collect()
    }

    #[test]
    fn test_generate_tree() {
        assert_eq!(
            generate_tree_from_paths(
                &create_paths_with_status(&[
                    "nvim/after/lsp/tailwindcss.lua",
                    "nvim/after/lsp/ts_ls.lua",
                    "nvim/after/lsp/denols.lua",
                    "nvim/lazy-lock.json",
                    "nvim/lua/setup-lazynvim.lua",
                    "nvim/lua/install-lazynvim.lua",
                    "nvim/lua/options.lua",
                    "nvim/lua/plugins/ui.lua",
                    "nvim/lua/plugins/completion.lua",
                    "nvim/lua/plugins/treesitter.lua",
                    "nvim/lua/plugins/fuzzy-finder.lua",
                    "nvim/lua/plugins/colorscheme.lua",
                    "nvim/lua/plugins/manipulation.lua",
                    "nvim/lua/plugins/lsp.lua",
                    "nvim/lua/plugins/formatter.lua",
                    "nvim/lua/plugins/others.lua",
                    "nvim/lua/plugins/alpha-nvim.lua",
                    "nvim/lua/plugins/filer.lua",
                    "nvim/lua/use-extui.lua",
                    "nvim/lua/disable-providers.lua",
                    "nvim/lua/keymappings.lua",
                    "nvim/lua/easy-setup-autocmd/init.lua",
                    "nvim/lua/improve-default-scheme/init.lua",
                    "nvim/init.lua",
                    "nvim/ftplugin/qf.lua"
                ]),
                &Options {
                    compact: false,
                    color: false
                }
            ),
            r#"└── nvim
    ├── after
    │   └── lsp
    │       ├── denols.lua
    │       ├── tailwindcss.lua
    │       └── ts_ls.lua
    ├── ftplugin
    │   └── qf.lua
    ├── init.lua
    ├── lazy-lock.json
    └── lua
        ├── disable-providers.lua
        ├── easy-setup-autocmd
        │   └── init.lua
        ├── improve-default-scheme
        │   └── init.lua
        ├── install-lazynvim.lua
        ├── keymappings.lua
        ├── options.lua
        ├── plugins
        │   ├── alpha-nvim.lua
        │   ├── colorscheme.lua
        │   ├── completion.lua
        │   ├── filer.lua
        │   ├── formatter.lua
        │   ├── fuzzy-finder.lua
        │   ├── lsp.lua
        │   ├── manipulation.lua
        │   ├── others.lua
        │   ├── treesitter.lua
        │   └── ui.lua
        ├── setup-lazynvim.lua
        └── use-extui.lua
"#
        );
    }

    #[test]
    fn test_generate_tree_compact() {
        assert_eq!(
            generate_tree_from_paths(
                &create_paths_with_status(&[
                    "dotfiles/nvim/after/lsp/tailwindcss.lua",
                    "dotfiles/nvim/after/lsp/ts_ls.lua",
                    "dotfiles/nvim/after/lsp/denols.lua",
                    "dotfiles/nvim/lazy-lock.json",
                    "dotfiles/nvim/lua/setup-lazynvim.lua",
                    "dotfiles/nvim/lua/install-lazynvim.lua",
                    "dotfiles/nvim/lua/options.lua",
                    "dotfiles/nvim/lua/plugins/ui.lua",
                    "dotfiles/nvim/lua/plugins/completion.lua",
                    "dotfiles/nvim/lua/plugins/treesitter.lua",
                    "dotfiles/nvim/lua/plugins/fuzzy-finder.lua",
                    "dotfiles/nvim/lua/plugins/colorscheme.lua",
                    "dotfiles/nvim/lua/plugins/manipulation.lua",
                    "dotfiles/nvim/lua/plugins/lsp.lua",
                    "dotfiles/nvim/lua/plugins/formatter.lua",
                    "dotfiles/nvim/lua/plugins/others.lua",
                    "dotfiles/nvim/lua/plugins/alpha-nvim.lua",
                    "dotfiles/nvim/lua/plugins/filer.lua",
                    "dotfiles/nvim/lua/use-extui.lua",
                    "dotfiles/nvim/lua/disable-providers.lua",
                    "dotfiles/nvim/lua/keymappings.lua",
                    "dotfiles/nvim/lua/easy-setup-autocmd/init.lua",
                    "dotfiles/nvim/lua/improve-default-scheme/init.lua",
                    "dotfiles/nvim/init.lua",
                    "dotfiles/nvim/ftplugin/qf.lua"
                ]),
                &Options {
                    compact: true,
                    color: false
                }
            ),
            r#"└── dotfiles/nvim
    ├── after/lsp
    │   ├── denols.lua
    │   ├── tailwindcss.lua
    │   └── ts_ls.lua
    ├── ftplugin
    │   └── qf.lua
    ├── init.lua
    ├── lazy-lock.json
    └── lua
        ├── disable-providers.lua
        ├── easy-setup-autocmd
        │   └── init.lua
        ├── improve-default-scheme
        │   └── init.lua
        ├── install-lazynvim.lua
        ├── keymappings.lua
        ├── options.lua
        ├── plugins
        │   ├── alpha-nvim.lua
        │   ├── colorscheme.lua
        │   ├── completion.lua
        │   ├── filer.lua
        │   ├── formatter.lua
        │   ├── fuzzy-finder.lua
        │   ├── lsp.lua
        │   ├── manipulation.lua
        │   ├── others.lua
        │   ├── treesitter.lua
        │   └── ui.lua
        ├── setup-lazynvim.lua
        └── use-extui.lua
"#
        );
    }

    #[test]
    fn test_format_tree_as_lines() {
        let mut tree = Tree::new();
        add_path_to_tree(&mut tree, Path::new("a/b"), Some("M".to_string()));
        add_path_to_tree(&mut tree, Path::new("a/c"), Some("A".to_string()));

        let lines = format_tree_as_entries(&tree, "", false);

        assert_eq!(
            lines,
            vec![
                LineEntry::Indent("".to_string()),
                LineEntry::Connector("└── ".to_string()),
                LineEntry::Directory("a".to_string()),
                LineEntry::Indent("    ".to_string()),
                LineEntry::Connector("├── ".to_string()),
                LineEntry::File("b".to_string(), Some("M".to_string())),
                LineEntry::Indent("    ".to_string()),
                LineEntry::Connector("└── ".to_string()),
                LineEntry::File("c".to_string(), Some("A".to_string()))
            ]
        );
    }

    #[test]
    fn test_generate_tree_with_color() {
        colored::control::set_override(true);
        let paths = vec![
            ("a/b".to_string(), "M".to_string()),
            ("a/c".to_string(), "A".to_string()),
        ];
        let options = &Options {
            compact: false,
            color: true,
        };
        assert_eq!(
            generate_tree_from_paths(&paths, options),
            "\u{1b}[90m\u{1b}[0m\u{1b}[90m└── \u{1b}[0m\u{1b}[34ma\u{1b}[0m\n\u{1b}[90m    \u{1b}[0m\u{1b}[90m├── \u{1b}[0m\u{1b}[33mb\u{1b}[0m\n\u{1b}[90m    \u{1b}[0m\u{1b}[90m└── \u{1b}[0m\u{1b}[32mc\u{1b}[0m\n"
        );
    }

    #[test]
    fn test_generate_tree_from_porcelain_output() {
        colored::control::set_override(true);
        let paths = vec![
            ("src/main.rs".to_string(), "M".to_string()),
            ("src/tree_generator.rs".to_string(), "M".to_string()),
            ("new_file.txt".to_string(), "A".to_string()),
            ("deleted_file.txt".to_string(), "D".to_string()),
            ("renamed_file.txt".to_string(), "R".to_string()),
            ("copied_file.txt".to_string(), "C".to_string()),
            ("unmerged_file.txt".to_string(), "U".to_string()),
            ("untracked_file.txt".to_string(), "??".to_string()),
        ];
        let options = &Options {
            compact: false,
            color: true,
        };
        let expected = "\u{1b}[90m\u{1b}[0m\u{1b}[90m├── \u{1b}[0m\u{1b}[35mcopied_file.txt\u{1b}[0m\n\u{1b}[90m\u{1b}[0m\u{1b}[90m├── \u{1b}[0m\u{1b}[31mdeleted_file.txt\u{1b}[0m\n\u{1b}[90m\u{1b}[0m\u{1b}[90m├── \u{1b}[0m\u{1b}[32mnew_file.txt\u{1b}[0m\n\u{1b}[90m\u{1b}[0m\u{1b}[90m├── \u{1b}[0m\u{1b}[36mrenamed_file.txt\u{1b}[0m\n\u{1b}[90m\u{1b}[0m\u{1b}[90m├── \u{1b}[0m\u{1b}[34msrc\u{1b}[0m\n\u{1b}[90m│   \u{1b}[0m\u{1b}[90m├── \u{1b}[0m\u{1b}[33mmain.rs\u{1b}[0m\n\u{1b}[90m│   \u{1b}[0m\u{1b}[90m└── \u{1b}[0m\u{1b}[33mtree_generator.rs\u{1b}[0m\n\u{1b}[90m\u{1b}[0m\u{1b}[90m├── \u{1b}[0m\u{1b}[1;31munmerged_file.txt\u{1b}[0m\n\u{1b}[90m\u{1b}[0m\u{1b}[90m└── \u{1b}[0m\u{1b}[90muntracked_file.txt\u{1b}[0m\n";

        assert_eq!(generate_tree_from_paths(&paths, options), expected);
    }
}
