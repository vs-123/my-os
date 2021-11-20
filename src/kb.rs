use crate::screen::Screen;
use crate::system::inportb;

pub struct KeyboardInput {}

impl KeyboardInput {
    pub fn read_line<'a>(mut screen: Screen) -> [u8; 256] {
        let mut buffer: u8 = '\x00' as u8;
        let mut buffer_string: [u8; 256] = [0x00; 256];
        let mut i = 0;
        let mut reading = true;

        while reading {
            if inportb(0x64) & 0x1 == 1 {
                match inportb(0x60) {
                    2 => {
                        buffer_string[i] = '1' as u8;
                        screen.print_char('1');
                        i += 1;
                    }

                    3 => {
                        buffer_string[i] = '2' as u8;
                        screen.print_char('2');
                        i += 1;
                    }

                    4 => {
                        buffer_string[i] = '3' as u8;
                        screen.print_char('3');
                        i += 1;
                    }

                    5 => {
                        buffer_string[i] = '4' as u8;
                        screen.print_char('4');
                        i += 1;
                    }

                    6 => {
                        buffer_string[i] = '5' as u8;
                        screen.print_char('5');
                        i += 1;
                    }

                    7 => {
                        buffer_string[i] = '6' as u8;
                        screen.print_char('6');
                        i += 1;
                    }

                    8 => {
                        buffer_string[i] = '7' as u8;
                        screen.print_char('7');
                        i += 1;
                    }

                    9 => {
                        buffer_string[i] = '8' as u8;
                        screen.print_char('8');
                        i += 1;
                    }

                    10 => {
                        buffer_string[i] = '9' as u8;
                        screen.print_char('9');
                        i += 1;
                    }

                    11 => {
                        buffer_string[i] = '0' as u8;
                        screen.print_char('0');
                        i += 1;
                    }

                    12 => {
                        buffer_string[i] = '-' as u8;
                        screen.print_char('-');
                        i += 1;
                    }

                    13 => {
                        buffer_string[i] = '=' as u8;
                        screen.print_char('=');
                        i += 1;
                    }

                    14 => {
                        buffer_string[i] = ' ' as u8;
                        screen.print_char('\x08');
                        if i > 0 {
                            i -= 1;
                        }
                    }

                    15 => {
                        buffer_string[i] = '\t' as u8;
                        screen.print_char('\t');
                        i += 1;
                    }

                    16 => {
                        buffer_string[i] = 'q' as u8;
                        screen.print_char('q');
                        i += 1;
                    }

                    17 => {
                        buffer_string[i] = 'w' as u8;
                        screen.print_char('w');
                        i += 1;
                    }

                    18 => {
                        buffer_string[i] = 'e' as u8;
                        screen.print_char('e');
                        i += 1;
                    }

                    19 => {
                        buffer_string[i] = 'r' as u8;
                        screen.print_char('r');
                        i += 1;
                    }

                    20 => {
                        buffer_string[i] = 't' as u8;
                        screen.print_char('t');
                        i += 1;
                    }

                    21 => {
                        buffer_string[i] = 'y' as u8;
                        screen.print_char('y');
                        i += 1;
                    }

                    22 => {
                        buffer_string[i] = 'u' as u8;
                        screen.print_char('u');
                        i += 1;
                    }

                    23 => {
                        buffer_string[i] = 'i' as u8;
                        screen.print_char('i');
                        i += 1;
                    }

                    24 => {
                        buffer_string[i] = 'o' as u8;
                        screen.print_char('o');
                        i += 1;
                    }

                    25 => {
                        buffer_string[i] = 'p' as u8;
                        screen.print_char('p');
                        i += 1;
                    }

                    26 => {
                        buffer_string[i] = '[' as u8;
                        screen.print_char('[');
                        i += 1;
                    }

                    27 => {
                        buffer_string[i] = ']' as u8;
                        screen.print_char(']');
                        i += 1;
                    }

                    28 => {
                        screen.print_char('\n');
                        break;
                    }

                    30 => {
                        buffer_string[i] = 'a' as u8;
                        screen.print_char('a');
                        i += 1;
                    }

                    31 => {
                        buffer_string[i] = 's' as u8;
                        screen.print_char('s');
                        i += 1;
                    }

                    32 => {
                        buffer_string[i] = 'd' as u8;
                        screen.print_char('d');
                        i += 1;
                    }

                    33 => {
                        buffer_string[i] = 'f' as u8;
                        screen.print_char('f');
                        i += 1;
                    }

                    34 => {
                        buffer_string[i] = 'g' as u8;
                        screen.print_char('g');
                        i += 1;
                    }

                    35 => {
                        buffer_string[i] = 'h' as u8;
                        screen.print_char('h');
                        i += 1;
                    }

                    36 => {
                        buffer_string[i] = 'j' as u8;
                        screen.print_char('j');
                        i += 1;
                    }

                    37 => {
                        buffer_string[i] = 'k' as u8;
                        screen.print_char('k');
                        i += 1;
                    }

                    38 => {
                        buffer_string[i] = 'l' as u8;
                        screen.print_char('l');
                        i += 1;
                    }

                    39 => {
                        buffer_string[i] = ';' as u8;
                        screen.print_char(';');
                        i += 1;
                    }

                    40 => {
                        buffer_string[i] = '\'' as u8;
                        screen.print_char('\'');
                        i += 1;
                    }

                    41 => {
                        buffer_string[i] = '`' as u8;
                        screen.print_char('`');
                        i += 1;
                    }

                    // 42 => {
                    //     buffer_string[i] = '\\' as u8;
                    //     screen.print_char('\\');
                    //     i += 1;
                    // }
                    44 => {
                        buffer_string[i] = 'z' as u8;
                        screen.print_char('z');
                        i += 1;
                    }

                    45 => {
                        buffer_string[i] = 'x' as u8;
                        screen.print_char('x');
                        i += 1;
                    }

                    46 => {
                        buffer_string[i] = 'c' as u8;
                        screen.print_char('c');
                        i += 1;
                    }

                    47 => {
                        buffer_string[i] = 'v' as u8;
                        screen.print_char('v');
                        i += 1;
                    }

                    48 => {
                        buffer_string[i] = 'b' as u8;
                        screen.print_char('b');
                        i += 1;
                    }

                    49 => {
                        buffer_string[i] = 'n' as u8;
                        screen.print_char('n');
                        i += 1;
                    }

                    50 => {
                        buffer_string[i] = 'm' as u8;
                        screen.print_char('m');
                        i += 1;
                    }

                    51 => {
                        buffer_string[i] = ',' as u8;
                        screen.print_char(',');
                        i += 1;
                    }

                    52 => {
                        buffer_string[i] = '.' as u8;
                        screen.print_char('.');
                        i += 1;
                    }

                    53 => {
                        buffer_string[i] = '/' as u8;
                        screen.print_char('/');
                        i += 1;
                    }

                    57 => {
                        buffer_string[i] = ' ' as u8;
                        screen.print_char(' ');
                        i += 1;
                    }

                    _ => {}
                }
            }
        }

        buffer_string[i] = '\x00' as u8;

        // (core::str::from_utf8(&buffer_string).unwrap())

        buffer_string
    }
}

#[macro_export]
macro_rules! keyboard_input {
    ($screen:ident, $to_store:ident) => {
        let $to_store = crate::kb::KeyboardInput::read_line($screen.clone());
        let $to_store = core::str::from_utf8(&$to_store).unwrap();
    };
}
