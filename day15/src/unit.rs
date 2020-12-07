use crate::race::Race;

#[derive(Debug, PartialEq, Clone)]
pub struct Unit {
    pub hit_points: u32,
    pub attack_power: u32,
    pub race: Race,
}

impl Unit {
    pub fn new(race: Race) -> Unit {
        Unit {
            race,
            hit_points: 200,
            attack_power: 3,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.hit_points > 0
    }

    pub fn attack(&self, target: &mut Unit) {
        if self.attack_power > target.hit_points {
            target.hit_points = 0;
        } else {
            target.hit_points -= self.attack_power;
        }
    }

    pub fn to_char(&self) -> char {
        match self.race {
            Race::Goblin => 'G',
            Race::Elf => 'E',
        }
    }
}
