use chrono::{DateTime, Utc};
use std::path::Path;
use std::fs::create_dir_all;
use shellexpand::tilde;

// Map constant root path for daily temp folders (can pull from env in future)
const ROOT_PATH: &str = "~/Workspace/temp";

// Map desired datetime format string
const TIME_FORMAT: &str = "%m-%e-%Y";

fn main() {
    // discover today's full folder name
    let now: DateTime<Utc> = Utc::now();
    let today_string = now.format(TIME_FORMAT);
    //println!("{}", today_string);

    let expanded_root = tilde(ROOT_PATH);
    let today_fullpath: String = format!("{}{}{}", expanded_root, "/", today_string);

    // check for today's folder existence
    if Path::new(&today_fullpath).exists() {
        //println!("{}", "folder already existss!");
        println!("{}", today_fullpath);
    } else{
        // create today's folder if not exists
        //println!("{}", "folder nonexistent");

        match create_dir_all(&today_fullpath) {
            Ok(_v) => println!("{}", today_fullpath),
            Err(_e) => panic!("Error creating directory! {}", today_fullpath)
        }
    }
}
