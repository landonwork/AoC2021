#[derive(Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct Player {
    pub position: usize,
    pub score: usize,
}

impl Player {
    pub fn new(position: usize) -> Self {
        Player { position, score: 0 }
    }

    pub fn advance(&mut self, movement: usize) {
        self.position += movement;
        self.position %= 10;
        self.score += self.position + 1;
    }
}

pub struct DeterministicDie {
    value: usize,
    pub n_rolls: usize,
}

impl Default for DeterministicDie {
    fn default() -> Self {
        DeterministicDie { value: 1, n_rolls: 0 }
    }
}

impl DeterministicDie {
    pub fn roll(&mut self) -> usize {
        let result = self.value;
        self.value %= 100;
        self.value += 1;
        self.n_rolls += 1;
        result
    }
}
