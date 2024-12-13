use crate::{
    grids::{GridlessGrid, HexagonalGrid, SquareGrid},
    traits::{BaseGrid, JsHelper},
    types::Point,
};

#[derive(Debug)]
pub enum Grid {
    Gridless(GridlessGrid),
    Square(SquareGrid),
    Hexagonal(HexagonalGrid),
}

impl Grid {
    pub fn size(&self) -> f64 {
        match self {
            Grid::Gridless(gridless_grid) => gridless_grid.size as f64,
            Grid::Square(square_grid) => square_grid.size as f64,
            Grid::Hexagonal(hexagonal_grid) => hexagonal_grid.size as f64,
        }
    }

    pub fn size_x(&self) -> f64 {
        match self {
            Grid::Gridless(gridless_grid) => gridless_grid.size as f64,
            Grid::Square(square_grid) => square_grid.size as f64,
            Grid::Hexagonal(hexagonal_grid) => hexagonal_grid.size_x,
        }
    }

    pub fn size_y(&self) -> f64 {
        match self {
            Grid::Gridless(gridless_grid) => gridless_grid.size as f64,
            Grid::Square(square_grid) => square_grid.size as f64,
            Grid::Hexagonal(hexagonal_grid) => hexagonal_grid.size_y,
        }
    }
}

impl crate::traits::JsDeserialize for Grid {
    fn from_js(data: impl wasm_bindgen::JsCast) -> Self {
        let value = data.as_ref();
        let r#type = value.get_value("type");

        match r#type {
            0 => Grid::Gridless(GridlessGrid { size: value.get_value("size") }),
            1 => Grid::Square(SquareGrid::new(value.get_value("size"))),
            2..=5 => Grid::Hexagonal(HexagonalGrid {
                size: value.get_value("size"),
                size_x: value.get_value("sizeX"),
                size_y: value.get_value("sizeY"),
                columns: value.get_value("columns"),
                even: value.get_value("even"),
            }),
            type_ => panic!("Unknown Grid Type - {type_}"),
        }
    }
}
