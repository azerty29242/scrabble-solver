use libc::{ioctl, STDOUT_FILENO, TIOCGWINSZ};
use std::io::{stdin, stdout, Write};

#[derive(Debug)]
#[repr(C)]
pub struct WindowSize {
    pub rows: u16,
    pub columns: u16,
    pub width: u16,
    pub height: u16,
}

impl WindowSize {
    pub fn new() -> WindowSize {
        WindowSize {
            rows: 0,
            columns: 0,
            width: 0,
            height: 0,
        }
    }

    pub fn update(&mut self) {
        unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, self) };
    }
}

pub fn get_input_line(prompt: String) -> String {
    print!("{prompt}");
    stdout().flush().unwrap();
    let mut result: String = String::new();
    stdin()
        .read_line(&mut result)
        .expect("An unexpected error happened while reading line from stdin. Restart the program.");
    if let Some('\n') = result.chars().next_back() {
        result.pop();
    }
    if let Some('\r') = result.chars().next_back() {
        result.pop();
    }
    return result;
}

pub fn hide_cursor() {
    print!("\x1b[?25l\0");
}

pub fn show_cursor() {
    print!("\x1b[?25h\0")
}

// pub struct ProgressBar {
//     items: usize,
//     total_items: usize,
//     columns: usize,
//     unit: String,
// }

// impl ProgressBar {
//     pub fn new(total_items: usize, unit: String) -> ProgressBar {
//         let mut winsize = Winsize::new();
//         winsize.update();

//         hide_cursor();

//         ProgressBar {
//             items: 0,
//             total_items,
//             columns: winsize.columns as usize,
//             unit,
//         }
//     }

//     pub fn update(&mut self) {
//         self.items += 1;

//         self.display();
//     }

//     fn display(&self) {
//         let percentage = (self.items * 100) / self.total_items;
//         let progress_text = format!(
//             "{}% | {}/{} {}",
//             percentage, self.items, self.total_items, self.unit
//         );
//         let available_space = self.columns - progress_text.len() - 3;
//         let filled_characters = (self.items * available_space) / self.total_items;
//         let empty_characters = available_space - filled_characters;

//         print!(
//             "[{}{}] {}\r",
//             "=".repeat(filled_characters),
//             "-".repeat(empty_characters),
//             progress_text
//         );
//     }

//     pub fn end(&mut self) {
//         let progress_text = format!(
//             "100% | {}/{} {}",
//             self.total_items, self.total_items, self.unit
//         );
//         let available_space = self.columns - progress_text.len() - 3;

//         println!("[{}] {}", "=".repeat(available_space), progress_text);

//         show_cursor();
//     }
// }

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
