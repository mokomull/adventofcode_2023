use wasm_bindgen::prelude::*;

mod day10;

macro_rules! common_day {
    ($krate: ident, $strukt: ident, $part1_result: ty, $part2_result: ty) => {
        #[wasm_bindgen]
        pub struct $strukt($krate::Solution);

        #[wasm_bindgen]
        impl $strukt {
            pub fn new(input: &str) -> Self {
                Self(<$krate::Solution as prelude::Day>::new(input))
            }

            pub fn part1(&self) -> Result<$part1_result, JsValue> {
                prelude::Day::part1(&self.0).map_err(|e| JsValue::from(e.to_string()))
            }

            pub fn part2(&self) -> Result<$part2_result, JsValue> {
                prelude::Day::part2(&self.0).map_err(|e| JsValue::from(e.to_string()))
            }
        }
    };
}

use common_day; // let submodule use it

common_day!(day_01, Day01, u64, u64);
common_day!(day_02, Day02, u64, u64);
common_day!(day_03, Day03, u64, u64);
common_day!(day_04, Day04, u64, u64);
common_day!(day_05, Day05, u64, u64);
common_day!(day_06, Day06, u64, u64);
common_day!(day_07, Day07, u64, u64);
common_day!(day_08, Day08, u64, u64);
common_day!(day_09, Day09, u64, u64);
common_day!(day_11, Day11, u64, u64);
common_day!(day_12, Day12, u64, u64);
common_day!(day_14, Day14, u64, u64);
common_day!(day_15, Day15, u64, u64);
common_day!(day_16, Day16, u64, u64);
common_day!(day_17, Day17, u64, u64);
common_day!(day_18, Day18, u64, u64);

#[wasm_bindgen(start)]
pub fn start() {
    prelude::init();
}
