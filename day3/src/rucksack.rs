use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Item(u8);

impl Item {
    pub fn priority(&self) -> u8 {
        match self {
            Item(b'a'..=b'z') => 1 + (self.0 - b'a'),
            Item(b'A'..=b'Z') => 27 + (self.0 - b'A'),
            _ => unreachable!(),
        }
    }
}

impl TryFrom<u8> for Item {
    type Error = color_eyre::Report;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'a'..=b'z' | b'A'..=b'Z' => Ok(Item(value)),
            _ => Err(color_eyre::eyre::eyre!(
                "{} is not a valid item",
                value as char
            )),
        }
    }
}

impl std::fmt::Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0 as char)
    }
}

#[derive(Debug)]
pub struct Rucksack {
    front: HashSet<Item>,
    back: HashSet<Item>,
}

impl Rucksack {
    pub fn parse(s: &str) -> Self {
        let n = s.len();
        let n2 = n / 2;

        let convert = |s: &str| s.bytes().map(|c| Item(c)).collect::<HashSet<_>>();

        Self {
            front: convert(&s[0..n2]),
            back: convert(&s[n2..]),
        }
    }

    pub fn common_priority(&self) -> u8 {
        let common = self.front.intersection(&self.back).next().unwrap();
        common.priority()
    }

    pub fn contents(&self) -> HashSet<Item> {
        self.front.union(&self.back).into_iter().cloned().collect()
    }
}

pub struct ElfGroup(Vec<Rucksack>);

impl ElfGroup {
    pub fn common_element(&self) -> Item {
        let imed: HashSet<_> = self.0[0].contents().intersection(&self.0[1].contents()).copied().collect();
        *self.0[2].contents().intersection(&imed).next().unwrap()
    }
    pub fn common_priority(&self) -> u8 {
        self.common_element().priority()
    }
}

pub fn parse_groups(s: &str) -> Vec<ElfGroup> {
    let n_lines = s.lines().count();
    let lines: Vec<&str> = s.lines().collect();

    (0..n_lines).step_by(3)
        .map(|n| {
            let elf_lines: Vec<Rucksack> = lines[n..n+3]
                .iter()
                .map(|l| Rucksack::parse(l))
                .collect();
            ElfGroup(elf_lines)
        })
        .collect::<Vec<ElfGroup>>()
}
