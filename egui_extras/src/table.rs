#![allow(unused_variables, dead_code)]

/* TODO: Rethink the data structure here. Just a brainstorming: We need as a root a `Table` struct
* that has all settings and data, to display the container correctly or settings that should be
* global and passed down to childs.
*
* A table in the end is just a container, that displays a dynamic
* number of `TableRow`s. These rows need all the settings and data to display a row and its childs
* correctly. Each `TableRow` has a fixed numbers of `TableCell`s that display the data or Widget.
* The `TableRow` needs also a kind of struct (a vec or slice for example) to handle multi col cells
*
* Table:
*   - striped
*   - num cols
*   - heading
*   - vertical alignment
*   - future functionality:
*       - sortable
*       - resizable
*       - fixed cols
*       - paging
*       - max results per page
*
* TableRow:
*   - horizontal align
*   - collapsible
*
* TableCell:
*   - editable
*   - mark
*   - copyable
*
*/

enum VertAlignment {
    Left,
    Center,
    Right,
}

enum HorizAlignment {
    Top,
    Center,
    Bottom,
}

#[derive(Default)]
pub struct Table {
    num_columns: usize,
    striped: bool,
    display_headers: bool,
    col_alignment: Vec<VertAlignment>,

    /* future functionality: resizable: bool,
    * sortable_cols: Vec<usize>,
    * searchable: bool,
    * paging: bool,
    * max_results_per_page: usize,
    */
}

pub struct TableRow {
    cell_alignment: Vec<HorizAlignment>,
    collapsible: bool,
}

pub struct TableCell {
    editable: bool,
}
