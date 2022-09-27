use chrono::{DateTime,Utc,FixedOffset, TimeZone};
const DIGITS : [[&str; 11]; 7] = [
    ["┏━┓ ","  ╦  "," ┏━┓ ", " ┏━┓ "," ╻ ╻ "," ┏━┓ "," ┏   "," ┏━┓ "," ┏━┓ "," ┏━┓ ","   "],
    [" ║ ║ ","   ║  ","     ║ ", "      ║ ","  ║ ║ ","  ║   ","    ║   ","     ║ ","  ║   ║ ","  ║  ║ "," ○ "],
    [" ║ ║ ","   ║  ","     ║ ", "      ║ ","  ║ ║ ","  ║   ","    ║   ","     ║ ","  ║   ║ ","  ║  ║ ","   "],
    [" ║ ║ ","   ║  "," ┏━┛ ", " ┣━┫ "," ┗━┫ "," ┗━┓ "," ┣━┓ ","   ║ ","  ┣━┫ "," ┗━┫","   "],
    [" ║ ║ ","   ║  ","  ║   ", "       ║ ","    ║ ","      ║ ","  ║   ║ ","   ║ ","  ║   ║ ","     ║ ","   "],
    [" ║ ║ ","   ║  ","  ║   ", "       ║ ","    ║ ","      ║ ","  ║   ║ ","   ║ ","  ║   ║ ","     ║ "," ○ "],
    ["┗━┛ ","  ╩  "," ┗━━ ", " ┗━┛ ","   ╹ ","  ┗━┛ "," ┗━┛ ","   ╹ "," ┗━┛ "," ┗━┛ ","   "],
  
];



fn main() {
   print!("\x1b[2J"); //removing the compiling messages
   print!("\x1b[?25l");
  let datetime1 = Utc::now();
  //println!("{}", Utc.timezone());
  
  loop{
    let t = Utc::now();
  
  // println!("{:?}", t);
    
   let time = t.format("%H:%M:%S").to_string();
     for row in &DIGITS {
           for c in time.chars() {
                let col = match c {
                    '0'..='9' => c as usize - '0' as usize,
                    ':' => 10,
                    _ => 10,
                };
                print!("{} ", row[col]);
            }
            
      // print!("{:?} ",time.chars());
            println!();
        }
   std::thread::sleep(std::time::Duration::from_millis(999));
     print!("\x1b[7A");  // clearing the console (previous output)

    
  }
   
}
