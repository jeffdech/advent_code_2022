use std::boxed::Box;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use camino::{Utf8Path, Utf8PathBuf};

use nom::{
    branch::{alt},
    bytes::complete::{take_while1, tag},
    combinator::{map, opt},
    multi::many1,
    sequence::{preceded, separated_pair, terminated},
    IResult
};

pub type PResult<'a, T> = IResult<&'a str, T>;

#[derive(Debug)]
pub enum Command {
    Ls,
    Cd(Utf8PathBuf),
}

#[derive(Debug)]
pub enum Entry {
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf)
}

#[derive(Debug)]
pub enum Line {
    Command(Command),
    Entry(Entry)
}

pub type NodeHandle = Rc<RefCell<Node>>;

#[derive(Default)]
pub struct Node {
    size: u64,
    children: HashMap<Utf8PathBuf, NodeHandle>,
    parent: Option<NodeHandle>
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("size", &self.size)
            .field("children", &self.children)
            .finish()
    }
}

impl Node {
    fn is_dir(&self) -> bool {
        self.size == 0 && !self.children.is_empty()
    }
    pub fn total_size(&self) -> u64 {
        let child_size: u64 = self.children.values().map(|v| v.borrow().total_size()).sum();
        self.size + child_size
    }
}

pub fn all_dirs(n: NodeHandle) -> Box<dyn Iterator<Item = NodeHandle>> {
    // #[allow(clippy::needless_collect)]
    let children = n.borrow().children.values().cloned().collect::<Vec<_>>();
    Box::new(
        std::iter::once(n.clone()).chain(
            children
                .into_iter()
                .filter_map(|c| {
                    if c.borrow().is_dir() {
                        Some(all_dirs(c))
                    } else {
                        None
                    }
                })
                .flatten()
        )
    )
}

pub fn min_removal(n: NodeHandle) -> u64 {
    let total_space = 70000000_u64;
    let needed_space = 30000000_u64;
    let used_space = n.borrow().total_size();
    let free_space = total_space.checked_sub(used_space).unwrap();
    let min_free = needed_space.checked_sub(free_space).unwrap();

    let removed_dir_size = all_dirs(n)
        .map(|d| d.borrow().total_size())
        .filter(|&s| s >= min_free)
        .inspect(|s| {dbg!(s);})
        .min();
    removed_dir_size.unwrap()
}

pub struct PrettyNode<'a>(pub &'a NodeHandle);

impl<'a> fmt::Debug for PrettyNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let this = self.0.borrow();
        if this.size == 0 {
            writeln!(f, "(dir)")?;
        } else {
            writeln!(f, "(file, size={})", this.size)?;
        }

        for (name, child) in &this.children {
            for (index, line) in format!("{:?}", PrettyNode(child)).lines().enumerate() {
                if index == 0 {
                    writeln!(f, "{name} {line}")?;
                } else {
                    writeln!(f, "   {line}")?;
                }
            }
        }

        Ok(())
    }
}

pub fn walk_lines(lines: Vec<Line>) -> NodeHandle {
    let root = Rc::new(RefCell::new(Node::default()));
    let mut node = root.clone();

    for line in lines.iter() {
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => {},
                Command::Cd(path) => match path.as_str() {
                    "/" => {},   // already at root and there's only one
                    ".." => {   // go to parent
                        let parent = node.borrow().parent.clone().unwrap();
                        node = parent;
                    }
                    _ => {      // add child node and change to
                        let child = node.borrow_mut().children.entry(path.clone()).or_default().clone();
                        node = child;
                    }
                }
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(dir) => {
                    let entry = node.borrow_mut().children.entry(dir.clone()).or_default().clone();
                    entry.borrow_mut().parent = Some(node.clone());
                },
                Entry::File(size, path) => {
                    let entry = node.borrow_mut().children.entry(path.clone()).or_default().clone();
                    entry.borrow_mut().size = *size;
                    entry.borrow_mut().parent = Some(node.clone());
                }
            }
        }
    }

    root
}

pub mod parser {
    use super::*;

    fn parse_path(i: &str) -> PResult<Utf8PathBuf> {
        map(
            take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
            Into::into
        )(i)
    }

    fn parse_ls(i: &str) -> PResult<Command> {
        map(tag("ls"), |_| Command::Ls)(i)
    }

    fn parse_cd(i: &str) -> PResult<Command> {
        map(preceded(tag("cd "), parse_path), Command::Cd)(i) 
    }

    fn parse_cmd(i: &str) -> PResult<Command> {
        let (rest, _) = tag("$ ")(i)?;
        alt((parse_ls, parse_cd))(rest)
    }

    fn parse_entry(i: &str) -> PResult<Entry> {
        let parse_file = map(
            separated_pair(nom::character::complete::u64, tag(" "), parse_path),
            |(size, path)| Entry::File(size, path)
        );

        let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);

        alt((parse_file, parse_dir))(i)
    }

    fn parse_line(i: &str) -> PResult<Line> {
        alt((
            map(parse_entry, Line::Entry),
            map(parse_cmd, Line::Command)
        ))(i)
    }

    pub fn parse_input(i: &str) -> PResult<Vec<Line>> {
        many1(terminated(parse_line, opt(nom::character::complete::newline)))(i)
    }
}