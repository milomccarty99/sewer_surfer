use std::io;
use std::io::{stdin, stdout, Write};
use std::cmp;
use termion::color;

const DISTANCE_SEEN: i32 = 100;
//static mut surfaced: bool = false;
struct Tunnel {
    length: i32,
    seed: i32,
    boss: String,
    breaches: [i32; 5]
}

fn main() {
    println!("sewer surfer - made with kowabunga");
    let mut pos_x = 0;

    let lvl1: Tunnel = Tunnel{
        length: 500,
        seed: 0,
        boss: String::from("Pizza"),
        breaches: [5, 100, 200, 300, 415],
    };
    // to-do 
    // x surface pop-out
        // x color surface
    // x resume sewer traversal
    // x make sewer look like sewer
    //  multiple level
    //  boss for each level
    let mut surfaced: bool = true;
    while pos_x < lvl1.length {
        let mut input: String = String::from("");
        stdin().read_line(&mut input);
        // advance 1 space
        if (input == "\n") 
        {
            pos_x += 1;
        }
        surfaced = is_at_breach(pos_x, &lvl1);
        println!("{}[2J", 27 as char);
        print_surface(pos_x, surfaced);
        print_tunnel(pos_x, surfaced, &lvl1);
        println!("{pos_x}");
    }


}
fn is_at_breach(pos_x: i32, lvl: &Tunnel) -> bool {
    for i in 0..5 {if pos_x == lvl.breaches[i] {return true}}
    return false;
}
fn print_surface(pos_x: i32, surfaced: bool) {
    let mut display: String = String::from("");
    let player: char = '@';
    for i in 0..DISTANCE_SEEN {
        display += "+";
    }
    // one char at a time for coloring purposes
    //println!("{}", player);
    //println!("{display}");
    if surfaced {
        write!(stdout(), "{}", color::Fg(color::LightBlue));
        write!(stdout(), "@\n");
    }
    for i in 0..display.len() {

        match display.chars().nth(i) {
            Some('+') => {
                write!(stdout(), "{}", color::Fg(color::Green));
                write!(stdout(), "+");
            },
            Some('*') => {
                write!(stdout(), "{}", color::Fg(color::Yellow));
                write!(stdout(), "*");
            },
            _ => (),
        }
    }
    println!();
}

fn print_tunnel(pos_x: i32, surfaced: bool, lvl: &Tunnel) {
    let mut cursor = 0;
    let mut display: String = String::from("");
    let mut surface: String = {
        let mut pat: String = String::from("");
        for i in 0..cmp::min(DISTANCE_SEEN, lvl.length - pos_x) {
            pat += "+"; 
        }
        pat
       
    };
    while cursor < DISTANCE_SEEN && cursor + pos_x < lvl.length {
        let mut iswall: bool = true;
        for i in 0..5 {
            if lvl.breaches[i] == cursor + pos_x {
                display += "|  |";
                iswall = false;
                cursor += 3;
                break;
            }
        }
        if iswall {
            display += "-";
        }
        cursor += 1;
    }
        // output
        
        write!(stdout(), "{}", color::Fg(color::LightRed));
        println!("{surface}");

        write!(stdout(), "{}", color::Fg(color::Blue));
        println!("{display}");
        println!("{display}");
        println!("{display}");
        println!("{display}");
        println!("{display}");
        if !surfaced {
            println!("{}@",color::Fg(color::LightBlue));
        }
}
