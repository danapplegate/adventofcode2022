use crate::solvable::Solvable;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::fmt::Debug;
use std::fs;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::{error::Error, path::PathBuf};

pub struct Solution {
    filepath: PathBuf,
}

const MAXIMUM_DISK_SPACE: u32 = 70000000;
const REQUIRED_DISK_SPACE: u32 = 30000000;

enum FilesystemEntry {
    Directory {
        name: String,
        parent: Option<Rc<RefCell<FilesystemEntry>>>,
        entries: Vec<Rc<RefCell<FilesystemEntry>>>,
        size: Option<u32>,
    },
    File {
        name: String,
        parent: Option<Rc<RefCell<FilesystemEntry>>>,
        size: u32,
    },
}
use FilesystemEntry::*;

impl FilesystemEntry {
    fn is_directory(&self) -> bool {
        match self {
            FilesystemEntry::Directory { .. } => true,
            FilesystemEntry::File { .. } => false,
        }
    }

    fn entries(&mut self) -> &mut Vec<Rc<RefCell<FilesystemEntry>>> {
        match self {
            FilesystemEntry::Directory {
                ref mut entries, ..
            } => entries,
            FilesystemEntry::File { .. } => panic!("Tried to get entries of a File"),
        }
    }

    fn name(&self) -> &str {
        match self {
            FilesystemEntry::Directory { name, .. } | FilesystemEntry::File { name, .. } => &name,
        }
    }

    fn parent(&self) -> Option<Rc<RefCell<FilesystemEntry>>> {
        match self {
            FilesystemEntry::Directory { parent, .. } | FilesystemEntry::File { parent, .. } => {
                parent.clone()
            }
        }
    }

    fn size(&mut self) -> u32 {
        match self {
            FilesystemEntry::File { size, .. } => *size,
            FilesystemEntry::Directory {
                entries,
                ref mut size,
                ..
            } => {
                if *size == None {
                    *size = Some(entries.iter().map(|e| e.as_ref().borrow_mut().size()).sum());
                }
                size.unwrap()
            }
        }
    }
}

impl Debug for FilesystemEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilesystemEntry::Directory {
                name,
                size,
                entries,
                ..
            } => f.write_fmt(format_args!(
                "Directory {{ name: {:?}, size: {:?}, entries: {:?} }}",
                name, size, entries
            ))?,
            FilesystemEntry::File { name, size, .. } => f.write_fmt(format_args!(
                "File {{ name: {:?}, size: {:?} }}",
                name, size
            ))?,
        }
        Ok(())
    }
}

fn build_filesystem(input: String) -> Result<Rc<RefCell<FilesystemEntry>>, Box<dyn Error>> {
    let root = Rc::new(RefCell::new(Directory {
        name: "/".to_string(),
        parent: None,
        entries: vec![],
        size: None,
    }));
    let mut current = root.clone();
    let mut iter = input.lines().skip(1).peekable();
    let mut line = iter.next();
    while let Some(l) = line {
        if l.starts_with("$ cd ..") {
            let new_current = current.as_ref().borrow().parent();
            current = new_current.unwrap();
            line = iter.next();
        } else if l.starts_with("$ cd") {
            let dir = &l[5..];
            let name = dir.trim();
            let temp_current = current.clone();
            let mut borrowed = temp_current.as_ref().borrow_mut();
            let entries = borrowed.entries();
            current = entries
                .iter()
                .find(|e| e.borrow().name() == name)
                .unwrap()
                .clone();
            line = iter.next();
        } else if l.starts_with("$ ls") {
            line = iter.next();
            while line != None && !line.unwrap().starts_with("$") {
                let mut borrowed = current.as_ref().borrow_mut();
                let entries = borrowed.entries();
                let line_val = line.unwrap();
                if line_val.starts_with("dir") {
                    let dirname = &line_val[4..];
                    entries.push(Rc::new(RefCell::new(Directory {
                        name: dirname.to_string(),
                        parent: Some(current.clone()),
                        entries: vec![],
                        size: None,
                    })))
                } else if char::is_digit(line_val.chars().collect::<Vec<char>>()[0], 10) {
                    let mut parts = line_val.split_whitespace();
                    let size = parts.next().unwrap();
                    let name = parts.next().unwrap();
                    entries.push(Rc::new(RefCell::new(File {
                        name: name.to_string(),
                        size: size.parse::<u32>()?,
                        parent: Some(current.clone()),
                    })));
                }
                line = iter.next();
            }
        }
    }
    Ok(root)
}

fn find_dir_sizes<F>(filesystem: Rc<RefCell<FilesystemEntry>>, f: F) -> Vec<u32>
where
    F: Fn(u32) -> bool,
{
    let mut found: Vec<u32> = vec![];
    let mut to_visit: Vec<Rc<RefCell<FilesystemEntry>>> = vec![filesystem];
    while let Some(current) = to_visit.pop() {
        let mut borrow = current.as_ref().borrow_mut();
        let size = borrow.size();
        to_visit.extend(
            borrow
                .entries()
                .iter()
                .filter(|e| e.as_ref().borrow().is_directory())
                .map(|e| e.clone()),
        );
        if f(size) {
            found.push(size);
        }
    }
    found
}

impl Solvable<7> for Solution {
    fn new(filename: &str) -> Self {
        Solution {
            filepath: Self::data_path().join(filename),
        }
    }

    fn answer1(&self) -> Result<String, Box<dyn Error>> {
        let input = fs::read_to_string(&self.filepath)?;
        let filesystem = build_filesystem(input)?;
        let found = find_dir_sizes(filesystem, |s| s <= 100_000);
        Ok(found.iter().sum::<u32>().to_string())
    }

    fn answer2(&self) -> Result<String, Box<dyn Error>> {
        let input = fs::read_to_string(&self.filepath)?;
        let filesystem = build_filesystem(input)?;
        let fs_size = filesystem.as_ref().borrow_mut().size();
        let current_free = MAXIMUM_DISK_SPACE - fs_size;
        let minimum_to_delete = REQUIRED_DISK_SPACE - current_free;
        let deletable_sizes: Vec<u32> = find_dir_sizes(filesystem, |s| s >= minimum_to_delete);
        Ok(deletable_sizes.iter().min().unwrap().to_string())
    }
}

#[allow(unused_imports)]
mod test {
    use super::*;

    #[test]
    fn it_gives_the_right_answer1_for_test1() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test1.txt");
        assert_eq!(solution.answer1()?, "95437");
        Ok(())
    }

    #[test]
    fn it_gives_the_right_answer2_for_test1() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test1.txt");
        assert_eq!(solution.answer2()?, "24933642");
        Ok(())
    }
}
