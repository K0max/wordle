#[derive(PartialEq)]
pub enum LetterState {
    Correct,
    WrongSpot,
    Wrong,
}

#[derive(PartialEq)]
pub struct Letter {
    pub alph: char,
    pub state: LetterState,
}

impl Letter {
    pub fn new(alph: char, state: LetterState) -> Letter {
        Letter {
            alph,
            state,
        }
    }
}

// impl PartialEq for LetterState {
//     fn eq(&self, other: &Self) -> bool {
//         match self {
//             LetterState::Correct => {
//                 match other {
//                     LetterState::Correct => true,
//                     _ => false,
//                 }
//             }
//             LetterState::WrongSpot => {
//                 match other {
//                     LetterState::WrongSpot => true,
//                     _ => false,
//                 }
//             }
//             LetterState::Wrong => {
//                 match other {
//                     LetterState::Wrong => true,
//                     _ => false,
//                 }
//             }
//         }
//     }
// }

// impl PartialEq for Letter {
//     fn eq(&self, other: &Self) -> bool {
//         self.alph == other.alph
//     }
// }