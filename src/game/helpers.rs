use crate::game::Table;
pub fn get_safe_coords(x: usize, xop: char, y: usize, yop: char, table: &Table) -> (usize, usize) {
    let xfinal: usize;
    let yfinal: usize;

    if xop == '+' {
        if x + 1 < table.rows.len() {
            xfinal = x + 1;
        } else {
            return (usize::MAX, usize::MAX);
        }
    } else if xop == '-' {
        if x > 0 {
            xfinal = x - 1;
        } else {
            return (usize::MAX, usize::MAX);
        }
    } else {
        xfinal = x
    }

    if yop == '+' {
        if y + 1 < table.rows[xfinal].cells.len() {
            yfinal = y + 1;
        } else {
            yfinal = usize::MAX;
        }
    } else if yop == '-' {
        if y > 0 {
            yfinal = y - 1;
        } else {
            yfinal = usize::MAX;
        }
    } else {
        yfinal = y
    }
    (xfinal, yfinal)
}

pub fn get_cell_status(x: usize, xop: char, y: usize, yop: char, table: &Table) -> i32 {
    let (xfinal, yfinal) = get_safe_coords(x, xop, y, yop, table);

    // get_cell_status_secure(xfinal, yfinal, table)
    match table.rows.get(xfinal) {
        None => 0,
        Some(row) => match row.cells.get(yfinal) {
            None => 0,
            Some(cell) => cell.state,
        },
    }
}
