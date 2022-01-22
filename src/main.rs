use std::{path, fs, io};
use std::io::prelude::*;

const DAT_DIR: &str = ".strack";

const SHOW_DAT: &str = "show_dat.txt";

const SEPERATOR: char = ',';
const WATCHLIST: &str = "to watch";

fn main() {
    create_dir();

    let mut shows: Vec<Show> = Show::file_to_vec();
    
    println!("{}", ansi_term::Style::new().italic().paint("type 'help' for help!"));
    loop {
        main_menu(&mut shows);
    }
}

enum MainMenu {
    Help,
    Add,
    Remove,
    Progress,
    Set,
    List,
    Exit,
}
impl MainMenu {
    fn value(&self) -> &str {
        match *self {
            MainMenu::Help     => "help",
            MainMenu::Add      => "add",
            MainMenu::Remove   => "remove",
            MainMenu::Progress => "progress",
            MainMenu::Set      => "set",
            MainMenu::List     => "list",
            MainMenu::Exit     => "exit",
        }
    }

    // option functions 
    fn add(shows: &mut Vec<Show>) {
        let name: String = prompt("name of show?").replace(SEPERATOR, "");

        for show in &mut *shows {
            if &show.name.to_lowercase() == &name.to_lowercase() {
                println!("'{}' already exists. add anways? (y/n)", &name);

                if prompt("").starts_with("n") { return }
                else { break }
            } else if name.len() == 0 { return }
        }

        let season: String = prompt("current season? (press enter to default to s1, e1)");

        if season.len() == 0 {
            Show::add_show(shows, Show {name, episode: 1, season: 1, last_watched: get_date_string(), finished: false});
        } else {
            let season: usize = season.trim_end().parse::<usize>().unwrap(); 
            let episode: usize = prompt_and_parse("current episode?");

            Show::add_show(shows, Show {name, episode, season, last_watched: get_date_string(), finished: false});
        }
    }

    fn remove(shows: &mut Vec<Show>) {
        for (i, show) in shows.iter().enumerate() {
            println!("{}: '{}'", i + 1, show.name);
        }

        let index = prompt_and_parse("please select an index to remove ('0' to cancel)");
        if index == 0 { return; }

        println!("removed '{}'!", shows[index - 1].name);
        shows.remove(index - 1);
        Show::vec_to_file(shows); 
    }

    fn set(shows: &mut Vec<Show>) {
        
    }

    fn list(shows: &mut Vec<Show>) {
        if shows.len() == 0 {
            println!("no shows found to list...\nyou can add some with the 'add' command");
        }

        let mut inprogress: Vec<&Show> = Vec::new(); 
        let mut finished: Vec<&Show> = Vec::new(); 
        let mut towatch: Vec<&Show> = Vec::new(); 

        let mut finished_length: usize = 0;
        let mut inprogress_length: usize = 0;
        for show in &mut *shows {
            if show.finished { 
                if show.name.len() > finished_length { finished_length = show.name.len(); }
                finished.push(&*show); 
            }
            else if show.last_watched == WATCHLIST { towatch.push(&*show); }
            else {
                if show.name.len() > inprogress_length { inprogress_length = show.name.len(); }
                inprogress.push(&*show);
            }
        }

        println!("{}", ansi_term::Style::new().bold().paint("finished shows:"));
        for show in finished {
            let mut space: String = String::new();
            for _n in 0..finished_length-show.name.len() {
                space += " ";
            }
            println!("{} '{}'{} {}", Show::position(show), show.name, space, show.last_watched);
        }
        
        println!("\n{}", ansi_term::Style::new().bold().paint("watch list:"));
        for show in towatch {
            println!("'{}'", show.name);
        }

        println!("\n{}", ansi_term::Style::new().bold().paint("in progress:"));
        for show in inprogress {
            let mut space: String = String::new();
            for _n in 0..inprogress_length-show.name.len() {
                space += " ";
            }
            println!("{} '{}'{} {}", Show::position(show), show.name, space, show.last_watched);
        }
    }
}

