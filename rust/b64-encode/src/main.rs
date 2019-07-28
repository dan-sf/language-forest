use std::fs;
use std::env;
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

fn push_char(output: &mut String, table: &HashMap<u8, char>, val: u8) {
    let b64_ch = match table.get(&val) {
        Some(ch) => ch,
        _ => &'%',
    };
    output.push(*b64_ch);
}

fn main() {

    // @TODO: Enable specifiing the file with command line args: -i <file> -o <file> for inputfile and output files, default to using stdin/stdout
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    let b64_table = init_b64_table();
    let mut output = String::new();

    let read_bytes = fs::read("test.tmp").unwrap();
    let mut b_iter = read_bytes.iter();
    let mut cur = b_iter.next().unwrap();

    let mut count = 0;
    let mut first: u8 = 0;
    let mut left: u8 = 0;

    for byte in b_iter {
        let next = byte;
        let cmod = count % 3;

        if cmod == 0 {
            first = cur>>2;
            left = (cur<<6)>>6;
            push_char(&mut output, &b64_table, first);
        } else if cmod == 1 {
            first = cur>>4 ^ left<<4;
            left = (cur<<4)>>4;
            push_char(&mut output, &b64_table, first);
        } else if cmod == 2 {
            first = cur>>6 ^ left<<2;
            left = (cur<<2)>>2;
            push_char(&mut output, &b64_table, first);
            push_char(&mut output, &b64_table, left);
        }

        cur = next;
        count += 1;
    }

    let cmod = count % 3;

    // @TODO: Validate the operations for the changes to left below
    if cmod == 0 {
        first = cur>>2;
        left = (cur<<6)>>2; // @Note: not sure if this is right ...
        push_char(&mut output, &b64_table, first);
        push_char(&mut output, &b64_table, left);
        output.push('=');
        output.push('=');
    } else if cmod == 1 {
        first = cur>>4 ^ left<<4;
        left = (cur<<4)>>2; // @Note: This may be incorrect, we might want (cur<<2)>>2 ...
        push_char(&mut output, &b64_table, first);
        push_char(&mut output, &b64_table, left);
        output.push('=');
    } else if cmod == 2 {
        first = cur>>6 ^ left<<2;
        left = (cur<<2)>>2;
        push_char(&mut output, &b64_table, first);
        push_char(&mut output, &b64_table, left);
    }

    println!("{}", output);
}
