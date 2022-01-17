
use rand::{thread_rng, Rng};

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
    let mut cell = &mut self.rows[0].cells[0];
    cell.state = 0;

    for x in 0..self.rows.len(){
        for y in 0..self.rows[x].cells.len() {
            let life_around = self.rows[x].cells[y].check(x, y, self);
            // this not work because is changing stage between checks, I should create a new table
            self.rows[x].cells[y].state = 0;
            if self.rows[x].cells[y].state == 0 {
                if life_around == 3 {
                    self.rows[x].cells[y].state = 1
                }
            }else{
                if life_around != 2 && life_around != 3 {
                    self.rows[x].cells[y].state = 0
                }
            }

        }
    }

 }

 fn print(&self){
    println!("");

    for ri in 0..self.rows.len() {
        for ci in 0..self.rows[ri].cells.len()   {
            if self.rows[ri].cells[ci].state == 1 {
                print!("😀");
            }else{
                print!("💩");
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
        for _ in 0..size {
            let is_alive = thread_rng().gen_bool(1.0 / 2.0);

            if is_alive {
                row.cells.push(Cell{state:1})
            }else{
                row.cells.push(Cell{state:0})
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
#[derive(Debug)]
struct Cell{
    state:i32
}

impl Cell{
    fn check(&self, x:usize,y:usize,table:&Table) -> i32{
        let size = table.rows.len()-1;
        
        let mut total_live = 0;

    
        if x > 0 && x < size && y > 0 && y < size {
            // internal cells
            total_live = total_live + table.rows[x-1].cells[y-1].state;
            total_live = total_live + table.rows[x-1].cells[y].state;
            total_live = total_live + table.rows[x-1].cells[y+1].state;
    
            total_live = total_live + table.rows[x+1].cells[y-1].state;
            total_live = total_live + table.rows[x+1].cells[y].state;
            total_live = total_live + table.rows[x+1].cells[y+1].state;
    
            total_live = total_live + table.rows[x].cells[y-1].state;
            total_live = total_live + table.rows[x].cells[y+1].state;

        }else {
            //external cells
            if x == 0 {
                if y == 0 {
                    total_live = total_live + table.rows[x+1].cells[y].state;
                    total_live = total_live + table.rows[x+1].cells[y+1].state;
                    total_live = total_live + table.rows[x].cells[y+1].state;
                }
                if y == size {
            
                    total_live = total_live + table.rows[x+1].cells[y-1].state;
                    total_live = total_live + table.rows[x+1].cells[y].state;
                    total_live = total_live + table.rows[x].cells[y-1].state;
                }
            }

            if x == size {
                if y == 0 {
                    total_live = total_live + table.rows[x-1].cells[y].state;
                    total_live = total_live + table.rows[x-1].cells[y+1].state;
                    total_live = total_live + table.rows[x].cells[y+1].state;
                }
                if y == size {
                    total_live = total_live + table.rows[x-1].cells[y-1].state;
                    total_live = total_live + table.rows[x-1].cells[y].state;
                    total_live = total_live + table.rows[x].cells[y-1].state;
                }
            }
        }

        total_live
        // STATE::DEAD
    }
}

fn clear_terminal (){
    std::process::Command::new("clear").status().unwrap();
}
fn main() {
    clear_terminal();
    println!("Hello, world!");

    let mut table = Table::new(60);
    table.print();
    table.next_day();
    table.print();

}
