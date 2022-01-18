use std::env;
use game_of_life2::game::Table;
use std::{thread, time};

fn clear_terminal (){
    std::process::Command::new("clear").status().unwrap();
}

fn sleep(milis:u64){
    let _time = time::Duration::from_millis(milis);
    thread::sleep(_time);
}

fn main() {
    
    let size = get_table_size();
    let speed = get_speed();

    clear_terminal();
    println!("Hello, world!");
    let mut gen = 0;
    let mut table = Table::new(size);
    
    loop {
        clear_terminal();
        println!("Generation: {}", gen);
        table.print();
        sleep(speed);
        let is_last_day = table.next_day();
        println!("is_last_day {}", is_last_day);
        if is_last_day {
            table.print();
            break;
        }
        gen = gen + 1;

    }

}

fn get_speed() -> u64{
    let args: Vec<String> = env::args().collect();
    
    let speed:String;
    if args.len() < 3 {
        speed = "1000".to_string();
    }else{
        speed = args[2].clone();
    }

    let speed:u64 = match speed.parse::<u64>() {
        Ok(num) => {
            if num >= 3 {
                num
            } else {
                5
            }
        },
        Err(_) => 10,
    };
    speed
}


fn get_table_size() -> usize{
    let args: Vec<String> = env::args().collect();
    
    let size:String;
    if args.len() < 2 {
        size = "3".to_string();
    }else{
        size = args[1].clone();
    }

    let size:usize = match size.parse::<usize>() {
        Ok(num) => {
            if num >= 3 {
                num
            } else {
                5
            }
        },
        Err(_) => 10,
    };
    size
}
