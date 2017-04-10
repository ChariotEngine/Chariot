extern crate open_aoe_drs;

extern crate pancurses;

extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("drs_browser")
        .version("0.1.0")
        .author("Taryn Hill <taryn@phrohdoh.com>")
        .about("TUI drs archive browser")
        .arg(clap::Arg::with_name("path")
            .long("file-path")
            .value_name("path to drs")
            .required(true)
            .takes_value(true))
        .get_matches();

    let drs_file_path = matches.value_of("path").unwrap();

    let drs = open_aoe_drs::DrsFile::read_from_file(&drs_file_path)
        .expect(&format!("Failed to read {}", drs_file_path));

    let slp_table = drs.find_table(open_aoe_drs::DrsFileType::Slp).expect("TODO: support non-slp tables");
    let entry_ids = slp_table.entries
        .iter()
        .map(|entry| {
            format!("{{id:{}, offset:{}, size:{}}}",
                    entry.file_id,
                    entry.file_offset,
                    entry.file_size)
        })
        .collect::<Vec<_>>();

    let win = pancurses::initscr();
    let max_y = win.get_max_y();

    if pancurses::has_colors() {
        pancurses::start_color();
    }

    pancurses::init_pair(1, pancurses::COLOR_GREEN, pancurses::COLOR_BLACK);
    let attr_selected = pancurses::ColorPair(1);


    win.keypad(true);
    pancurses::noecho();

    let mut selected_idx = 0;

    let mut explicit_quit = false;

    loop {
        for (i, s) in entry_ids.iter().enumerate() {
            let i = i as i32;
            if i == selected_idx {
                win.attron(attr_selected);
            }

            if i >= max_y {
                continue;
            }

            let print_y = {
                let next = i + 1;
                let max_minus_1 = max_y - 1;

                if next < max_minus_1 {
                    next
                } else {
                    max_minus_1
                }
            };

            win.mvprintw(print_y, 0, &s);
            win.attroff(attr_selected);
        }

        win.mvprintw(max_y - 1, 0, &format!("Found {} entries", entry_ids.len()));

        match win.getch() {
            Some(pancurses::Input::KeyDown) => {
                selected_idx += 1;
                if selected_idx as usize > entry_ids.len() {
                    selected_idx = entry_ids.len() as i32 - 1;
                }

                ()
            }
            Some(pancurses::Input::KeyUp) => {
                selected_idx -= 1;
                if selected_idx < 0 {
                    selected_idx = 0;
                }

                ()
            }
            Some(pancurses::Input::Character('\n')) => break,
            Some(pancurses::Input::Character('q')) => {
                explicit_quit = true;
                break;
            }
            _ => (),
        }

        win.refresh();
    }

    pancurses::endwin();
}
