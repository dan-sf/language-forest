use std::fs;
use std::io::Read;
use std::collections::HashMap;

fn init_b64_table() -> HashMap<u8, char> {
    let mut table: HashMap<u8, char> = HashMap::with_capacity(64);
    table.insert(0, 'A');  table.insert(16, 'Q'); table.insert(32, 'g'); table.insert(48, 'w');
    table.insert(1, 'B');  table.insert(17, 'R'); table.insert(33, 'h'); table.insert(49, 'x');
    table.insert(2, 'C');  table.insert(18, 'S'); table.insert(34, 'i'); table.insert(50, 'y');
    table.insert(3, 'D');  table.insert(19, 'T'); table.insert(35, 'j'); table.insert(51, 'z');
    table.insert(4, 'E');  table.insert(20, 'U'); table.insert(36, 'k'); table.insert(52, '0');
    table.insert(5, 'F');  table.insert(21, 'V'); table.insert(37, 'l'); table.insert(53, '1');
    table.insert(6, 'G');  table.insert(22, 'W'); table.insert(38, 'm'); table.insert(54, '2');
    table.insert(7, 'H');  table.insert(23, 'X'); table.insert(39, 'n'); table.insert(55, '3');
    table.insert(8, 'I');  table.insert(24, 'Y'); table.insert(40, 'o'); table.insert(56, '4');
    table.insert(9, 'J');  table.insert(25, 'Z'); table.insert(41, 'p'); table.insert(57, '5');
    table.insert(10, 'K'); table.insert(26, 'a'); table.insert(42, 'q'); table.insert(58, '6');
    table.insert(11, 'L'); table.insert(27, 'b'); table.insert(43, 'r'); table.insert(59, '7');
    table.insert(12, 'M'); table.insert(28, 'c'); table.insert(44, 's'); table.insert(60, '8');
    table.insert(13, 'N'); table.insert(29, 'd'); table.insert(45, 't'); table.insert(61, '9');
    table.insert(14, 'O'); table.insert(30, 'e'); table.insert(46, 'u'); table.insert(62, '+');
    table.insert(15, 'P'); table.insert(31, 'f'); table.insert(47, 'v'); table.insert(63, '/');
    table
}

fn main() {
    let b64_table = init_b64_table();
    let mut output = String::new();

    let test_file = fs::File::open("test.tmp").unwrap();
    let read_bytes = fs::read("test.tmp").unwrap();

    let mut b_iter = read_bytes.iter();

    let mut cur = b_iter.next().unwrap();

    let mut offset = 0;
    let mut count = 0;

    let mut first = 0 as u8;
    let mut left = 0 as u8;

    for byte in b_iter {
        let next = byte;
        let cmod = count % 3;
        let mut print_left = false;

        if cmod == 0 {
            first = cur>>2;
            left = (cur<<6)>>6;
        } else if cmod == 1 {
            first = cur>>4 ^ left<<4;
            left = (cur<<4)>>4;
        } else if cmod == 2 {
            first = cur>>6 ^ left<<2;
            left = (cur<<2)>>2;
            print_left = true;
        }
        // } else if cmod == 3 { // Without this block I miss every fourth char but get most of it, with it we get messed up after the 4th char
        //     first = left;
        // }

        let b64_ch = match b64_table.get(&first) {
            Some(ch) => ch,
            _ => &'%',
        };

        println!("b64_ch: {}, cur: {} -> {1:b}, next: {} -> {2:b}, first: {} -> {3:b}, left: {} -> {4:b}", b64_ch, cur, next, first, left);
        cur = next;
        count += 1;

        if print_left {
            let l = match b64_table.get(&left) {
                Some(ch) => ch,
                _ => &'%',
            };
            println!("this is left: {},",l);
        }

    }

    let cmod = count % 3;

    // if cmod == 0 {
    //     first = cur>>2;
    //     left = (cur<<6)>>6;
    // } else if cmod == 1 {
    //     first = cur>>4 ^ left<<4;
    //     left = (cur<<4)>>4;
    // } else if cmod == 2 {
    //     first = cur>>6 ^ left<<2;
    //     left = (cur<<2)>>2;
    // }
    first = left;

    let b64_ch = match b64_table.get(&first) {
        Some(ch) => ch,
        _ => &'%',
    };

    println!("b64_ch: {}, cur: {} -> {1:b}, first: {} -> {2:b}, left: {} -> {3:b}", b64_ch, cur, first, left);

    /*
    // ----

    let next = b_iter.next().unwrap();

    let first = cur>>2;
    let left = (cur<<6)>>6;
    let b64_ch = match b64_table.get(&first) {
        Some(ch) => ch,
        _ => &'%',
    };

    println!("b64_ch: {}, cur: {} -> {1:b}, first: {} -> {2:b}, left: {} -> {3:b}", b64_ch, cur, first, left);

    cur = next;

    // ----

    let next = b_iter.next().unwrap();

    let first = cur>>4 ^ left<<4;
    let left = (cur<<4)>>4;

    let b64_ch = match b64_table.get(&first) {
        Some(ch) => ch,
        _ => &'%',
    };

    println!("b64_ch: {}, cur: {} -> {1:b}, first: {} -> {2:b}, left: {} -> {3:b}", b64_ch, cur, first, left);

    cur = next;

    // ----

    let next = b_iter.next().unwrap();

    let first = cur>>6 ^ left<<2;
    let left = (cur<<2)>>2;

    let b64_ch = match b64_table.get(&first) {
        Some(ch) => ch,
        _ => &'%',
    };

    println!("b64_ch: {}, cur: {} -> {1:b}, first: {} -> {2:b}, left: {} -> {3:b}", b64_ch, cur, first, left);

    cur = next;

    // ----

    //let next = b_iter.next().unwrap(); // Not needed here

    let first = left;
    //let left = (cur<<2)>>2; // Not needed here

    let b64_ch = match b64_table.get(&first) {
        Some(ch) => ch,
        _ => &'%',
    };

    println!("b64_ch: {}, cur: {} -> {1:b}, first: {} -> {2:b}, left: {} -> {3:b}", b64_ch, cur, first, left);

    cur = next;
    */
}
