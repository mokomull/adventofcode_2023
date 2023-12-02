use wasm_bindgen::prelude::*;

macro_rules! common_day {
    ($krate: ident, $strukt: ident, $part1_result: ty, $part2_result: ty) => {
        #[wasm_bindgen]
        pub struct $strukt($krate::Solution);

        #[wasm_bindgen]
        impl $strukt {
            pub fn new(input: &str) -> Self {
                Self($krate::Solution::new(input))
            }

            pub fn part1(&self) -> $part1_result {
                self.0.part1()
            }

            pub fn part2(&self) -> $part2_result {
                self.0.part2()
            }
        }
    };
}

common_day!(day_01, Day01, u64, u64);
