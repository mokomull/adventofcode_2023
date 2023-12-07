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

            pub fn part1(&self) -> Result<$part1_result, JsValue> {
                self.0.part1().map_err(|e| JsValue::from(e.to_string()))
            }

            pub fn part2(&self) -> Result<$part2_result, JsValue> {
                self.0.part2().map_err(|e| JsValue::from(e.to_string()))
            }
        }
    };
}

common_day!(day_01, Day01, u64, u64);
common_day!(day_02, Day02, u64, u64);
common_day!(day_03, Day03, u64, u64);
common_day!(day_04, Day04, u64, u64);
common_day!(day_05, Day05, u64, u64);
common_day!(day_06, Day06, u64, u64);
common_day!(day_07, Day07, u64, u64);
