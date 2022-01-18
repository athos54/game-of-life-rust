pub mod game {
    use rand::{thread_rng, Rng};

    #[derive(Debug, Clone)]
    #[warn(dead_code)]
    pub struct Table {
        pub rows: Vec<Row>,
    }

    impl Table {
        pub fn new(size: usize) -> Table {
            println!("Building new table with size {}", size);
            let mut rows: Vec<Row> = Vec::new();
            for _ in 0..size {
                rows.push(Row::new(size));
            }
            let table = Table { rows: rows };

            table
        }

        pub fn next_day(&mut self) -> bool {
            let mut table2 = self.clone();
            let mut is_some_cell_alive = false;
            for x in 0..self.rows.len() {
                for y in 0..self.rows[x].cells.len() {
                    let life_around = self.rows[x].cells[y].check(x, y, self);
                    // this not work because is changing stage between checks, I should create a new table
                    // self.rows[x].cells[y].state = 0;
                    if self.rows[x].cells[y].state == 0 {
                        if life_around == 3 {
                            is_some_cell_alive = true;
                            table2.rows[x].cells[y].state = 1
                        } else {
                            table2.rows[x].cells[y].state = 0
                        }
                    } else {
                        if life_around != 2 && life_around != 3 {
                            table2.rows[x].cells[y].state = 0
                        } else {
                            is_some_cell_alive = true;
                            table2.rows[x].cells[y].state = 1
                        }
                    }
                }
            }

            self.rows = table2.rows;
            !is_some_cell_alive
        }

        pub fn print(&self) {
            println!("");

            for ri in 0..self.rows.len() {
                for ci in 0..self.rows[ri].cells.len() {
                    if self.rows[ri].cells[ci].state == 1 {
                        print!("😀");
                    } else {
                        print!("💩");
                    }
                }
                println!("")
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct Row {
        pub cells: Vec<Cell>,
    }
    impl Row {
        fn new(size: usize) -> Row {
            let mut row = Row { cells: Vec::new() };
            for _ in 0..size {
                let is_alive = thread_rng().gen_bool(1.0 / 2.0);

                if is_alive {
                    row.cells.push(Cell { state: 1 })
                } else {
                    row.cells.push(Cell { state: 0 })
                }
            }
            row
        }
    }
    // #[derive(Debug)]
    // enum STATE{
    //     ALIVE=1,
    //     DEAD=0
    // }
    #[derive(Debug, Clone)]
    pub struct Cell {
        pub state: i32,
    }

    impl Cell {
        fn check(&self, x: usize, y: usize, table: &Table) -> i32 {
            let size = table.rows.len() - 1;
            let mut total_live = 0;

            if x > 0 && x < size && y > 0 && y < size {
                // internal cells
                total_live = total_live + table.rows[x - 1].cells[y - 1].state;
                total_live = total_live + table.rows[x - 1].cells[y].state;
                total_live = total_live + table.rows[x - 1].cells[y + 1].state;

                total_live = total_live + table.rows[x + 1].cells[y - 1].state;
                total_live = total_live + table.rows[x + 1].cells[y].state;
                total_live = total_live + table.rows[x + 1].cells[y + 1].state;

                total_live = total_live + table.rows[x].cells[y - 1].state;
                total_live = total_live + table.rows[x].cells[y + 1].state;
            } else {
                //external cells
                if x == 0 {
                    if y == 0 {
                        total_live = total_live + table.rows[x + 1].cells[y].state;
                        total_live = total_live + table.rows[x + 1].cells[y + 1].state;
                        total_live = total_live + table.rows[x].cells[y + 1].state;
                    }
                    if y > 0 && y < size {
                        total_live = total_live + table.rows[x + 1].cells[y - 1].state;
                        total_live = total_live + table.rows[x + 1].cells[y].state;
                        total_live = total_live + table.rows[x + 1].cells[y + 1].state;

                        total_live = total_live + table.rows[x].cells[y - 1].state;
                        total_live = total_live + table.rows[x].cells[y + 1].state;
                    }
                    if y == size {
                        total_live = total_live + table.rows[x + 1].cells[y - 1].state;
                        total_live = total_live + table.rows[x + 1].cells[y].state;
                        total_live = total_live + table.rows[x].cells[y - 1].state;
                    }
                }

                if x == size {
                    if y == 0 {
                        total_live = total_live + table.rows[x - 1].cells[y].state;
                        total_live = total_live + table.rows[x - 1].cells[y + 1].state;
                        total_live = total_live + table.rows[x].cells[y + 1].state;
                    }
                    if y > 0 && y < size {
                        total_live = total_live + table.rows[x - 1].cells[y - 1].state;
                        total_live = total_live + table.rows[x - 1].cells[y].state;
                        total_live = total_live + table.rows[x - 1].cells[y + 1].state;

                        total_live = total_live + table.rows[x].cells[y - 1].state;
                        total_live = total_live + table.rows[x].cells[y + 1].state;
                    }
                    if y == size {
                        total_live = total_live + table.rows[x - 1].cells[y - 1].state;
                        total_live = total_live + table.rows[x - 1].cells[y].state;
                        total_live = total_live + table.rows[x].cells[y - 1].state;
                    }
                }

                if x > 0 && x < size {
                    if y == 0 {
                        total_live = total_live + table.rows[x - 1].cells[y].state;
                        total_live = total_live + table.rows[x - 1].cells[y + 1].state;

                        total_live = total_live + table.rows[x + 1].cells[y].state;
                        total_live = total_live + table.rows[x + 1].cells[y + 1].state;

                        total_live = total_live + table.rows[x].cells[y + 1].state;
                    }
                    if y == size {
                        total_live = total_live + table.rows[x - 1].cells[y - 1].state;
                        total_live = total_live + table.rows[x - 1].cells[y].state;

                        total_live = total_live + table.rows[x + 1].cells[y - 1].state;
                        total_live = total_live + table.rows[x + 1].cells[y].state;

                        total_live = total_live + table.rows[x].cells[y - 1].state;
                    }
                }
            }

            total_live
            // STATE::DEAD
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::Table;

    #[test]
    fn size_should_be_the_same_between_rows_and_columns() {
        let size = 3;
        let table = Table::new(size);
        assert_eq!(table.rows[0].cells.len(), size);
        assert_eq!(table.rows.len(), size);
    }

    #[test]
    fn should_calculate_next_day_correctly() {
        let size = 3;
        let mut table = Table::new(size);

        table.rows[0].cells[0].state = 0;
        table.rows[0].cells[1].state = 0;
        table.rows[0].cells[2].state = 1;

        table.rows[1].cells[0].state = 1;
        table.rows[1].cells[1].state = 0;
        table.rows[1].cells[2].state = 0;

        table.rows[2].cells[0].state = 1;
        table.rows[2].cells[1].state = 0;
        table.rows[2].cells[2].state = 0;

        table.next_day();

        assert_eq!(table.rows[0].cells[0].state, 0);
        assert_eq!(table.rows[0].cells[1].state, 0);
        assert_eq!(table.rows[0].cells[2].state, 0);

        assert_eq!(table.rows[1].cells[0].state, 0);
        assert_eq!(table.rows[1].cells[1].state, 1);
        assert_eq!(table.rows[1].cells[2].state, 0);

        assert_eq!(table.rows[2].cells[0].state, 0);
        assert_eq!(table.rows[2].cells[1].state, 0);
        assert_eq!(table.rows[2].cells[2].state, 0);
    }
}
