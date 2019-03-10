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

fn parse_args(args:Vec<String>) -> Result<String, &'static str> {
    match args.len(){
        1 => Ok(String::from("./")),
        _ => {
                if Path::new(&args[1]).exists(){            
                    Ok(args[1].clone())
                }else{
                    Err("Please pass a path string for listing files.")
                }
            }
    }
}

#[test]
fn test_arg_parse1(){
    let input = vec!["".to_string(),"./".to_string()];
    let result = parse_args(input);
    assert_eq!(result.unwrap(), "./".to_string());
}

#[test]
#[should_panic]
fn test_arg_parse2(){
    let input2 = vec!["".to_string(),"12387".to_string()];
    let result2 = parse_args(input2);
    let x = result2.unwrap();   
}