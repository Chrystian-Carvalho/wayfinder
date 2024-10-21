#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum WallMovementType {
    None = 0,
    Normal = 20,
}

impl crate::traits::JsDeserialize for WallMovementType {
    fn from_value(value: wasm_bindgen::JsValue) -> Self {
        let value = i32::from_value(value);

        match value {
            0 => WallMovementType::None,
            20 => WallMovementType::Normal,
            _ => panic!("Unknown Wall Movement Type - {value}"),
        }
    }
}