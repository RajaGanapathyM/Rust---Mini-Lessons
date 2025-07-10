#[derive(Debug)]
enum FsEntry {
    File { name: String, size: u64 },
    Directory { name: String, child_file: Vec<FsEntry> },
    SymLink { name: String, target: String },
}

fn print_tree(entry: &FsEntry, indent: usize) {
    let space = " ".repeat(indent);

    match entry {
        FsEntry::File { name, size } => {
            println!("{}File: {} ({} bytes)", space, name, size);
        }
        FsEntry::Directory { name, child_file } => {
            println!("{}Directory: {}", space, name);
            for sub_entry in child_file {
                print_tree(sub_entry, indent + 2);
            }
        }
        FsEntry::SymLink { name, target } => {
            println!("{}Symlink: {} -> {}", space, name, target);
        }
    }
}

fn main() {
    let fs = FsEntry::Directory {
        name: String::from("root"),
        child_file: vec![
            FsEntry::File {
                name: String::from("file1.txt"),
                size: 1024,
            },
            FsEntry::Directory {
                name: String::from("subdir"),
                child_file: vec![
                    FsEntry::File {
                        name: String::from("file2.txt"),
                        size: 512,
                    },
                    FsEntry::SymLink {
                        name: String::from("link_to_file1"),
                        target: String::from("file1.txt"),
                    },
                ],
            },
            FsEntry::SymLink {
                name: String::from("link_to_subdir"),
                target: String::from("subdir"),
            },
        ],
    };

    print_tree(&fs, 0);
}
