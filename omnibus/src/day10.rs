use std::collections::HashMap;

use wasm_bindgen::prelude::*;

crate::common_day!(day_10, Day10, u64, u64);

#[wasm_bindgen]
impl Day10 {
    pub fn get_distances(&self) -> Result<JsValue, JsValue> {
        self.0
            .get_distances()
            .map_err(|e| JsValue::from(e.to_string()))
            .map(|res| {
                serde_wasm_bindgen::to_value(
                    &res.into_iter()
                        // tuples become arrays, and array keys in maps in JS are fraught with peril
                        .map(|((x, y), v)| (format!("{x},{y}"), v))
                        .collect::<HashMap<_, _>>(),
                )
                .unwrap()
            })
    }
}
