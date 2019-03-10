use std::fs;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = parse_args(args);
    let p = path.unwrap();
    let paths = fs::read_dir(p).unwrap();
    for path in paths {
        println!("{}", path.unwrap().path().display());
    }
}

fn parse_args(args:Vec<String>) -> Result<&'static Path, &'static str> {
    match args.len(){
        1 => Ok(Path::new("./")),
        2 => {
                if Path::new(&args[1]).exists(){
                    Ok(Path::new(&args[1]))
                }else{
                    Err("Please pass a path for listing files.")
                }
            },
        _ => {
            Err("Please pass only one path string for listing files.")
        }
    }
}

#[test]
fn test_arg_parse1(){
    let input = vec!["","./"];
    let result = parse_args(&input);
    assert_eq!(result, "");
}

#[test]
fn test_arg_parse2(){
    let input2 = vec!["","123"];
    let result2 = parse_args(&input2);
    assert_eq!(result2, "");   
}