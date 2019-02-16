use std::io::prelude::*;
use std::fs::File;
use std::thread;
use std::str;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};

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

pub fn run(args: Arguments, sender_res: mpsc::Sender<u64>, 
    sender_contents: mpsc::Sender<String>, 
    receiver_contents: Arc<Mutex<mpsc::Receiver<String>>>) -> Result<(), &'static str> {
    if let Err(err) = read_file(&args.file_name, sender_contents) {
            return Err(err);
    };

    let counter = Arc::new(Mutex::new(0));
    let keyword = args.keyword.to_lowercase();
    

    for _ in 0..args.threads {
        let receiver_contents = receiver_contents.clone();
        let counter = counter.clone();
        let sender_res = sender_res.clone();
        let keyword = keyword.clone();
        thread::spawn(move || {
            for c in receiver_contents.lock().unwrap().iter() {
                let counter = counter.clone();
                let sender_res = sender_res.clone();
                let keyword = keyword.clone();
                search(sender_res, counter, keyword, c);
            }
        });
    }

    Ok(())
}

fn search(tx: mpsc::Sender<u64>, count: Arc<Mutex<u64>>, keyword: String, contents: String) {
    let mut num = count.lock().unwrap();
    for w in contents.lines() {
        for _ in w.matches(&keyword){      
            *num += 1;
        }
    }
    tx.send(*num).unwrap();
}

fn read_file(path: &String, tx: mpsc::Sender<String>) -> Result<(), &'static str> {
    let mut f = match File::open(&path) {
        Ok(f) => f,
        Err(_) => return Err("error reading path file"), 
    };

     thread::spawn(move || {
        loop {
            let mut buffer = vec![0; 1024];
            // read up to 1024 bytes
            let line_read = f.read(&mut buffer[..]).unwrap();

            if line_read == 0 {
                break;
            }

            let content_chunks = str::from_utf8(&buffer[..line_read]).unwrap();
            
            tx.send(content_chunks.to_lowercase()).unwrap();
        }
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
