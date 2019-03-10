use std::fs;
use std::fs::DirEntry;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = parse_args(args);
    let p = path.unwrap();
    let i = p.len();
    let paths = fs::read_dir(p).unwrap();
    let mut output:String = String::new();

    let mut paths_vec: Vec<DirEntry> = paths.map(Result::unwrap).collect();
    paths_vec.sort_unstable_by(|a,b| a.path().cmp(&b.path()));

    for path in paths_vec {
        let s = path.path().display().to_string();
        //substring to remove the path from front
        let s_sub = &s[i..s.len()];
        output.push_str(&format!("{}  ", s_sub));        
    }
    
    println!("{}",output);
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
    result2.unwrap();   
}

#[test]
fn test_substring(){
    let input = "123456";
    let result = &input[2..6];
    assert_eq!(result, "3456");
}