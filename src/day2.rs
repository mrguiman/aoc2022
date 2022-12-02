#[derive(Eq, PartialEq)]
enum JankenShape {
    Rock,
    Paper,
    Scissors,
}
#[derive(Eq, PartialEq)]
enum JankenResult {
    LOSS,
    DRAW,
    WIN,
}
impl JankenResult {
    pub fn to_score(&self) -> i32 {
        match self {
            JankenResult::LOSS => 0,
            JankenResult::DRAW => 3,
            JankenResult::WIN => 6,
        }
    }
    pub fn from_str(value: &str) -> JankenResult {
        match value {
            "X" => JankenResult::LOSS,
            "Y" => JankenResult::DRAW,
            "Z" => JankenResult::WIN,
            _ => JankenResult::DRAW,
        }
    }
}
impl JankenShape {
    pub fn get_round_score(&self, opponent: &JankenShape) -> i32 {
        self.to_score() as i32 + self.get_result(opponent).to_score() as i32
    }

    fn to_score(&self) -> i32 {
        match self {
            JankenShape::Rock => 1,
            JankenShape::Paper => 2,
            JankenShape::Scissors => 3,
        }
    }
    fn get_result(&self, opponent: &JankenShape) -> JankenResult {
        match (self, opponent) {
            (&JankenShape::Rock, &JankenShape::Paper) => JankenResult::LOSS,
            (&JankenShape::Rock, &JankenShape::Scissors) => JankenResult::WIN,
            (&JankenShape::Paper, &JankenShape::Scissors) => JankenResult::LOSS,
            (&JankenShape::Paper, &JankenShape::Rock) => JankenResult::WIN,
            (&JankenShape::Scissors, &JankenShape::Rock) => JankenResult::LOSS,
            (&JankenShape::Scissors, &JankenShape::Paper) => JankenResult::WIN,
            _ => JankenResult::DRAW,
        }
    }
    pub fn from_str(value: &str) -> JankenShape {
        match value {
            "A" | "X" => JankenShape::Rock,
            "B" | "Y" => JankenShape::Paper,
            "C" | "Z" => JankenShape::Scissors,
            _ => JankenShape::Rock,
        }
    }
    pub fn from_expected_result(
        opponent: &JankenShape,
        expected_result: &JankenResult,
    ) -> JankenShape {
        for shape in vec![JankenShape::Rock, JankenShape::Paper, JankenShape::Scissors] {
            if &shape.get_result(opponent) == expected_result {
                return shape;
            }
        }
        return JankenShape::Rock;
    }
}

pub fn get_part1_answer(input: &String) -> i32 {
    input
        .split('\n')
        .map(|round_str| {
            let shapes_strs = round_str.split_whitespace().collect::<Vec<&str>>();
            JankenShape::from_str(shapes_strs[1])
                .get_round_score(&JankenShape::from_str(shapes_strs[0]))
        })
        .sum::<i32>()
}
pub fn get_part2_answer(input: &String) -> i32 {
    input
        .split('\n')
        .map(|round_str| {
            let shapes_strs = round_str.split_whitespace().collect::<Vec<&str>>();
            let expected_result = JankenResult::from_str(shapes_strs[1]);
            let expected_shape = JankenShape::from_expected_result(
                &JankenShape::from_str(shapes_strs[0]),
                &expected_result,
            );

            expected_result.to_score() + expected_shape.to_score()
        })
        .sum::<i32>()
}
