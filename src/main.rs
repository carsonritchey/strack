use std::{fs, io::{prelude::*, BufReader, Write}};

const DAT_DIR: &str = ".strack";

const SHOW_DAT: &str = "show_dat.txt";
const TO_WATCH: &str = "to_watch.txt";

const SEPERATOR: char = ',';

fn main() {
    let mut shows: Vec<Show> = Show::file_to_vec(); 

    println!("{}", ansi_term::Style::new().italic().paint("type 'help' for help!"));
    loop {
        main_menu(&shows);
    }
}

enum MainMenuOptions {
    Help,
    Add,
    Remove,
    Progress,
    Set,
    List,
    Exit,
}
impl MainMenuOptions {
    fn value(&self) -> &str {
        match *self {
            MainMenuOptions::Help => "help",
            MainMenuOptions::Add => "add",
            MainMenuOptions::Remove => "remove",
            MainMenuOptions::Progress => "progress",
            MainMenuOptions::Set => "set",
            MainMenuOptions::List => "list",
            MainMenuOptions::Exit => "exit",
        }
    }
}

struct Show {
    episode: usize,
    season: usize,
    last_watched: String,
    finished: bool,
}
impl Show {
    fn file_to_vec() -> Vec<Show> {
        let v: Vec<Show> = Vec::new();

        v
    }

    fn vec_to_file(shows: &Vec<Show>) {

    }

    fn new_show() -> Show {
        Show {episode: prompt_and_parse("current episode?"), season: prompt_and_parse("current season?"), last_watched: get_date_string(), finished: false}
    }

    fn add_show(shows: &mut Vec<Show>, show: &Show) {
        shows.push(Show::new_show());

        Show::vec_to_file(shows);
    }

    fn remove_show(shows: &Vec<Show>, index: usize) {
        Show::vec_to_file(shows);
    }
}

fn main_menu(shows: &Vec<Show>) {
    let choice = prompt("");

    if choice.starts_with(MainMenuOptions::Help.value()) {
        println!("\n'add'      to add a show\n'reomve'   to remove a show\n'progress' to progress in a show\n'set'      to set a show to a specific episode\n'list'     to list your shows\n'exit'     to exit");
    } else if choice.starts_with(MainMenuOptions::Add.value()) {

    } else if choice.starts_with(MainMenuOptions::Remove.value()) {

    } else if choice.starts_with(MainMenuOptions::Progress.value()) {

    } else if choice.starts_with(MainMenuOptions::Set.value()) {

    } else if choice.starts_with(MainMenuOptions::List.value()) {

    } else if choice.starts_with(MainMenuOptions::Exit.value()) {
        std::process::exit(1); 
    } else {
        println!("{}", ansi_term::Style::new().italic().paint("unknown command"));
    }
}

// helper functions 
fn prompt(prompt: &str) -> String {
    let mut s = String::new();
    if prompt.len() != 0 {
        println!("{}", prompt);
    }
    std::io::stdin().read_line(&mut s).unwrap();

    s.trim_end().to_string()
}

fn prompt_and_parse(prompt: &str) -> usize {
    return loop {
        let mut s = String::new();
        println!("{}", prompt);
        std::io::stdin().read_line(&mut s).unwrap();

        let test = &s.trim_end().parse::<usize>();
        match test {
            Ok(ok) => break *ok,
            _ => continue,
        }
    }
}
// returns absolute path to the dat dir
fn dat_path() -> String {
    match home::home_dir() {
        Some(p) => return format!("{}/{}", p.display().to_string(), DAT_DIR),
        None => panic!("unable to locate home directory"),
    }
} 

fn get_date_string() -> String {
    let date: String = chrono::offset::Local::now().to_string();
    let now: Vec<&str> = date.split(' ').collect();

    now.get(0).expect("unable to get date").to_string()
}
