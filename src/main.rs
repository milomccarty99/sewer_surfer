use std::io;
use std::io::{stdin, stdout};
use std::cmp;


const DISTANCE_SEEN: i32 = 100;
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
    // surface pop-out
    // color surface
    // resume sewer traversal
    // make sewer look like sewer
    // multiple level
    // boss for each level
    while pos_x < lvl1.length {
        let mut cursor = 0;
        let mut display: String = String::from("");
        let mut surface: String = {
            let mut pat: String = String::from("");
            for i in 0..cmp::min(DISTANCE_SEEN, lvl1.length - pos_x) {
                pat += "+"; 
            }
            pat
        };
        while cursor < DISTANCE_SEEN && cursor + pos_x < lvl1.length {
            let mut iswall: bool = true;
            for i in 0..5 {
                if lvl1.breaches[i] == cursor + pos_x {
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
        println!("{}[2J", 27 as char);
        println!("{surface}");
        println!("{display}");
        println!("{display}");
        println!("{display}");
        println!("{display}");
        println!("{display}");
        let mut input: String = String::from("");
        stdin().read_line(&mut input);
        // advance 1 space
        pos_x += 1;
    }


}
