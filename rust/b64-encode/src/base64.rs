use std::io;
use std::fmt;
use std::error;
use std::collections::HashMap;


// Custom error which wraps both io errors and encoding errors in one type
pub enum EncodeError {
    Io(io::Error),
    Encode,
}

impl fmt::Debug for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EncodeError::Io(ref e) => e.fmt(f),
            EncodeError::Encode =>
                write!(f, "No data to encode"),
        }
    }
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EncodeError::Io(ref e) => e.fmt(f),
            EncodeError::Encode =>
                write!(f, "Encoding error"),
        }
    }
}

impl From<io::Error> for EncodeError {
    fn from(error: io::Error) -> Self {
        EncodeError::Io(error)
    }
}

impl error::Error for EncodeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            EncodeError::Encode => None,
            EncodeError::Io(ref e) => Some(e),
        }
    }
}

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

pub fn base_64_encode(bytes: Vec<u8>) -> Result<String, EncodeError> {
    let b64_table = init_b64_table();
    let mut output = String::new();

    let mut b_iter = bytes.iter();
    let mut cur = match b_iter.next() {
        Some(it) => it,
        None => {
            return Err(EncodeError::Encode);
        }
    };

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
    if cmod == 0 {
        first = cur>>2;
        left = (cur<<6)>>2;
        push_char(&mut output, &b64_table, first);
        push_char(&mut output, &b64_table, left);
        output.push('=');
        output.push('=');
    } else if cmod == 1 {
        first = cur>>4 ^ left<<4;
        left = (cur<<4)>>2;
        push_char(&mut output, &b64_table, first);
        push_char(&mut output, &b64_table, left);
        output.push('=');
    } else if cmod == 2 {
        first = cur>>6 ^ left<<2;
        left = (cur<<2)>>2;
        push_char(&mut output, &b64_table, first);
        push_char(&mut output, &b64_table, left);
    }

    Ok(output)
}

fn push_char(output: &mut String, table: &HashMap<u8, char>, val: u8) {
    let b64_ch = match table.get(&val) {
        Some(ch) => ch,
        _ => &'%',
    };
    output.push(*b64_ch);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_no_padding () {
        let actual = base_64_encode(String::from("Man").as_bytes().to_vec()).unwrap();
        let expected = String::from("TWFu");
        assert_eq!(actual, expected);
    }

    #[test]
    fn encode_single_padding () {
        let actual = base_64_encode(String::from("Ma").as_bytes().to_vec()).unwrap();
        let expected = String::from("TWE=");
        assert_eq!(actual, expected);
    }

    #[test]
    fn encode_double_padding () {
        let actual = base_64_encode(String::from("M").as_bytes().to_vec()).unwrap();
        let expected = String::from("TQ==");
        assert_eq!(actual, expected);
    }

    #[test]
    fn longer_encode_no_padding () {
        let actual = base_64_encode(String::from("foobar").as_bytes().to_vec()).unwrap();
        let expected = String::from("Zm9vYmFy");
        assert_eq!(actual, expected);
    }

    #[test]
    fn longer_encode_single_padding () {
        let actual = base_64_encode(String::from("fooba").as_bytes().to_vec()).unwrap();
        let expected = String::from("Zm9vYmE=");
        assert_eq!(actual, expected);
    }

    #[test]
    fn longer_encode_double_padding () {
        let actual = base_64_encode(String::from("foob").as_bytes().to_vec()).unwrap();
        let expected = String::from("Zm9vYg==");
        assert_eq!(actual, expected);
    }

    #[test]
    fn no_data_error() {
        let actual = base_64_encode(String::from("").as_bytes().to_vec());
        let expected = match actual {
            Err(_) => true,
            _ => false,
        };
        assert!(expected);
    }
}

