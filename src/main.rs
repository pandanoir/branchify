use clap::Parser;
use std::collections::BTreeMap;
use std::fmt::Write;
use std::io::{self, BufRead};
use std::path::Path;

type Tree = BTreeMap<String, Node>;

#[derive(Debug, PartialEq)]
enum Node {
    File,
    Directory(Tree),
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    compact: bool,
}

fn main() {
    let paths = io::stdin().lock().lines().map_while(Result::ok).collect();
    let args = Args::parse();

    println!("{}", generate_tree_from_paths(&paths, args.compact));
}

fn generate_tree_from_paths(paths: &Vec<String>, compact: bool) -> String {
    let mut root = Tree::new();
    for path_str in paths {
        if !path_str.trim().is_empty() {
            add_path_to_tree(&mut root, Path::new(&path_str));
        }
    }

    return format_tree_as_string(&root, "", compact);
}

fn add_path_to_tree(tree: &mut Tree, path: &Path) {
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
            current_tree.entry(component_name).or_insert(Node::File);
            continue;
        }
        let entry = current_tree
            .entry(component_name)
            .or_insert_with(|| Node::Directory(BTreeMap::new()));

        if let Node::Directory(subtree) = entry {
            current_tree = subtree;
        } else {
            break;
        }
    }
}

/// Recursively prints the tree structure to the console.
fn format_tree_as_string(tree: &Tree, prefix: &str, compact: bool) -> String {
    let mut result = String::new();
    let mut iter = tree.iter().peekable();
    while let Some((name, node)) = iter.next() {
        let mut compacted_name = name.clone();
        let mut node_to_print = node;

        if compact {
            // Look ahead for chains of single-directory entries to compact.
            while let Node::Directory(current_subtree) = node_to_print {
                if current_subtree.len() != 1 {
                    break;
                }
                let (child_name, child_node) = current_subtree.iter().next().unwrap();
                if let Node::Directory(_) = child_node {
                    compacted_name.push('/');
                    compacted_name.push_str(child_name);
                    node_to_print = child_node;
                    continue;
                }
                break;
            }
        }

        let is_last = iter.peek().is_none();

        write!(
            &mut result,
            "{}{}{}\n",
            prefix,
            if is_last { "└── " } else { "├── " },
            compacted_name
        )
        .unwrap();

        if let Node::Directory(subtree) = node_to_print {
            result += format_tree_as_string(
                subtree,
                &format!("{}{}", prefix, if is_last { "    " } else { "│   " }),
                compact,
            )
            .as_str();
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_tree() {
        assert_eq!(
            generate_tree_from_paths(
                &[
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
                ]
                .iter()
                .map(|&s| s.into())
                .collect(),
                false
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
                &[
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
                ]
                .iter()
                .map(|&s| s.into())
                .collect(),
                true
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
}
