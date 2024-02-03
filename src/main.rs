
/*

valid input in command line look like : 

ip_sniffer -h   // for help 
ip_sniffer -j 100 192.168.1.1       // set how many threads user wants
ip_sniffer 192.168ji.1.1  // default number of threads


*/

// libraries to import 
use std::env;
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::process;
use std::thread;
use std::sync::mpsc::{Sender, channel};
use std::io::{self, Write};


// Setting max range for ports 65535, so that it will stop after reaching max 
const MAX: u16 = 65535;

// struct Definition for values that we use
struct Arguments{
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str>{
        if args.len() < 2 {
            return  Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }
        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments {
                flag : String::from(""),
                ipaddr,
                threads: 4
            });
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") || flag.contains("--help") && args.len() == 2 {
                println!("Usage: -j to select how many threads you want
                \r\n    -h or -help or --help to show this help message");
                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help") || flag.contains("--help") {
                return Err("too many arguments");
            } else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]){
                    Ok(s) => s,
                    Err(_) => return Err("not a valid IP_Address; must be use IPv4 or IPv6")
                };
                let threads = match args[2].parse::<u16>(){
                    Ok(s) => s,
                    Err(_) => return Err("failed to parse thread number")
                };
                return Ok(Arguments{threads, flag, ipaddr});
            } else{
                return Err("invalid syntax");
            }
        }
    }
}

// Scan function : 
fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16){
    // variable to store port, & increment by 1
    let mut port: u16 = start_port + 1;

    loop {
       match TcpStream::connect((addr, port)){
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
       }

       if ( MAX - port) <= num_threads {
            break;
       }
       port += num_threads;
    }
}


fn main() {
    // using vector to store all the arguments
    let args: Vec<String> = env::args().collect();
    let program =  args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(0)
            } else{
                eprintln!("{} problem parsing arguments: {}", program, err);
                process::exit(0);
            }
        }
    );

    let num_threads = arguments.threads;
    let addr = arguments.ipaddr;
    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();
        
        thread::spawn(move || {
            scan(tx, i, arguments.ipaddr, num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}