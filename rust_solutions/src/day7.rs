use itertools::Itertools;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::fmt::Display;
use std::iter::{once, Iterator};
use std::rc::{Rc, Weak};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
fn generate(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|line| {
            let mut c_line = line.split_whitespace();
            match (c_line.next(), c_line.next()) {
                (Some("$"), Some("cd")) => Entry::Cd {
                    dir: c_line.next().unwrap().to_string(),
                },
                (Some("$"), Some("ls")) => Entry::List,
                (Some("dir"), Some(name)) => Entry::Dir {
                    name: name.to_string(),
                },
                (Some(size), Some(name)) => Entry::File {
                    size: size.parse().unwrap(),
                    name: name.to_string(),
                },
                (_, _) => Entry::None,
            }
        })
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &[Entry]) -> usize {
    let sum = File::from_entries(input)
        .iter()
        .filter(|file| match &*file.borrow() {
            File::File { name: _, size: _ } => false,
            File::Dir {
                name: _,
                nodes: _,
                parent: _,
            } => true,
        })
        .filter(|file| file.size() <= 100000)
        .map(|file| file.size())
        .sum();
    sum
}

#[aoc(day7, part2)]
fn part2(input: &[Entry]) -> usize {
    let root = File::from_entries(input);
    let det = 30_000_000 - (70_000_000 - root.size());
    let sum = root
        .iter()
        .filter(|file| match &*file.borrow() {
            File::File { name: _, size: _ } => false,
            File::Dir {
                name: _,
                nodes: _,
                parent: _,
            } => true,
        })
        .filter(|file| file.size() >= det)
        .sorted_by_key(|file| file.size())
        .map(|file| file.size())
        .nth(0);
    sum.unwrap()
}

#[derive(Debug)]
enum Entry {
    Cd { dir: String },
    List,
    Dir { name: String },
    File { size: usize, name: String },
    None,
}

#[derive(Debug)]
enum File {
    File {
        name: String,
        size: usize,
    },
    Dir {
        name: String,
        nodes: RefCell<Vec<Rc<File>>>,
        parent: RefCell<Weak<File>>,
    },
}

impl File {
    fn from_entries(entries: &[Entry]) -> Rc<File> {
        let root = Rc::new(File::Dir {
            name: "/".to_string(),
            nodes: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new()),
        });
        let mut current = Rc::clone(&root);

        for entry in &entries[1..] {
            let temp_current = Rc::clone(&current);
            match entry {
                Entry::Cd { dir } => match &*temp_current {
                    File::Dir {
                        name: _,
                        nodes: _,
                        parent,
                    } if dir == ".." => {
                        if let Some(p) = parent.borrow().upgrade() {
                            current = Rc::clone(&p);
                        }
                    }
                    File::Dir {
                        name: _,
                        nodes,
                        parent: _,
                    } => {
                        for node in &*nodes.borrow() {
                            if let File::Dir {
                                name,
                                nodes: _,
                                parent: _,
                            } = node.borrow()
                            {
                                if name == dir {
                                    current = Rc::clone(&node);
                                }
                            }
                        }
                    }
                    File::File { name: _, size: _ } => unreachable!(),
                },
                Entry::List => {}
                Entry::Dir { name } => match &*temp_current {
                    File::File { name: _, size: _ } => unreachable!(),
                    File::Dir {
                        name: _,
                        nodes,
                        parent: _,
                    } => {
                        nodes.borrow_mut().push(Rc::new(File::Dir {
                            name: name.to_string(),
                            nodes: RefCell::new(vec![]),
                            parent: RefCell::new(Rc::downgrade(&current)),
                        }));
                    }
                },
                Entry::File { size, name } => match &*temp_current {
                    File::File { name: _, size: _ } => unreachable!(),
                    File::Dir {
                        name: _,
                        nodes,
                        parent: _,
                    } => {
                        nodes.borrow_mut().push(Rc::new(File::File {
                            name: name.to_string(),
                            size: *size,
                        }));
                    }
                },
                _ => {}
            }
        }
        root
    }

    fn size(&self) -> usize {
        match self {
            File::File { name: _, size } => *size,
            File::Dir {
                name: _,
                nodes,
                parent: _,
            } => nodes.borrow().iter().map(|file| file.size()).sum(),
        }
    }

    fn iter(self: &Rc<Self>) -> Box<dyn Iterator<Item = Rc<File>>> {
        match &*self.borrow() {
            File::File { name: _, size: _ } => Box::new(once(Rc::clone(&self))),
            File::Dir {
                name: _,
                nodes,
                parent: _,
            } => {
                let nodes_iter = nodes
                    .borrow()
                    .iter()
                    .flat_map(|node| Rc::clone(node).iter())
                    .collect::<Vec<_>>();
                Box::new(once(Rc::clone(&self)).chain(nodes_iter.into_iter()))
            }
        }
    }

    fn format(&self, n: usize) -> String {
        match self {
            File::File { name, size } => format!("{}- {name} (file, size={size})", "  ".repeat(n)),
            File::Dir {
                name,
                nodes,
                parent: _,
            } => {
                let top = format!("{}- {name} (dir)", "  ".repeat(n));
                let files_fmt = nodes
                    .borrow()
                    .iter()
                    .map(|node| node.format(n + 1))
                    .join("\n");
                if files_fmt.is_empty() {
                    top
                } else {
                    format!("{top}\n{files_fmt}")
                }
            }
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format(0))
    }
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc, rc::Weak};

    use crate::day7::File;

    #[test]
    fn pass() {
        let tree = File::Dir {
            parent: RefCell::new(Weak::new()),
            name: "/".to_string(),
            nodes: RefCell::new(vec![
                Rc::new(File::Dir {
                    name: "a".to_string(),
                    nodes: RefCell::new(vec![]),
                    parent: RefCell::new(Weak::new()),
                }),
                Rc::new(File::File {
                    name: "b".to_string(),
                    size: 345,
                }),
                Rc::new(File::Dir {
                    name: "e".to_string(),
                    nodes: RefCell::new(vec![Rc::new(File::File {
                        name: "f".to_string(),
                        size: 546,
                    })]),
                    parent: RefCell::new(Weak::new()),
                }),
            ]),
        };
        eprintln!("{tree}");
        assert_eq!(tree.size(), 345 + 546);
    }

    #[test]
    fn iterator() {
        let tree = File::Dir {
            parent: RefCell::new(Weak::new()),
            name: "/".to_string(),
            nodes: RefCell::new(vec![
                Rc::new(File::Dir {
                    name: "a".to_string(),
                    nodes: RefCell::new(vec![]),
                    parent: RefCell::new(Weak::new()),
                }),
                Rc::new(File::File {
                    name: "b".to_string(),
                    size: 345,
                }),
                Rc::new(File::Dir {
                    name: "e".to_string(),
                    nodes: RefCell::new(vec![Rc::new(File::File {
                        name: "f".to_string(),
                        size: 546,
                    })]),
                    parent: RefCell::new(Weak::new()),
                }),
            ]),
        };

        for node in Rc::new(tree).iter() {
            println!("{node}");
        }
    }
}
