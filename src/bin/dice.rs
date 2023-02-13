
//enum SkillLevel {
    
//}

struct DicePoolD6 {
    level: u32,
    black: u32,
    white: u32,
    b_good: u32,
    b_bad:  u32,
    b_ugly: u32,
    w_good: u32,
    w_bad:  u32,
    w_ugly: u32,
}

impl DicePoolD6 {
    fn new_pool (level: u32) -> Self {
        DicePoolD6 {
            level: level,
            black: 5 - level,
            white: level,
            b_good: 0,
            b_bad: 0,
            b_ugly: 0,
            w_good: 0,
            w_bad: 0,
            w_ugly: 0,
        }
    }

    //fn roll_statistics (self, rolls: u32) {

    //}
}

fn main () {
    //let roll1: DicePoolD6 = DicePoolD6::new_roll_statistics(600, 1);
}