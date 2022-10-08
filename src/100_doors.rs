// The problem at hand is as follows: There are 100 doors in a row that are initially closed. 
// You make 100 passes by the doors. The first time through, you visit every door and toggle
// the door (if the door is closed, open it; if it is open, close it). The second time you
// only visit evern second door (Door #2, #4, #6...) and toggle it. The third time you only
// visit every third door (Door #3, #6, $9...), etc. You repeat this process until you only
// visit the 100th door


fn main() {
    let mut door_open = [false; 100];
    for pass in 1..101 {
        let mut door = pass;
        while door <= 100 {
            door_open[door - 1] = !door_open[door - 1];
            door += pass;
        }
    }

    for (i, &is_open) in door_open.iter().enumerate() {
        println!("Door {} is {}", i + 1, if is_open { "open" } else { "closed" });
    }
}

// Ultra optimized version:

// fn main() {
//    for i in 1u32..11u32 {
//        println!("Door {} is open", i.pow(2));
//    }
//}