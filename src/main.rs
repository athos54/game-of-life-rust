
#[derive(Debug)]
#[warn(dead_code)]
struct Table {
    rows: Vec<Row>
}

impl Table {
 fn new(size:usize) -> Table{
    println!("Building new table with size {}",size);
    let mut rows:Vec<Row> = Vec::new();
    for _ in 0..size {
        rows.push(Row::new(size));
    }
    let table = Table{rows:rows};

    table
 }

 fn next_day(&mut self)  {
    // self
    let mut cell = &mut self.rows[0].cells[0];
    cell.state = STATE::DEAD;
    println!("next day {:#?}", &self.rows[0].cells[0].state)
 }

 fn print(&self){
    for ri in 0..self.rows.len() {
        for ci in 0..self.rows[ri].cells.len()   {
            match &self.rows[ri].cells[ci].state {
                STATE::ALIVE => print!("😀"),
                STATE::DEAD => print!("💩"),
            }
        }
        println!("")
    }
 }
}
#[derive(Debug)]
struct Row {
    cells: Vec<Cell>
}
impl Row {
    fn new(size:usize)->Row{
        let mut row = Row { cells: Vec::new() };
        for i in 0..size {
            if i==2{
                row.cells.push(Cell{state:STATE::DEAD})
            }else{
                row.cells.push(Cell{state:STATE::ALIVE})
            }
        }
        row
    }
}
#[derive(Debug)]
enum STATE{
    ALIVE,
    DEAD
}
#[derive(Debug)]
struct Cell{
    state:STATE
}

fn clear_terminal (){
    std::process::Command::new("clear").status().unwrap();
}
fn main() {
    clear_terminal();
    println!("Hello, world!");

    let mut table = Table::new(10);
    table.print();
    table.next_day();
    table.print();

}
