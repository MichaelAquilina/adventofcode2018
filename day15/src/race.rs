#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]

pub enum Race {
    Goblin,
    Elf,
}

impl Race {
    pub fn enemy(&self) -> Race {
        match self {
            Race::Goblin => Race::Elf,
            Race::Elf => Race::Goblin,
        }
    }
}
