use std::{thread, time};
use rand::{thread_rng, Rng};

#[derive(Debug,Clone)]
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
    let mut table2 = self.clone();
    for x in 0..self.rows.len(){
        for y in 0..self.rows[x].cells.len() {
            let life_around = self.rows[x].cells[y].check(x, y, self);
            // this not work because is changing stage between checks, I should create a new table
            // self.rows[x].cells[y].state = 0;
            if self.rows[x].cells[y].state == 0 {
                if life_around == 3 {
                    table2.rows[x].cells[y].state = 1
                }else{
                    table2.rows[x].cells[y].state = 0
                }
            }else{
                if life_around != 2 && life_around != 3 {
                    table2.rows[x].cells[y].state = 0
                }else{
                    table2.rows[x].cells[y].state = 1
                }
            }

        }
    }

    self.rows = table2.rows

 }

 fn print(&self){
    println!("");

    for ri in 0..self.rows.len() {
        for ci in 0..self.rows[ri].cells.len()   {
            if self.rows[ri].cells[ci].state == 1 {
                // print!("1 ");
                print!("😀");
            }else{
                // print!("0 ");
                print!("💩");
            }
        }
        println!("")
    }
 }

}
#[derive(Debug,Clone)]
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
#[derive(Debug,Clone)]
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
                if y > 0 && y < size {
                    total_live = total_live + table.rows[x+1].cells[y-1].state;
                    total_live = total_live + table.rows[x+1].cells[y].state;
                    total_live = total_live + table.rows[x+1].cells[y+1].state;
            
                    total_live = total_live + table.rows[x].cells[y-1].state;
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
                if y > 0 && y < size {
                    total_live = total_live + table.rows[x-1].cells[y-1].state;
                    total_live = total_live + table.rows[x-1].cells[y].state;
                    total_live = total_live + table.rows[x-1].cells[y+1].state;
            
                    total_live = total_live + table.rows[x].cells[y-1].state;
                    total_live = total_live + table.rows[x].cells[y+1].state;
                }
                if y == size {
                    total_live = total_live + table.rows[x-1].cells[y-1].state;
                    total_live = total_live + table.rows[x-1].cells[y].state;
                    total_live = total_live + table.rows[x].cells[y-1].state;
                }
            }

            if x > 0 && x < size {
                if y == 0 {
                    total_live = total_live + table.rows[x-1].cells[y].state;
                    total_live = total_live + table.rows[x-1].cells[y+1].state;
            
                    total_live = total_live + table.rows[x+1].cells[y].state;
                    total_live = total_live + table.rows[x+1].cells[y+1].state;
            
                    total_live = total_live + table.rows[x].cells[y+1].state;
                }
                if y == size {
                    total_live = total_live + table.rows[x-1].cells[y-1].state;
                    total_live = total_live + table.rows[x-1].cells[y].state;
            
                    total_live = total_live + table.rows[x+1].cells[y-1].state;
                    total_live = total_live + table.rows[x+1].cells[y].state;
            
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

fn sleep(milis:u64){
    let _time = time::Duration::from_millis(milis);
    thread::sleep(_time);
}
fn main() {
    clear_terminal();
    println!("Hello, world!");
    let mut gen = 0;
    let mut table = Table::new(50);
    
    loop {
        clear_terminal();
        println!("Generation: {}", gen);
        table.print();
        sleep(500);
        table.next_day();
        gen=gen+1;
    }

}
