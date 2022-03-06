#![allow(unused_variables, dead_code)]

use egui::{Response, Ui, Vec2, Widget, WidgetText};

/// WIP: table widget
///
#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
#[derive(Default)]
pub struct Table {
    num_columns: usize,
    fixed_cols: Option<Vec<usize>>,
    resizable: bool,
    sortable_cols: Option<Vec<usize>>,
    title: Option<WidgetText>,
    striped: bool,
    min_col_width: Option<f32>,
    min_row_height: Option<f32>,
    max_cell_size: Vec2,
    spacing: Vec2,
    searchable: bool,
}

impl Table {
    pub fn new(columns: usize) -> Self {
        Self {
            num_columns: columns,
            ..Default::default()
        }
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    pub fn title(mut self, title: impl Into<WidgetText>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn striped(mut self, striped: bool) -> Self {
        self.striped = striped;
        self
    }

    pub fn searchable(mut self, searchable: bool) -> Self {
        self.searchable = searchable;
        self
    }

    pub fn fixed_cols(mut self, fixed_cols: impl Into<Vec<usize>>) -> Self {
        self.fixed_cols = Some(fixed_cols.into());
        self
    }

    pub fn sortable_cols(mut self, sortable_cols: impl Into<Vec<usize>>) -> Self {
        self.sortable_cols = Some(sortable_cols.into());
        self
    }

    pub fn spacing(mut self, spacing: impl Into<Vec2>) -> Self {
        self.spacing = spacing.into();
        self
    }

    pub fn max_cell_size(mut self, max_cell_size: impl Into<Vec2>) -> Self {
        self.max_cell_size = max_cell_size.into();
        self
    }

    pub fn min_col_width(mut self, min_col_width: f32) -> Self {
        self.min_col_width = Some(min_col_width);
        self
    }

    pub fn min_row_height(mut self, min_row_height: f32) -> Self {
        self.min_row_height = Some(min_row_height);
        self
    }
}

impl Widget for Table {
    fn ui(self, ui: &mut Ui) -> Response {
        todo!()
    }
}