struct Show {
    name: String,
    season: usize,
    episode: usize,
    last_watched: String,
    finished: bool,
}
impl Show {
    // reads file and puts its contents into Vec<Show>
    fn file_to_vec() -> Vec<Show> {
        let mut v: Vec<Show> = Vec::new();

        let dat = fs::read_to_string(format!("{}/{}", dat_path(), SHOW_DAT)).expect("unable to read show_dat.txt");

        for line in dat.lines() {
            let mut show = Show{name: String::new(), episode: 0, season: 0, last_watched: String::new(), finished: true}; 
            let split: Vec<&str> = line.split(SEPERATOR).collect();

            show.name         = split[0].to_string();
            show.season       = split[1].parse::<usize>().expect("unable to read show_dat.txt");
            show.episode      = split[2].parse::<usize>().expect("unable to read show_dat.txt");
            show.last_watched = split[3].to_string();
            show.finished     = parse_bool(split[4]);

            v.push(show);
        }

        v
    }

    // writes contents of Vec<Show> to file
    fn vec_to_file(shows: &Vec<Show>) {
        let mut to_send = String::new();
        for show in shows {
            let fuck = format!("{}{}{}{}{}{}{}{}{}\n", &show.name, SEPERATOR, &show.season, SEPERATOR, &show.episode, SEPERATOR, &show.last_watched, SEPERATOR, &show.finished);

            to_send.push_str(&fuck);
        }

        if to_send.len() == 0 { return }

        // clears file
        let mut f = fs::File::create(format!("{}/{}", dat_path(), SHOW_DAT)).expect("unable to open file");

        f.write_all(to_send.as_bytes()).expect("unable to write to file");
    }

    fn add_show(shows: &mut Vec<Show>, show: Show) {
        println!("{}", ansi_term::Style::new().italic().paint(format!("added '{}' @ (s{}, e{})", &show.name, &show.season, &show.episode)));
        shows.push(show);
        Show::vec_to_file(shows);
    }

    // returns current season and episode i.e. (s02, e01)
    fn position(show: &Show) -> String {
        format!("(s{:0w$},e{:0w$})", show.season, show.episode, w = 2)
    }
}

fn main_menu(shows: &mut Vec<Show>) {

    let choice = prompt("");
    if choice.starts_with(MainMenu::Help.value()) {
        println!("'add'      to add a show\n'remove'   to remove a show\n'progress' to progress in a show\n'set'      to set a show to a specific episode or watch later\n'list'     to list your shows\n'exit'     to exit");
    } else if choice.starts_with(MainMenu::Add.value()) {
        MainMenu::add(shows);
    } else if choice.starts_with(MainMenu::Remove.value()) {
        MainMenu::remove(shows);
    } else if choice.starts_with(MainMenu::Progress.value()) {

    } else if choice.starts_with(MainMenu::Set.value()) {
        MainMenu::set(shows);
    } else if choice.starts_with(MainMenu::List.value()) || choice == "ls" {
        MainMenu::list(shows); 
    } else if choice.starts_with(MainMenu::Exit.value()) {
        std::process::exit(1); 
    } else {
        println!("{}", ansi_term::Style::new().italic().paint("unknown command"));
    }
}


// helper functions 

// prompts and returns what user inputs
fn prompt(prompt: &str) -> String {
    let mut s = String::new();
    if prompt.len() != 0 {
        println!("{}", prompt);
    }
    print!("> ");
    io::stdout().flush().expect("unable to format menu input");
    std::io::stdin().read_line(&mut s).unwrap();

    s.trim_end().to_string()
}

// prompts and converts to usize
fn prompt_and_parse(prompt: &str) -> usize {
    return loop {
        let mut s = String::new();
        println!("{}", prompt);
        print!("> ");
        io::stdout().flush().expect("unable to format menu input");
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
        None => panic!("unable to find home directory"),
    }
} 

// returns the current date formatted
fn get_date_string() -> String {
    let date: String = chrono::offset::Local::now().to_string();
    let now: Vec<&str> = date.split(' ').collect();

    now.get(0).expect("unable to get date").to_string()
}

// creates strack directory and populates it
fn create_dir() {
    let path: String = dat_path(); 

    if !path::Path::new(&path).is_dir() {
        println!("{} was not found; creating directory...", &path);
        fs::create_dir_all(&path).expect("unable to create strack dir");

        fs::File::create(format!("{}/{}", &path, SHOW_DAT)).expect("unable to create show_dat file");
    }
}

// converts string to bool
fn parse_bool(s: &str) -> bool {
    if s == "true" {
        return true;
    }

    false
}
