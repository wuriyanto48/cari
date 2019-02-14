use std::io::prelude::*;
use std::fs::File;

const VERSION: &'static str = "0.0.0";

#[derive(Debug)]
pub struct Arguments {
    pub flag: String,
    pub keyword: String,
    pub file_name: String,
    pub threads: u32,
}

pub struct Output (String, u64);

impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 7 {
            return Err("too many arguments");
        }

        // app -k wury -f f.txt -t 5
        // 0   1   2   3   4    5  6
        let flag_1 = args[1].clone();
        
        if flag_1.contains("-h") || flag_1.contains("-help") && args.len() == 2 {
            println!("Usage");
            println!("-k: keyword to search");
            println!("-f: filename path");
            println!("-t: num of threads");
            println!("-h/ -help: show help");
            println!("-v: show version");
            return Err("help");
        } else if flag_1.contains("-h") || flag_1.contains("-help") {
            return Err("too many arguments");
        } else if flag_1.contains("-v") || flag_1.contains("-version") && args.len() == 2 {
            println!("sniff version: {}", VERSION);
            return Err("help");
        } else if flag_1.contains("-v") || flag_1.contains("-version") {
            return Err("too many arguments");
        }else if flag_1.contains("-k") || flag_1.contains("-f"){
            if args.len() > 4 {
                let keyword: String;
                let file_name: String;
                let threads: u32;

                if flag_1.contains("-k") {
                    keyword = args[2].clone();
                    file_name = args[4].clone();
                } else if flag_1.contains("-f") {
                    keyword = args[4].clone();
                    file_name = args[2].clone();
                } else {
                    keyword = args[2].clone();
                    file_name = args[4].clone();
                }

               if args.len() > 6 && args[5].clone().contains("-t"){
                    threads = match args[6].parse::<u32>() {
                        Ok(s) => s,
                        Err(_) => return Err("-t flag is not a number")
                    };
               } else {
                   threads = 4;
               }

                return Ok(Arguments{
                    flag: flag_1,
                    keyword,
                    file_name,
                    threads,
                });
            } else {
                return Err("not enough arguments");
            }
        } else {
            return Err("invalid arguments");
        }
    }
}

fn read_file(path: &String) -> Result<String, &'static str> {
    let mut f = match File::open(&path) {
        Ok(f) => f,
        Err(_) => return Err("error reading path file"), 
    };

    let mut contents = String::new();
    if let Err(_) = f.read_to_string(&mut contents) {
        return Err("error reading contents");
    };

    Ok(contents)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
