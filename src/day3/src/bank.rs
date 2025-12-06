pub struct Bank {
    pub number_str : String
}

pub struct MaxJolt {
    pub first : i32,
    pub second : i32
}

impl Default for MaxJolt {
    fn default() -> Self {
        MaxJolt { first: 0, second: 0 }
    }
}
impl MaxJolt {
    pub fn sum(&self) -> i32 {
        format!("{}{}", self.first, self.second).parse::<i32>().unwrap_or(0)
    }
}

impl Bank {
    pub fn compute_max_jolt(&self) -> i32 {
        self.number_str
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .enumerate()
            .fold(MaxJolt::default(), |mut acc, (_idx, num)| {
                let is_not_last_index = _idx != self.number_str.len() - 1;
                if num > acc.first && is_not_last_index {
                    acc.first = num;
                    acc.second = 0;
                } else if num > acc.second {
                    acc.second = num
                }
                acc
            })
            .sum()
    }
}