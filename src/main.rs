#[macro_use]
extern crate lazy_static;
use rand::Rng;
use std::{
    env, thread,
    time::{Duration, Instant},
};

lazy_static! {
    static ref ARGS: Vec<String> = env::args().collect();
    static ref PAGE: i32 = 512;
}

trait SingleRun {
    fn run(&mut self);
}

pub struct Arr64<T> {
    pub vector: Vec<T>,
}

impl SingleRun for Arr64<f64> {
    fn run(&mut self) {
        let mut rng = rand::thread_rng();
        for _ in 0..*PAGE {
            let mut square: f64 = 0.0;
            for _ in 0..*PAGE {
                square += f64::sqrt(rng.gen::<f64>() / rng.gen::<f64>());
            }
            self.vector.append(&mut vec![square]);
        }
    }
}
impl SingleRun for Arr64<i64> {
    fn run(&mut self) {
        let mut rng = rand::thread_rng();
        for _ in 0..*PAGE {
            let mut square: i64 = 0;

            for _ in 0..*PAGE {
                square += i64::pow(rng.gen::<i64>() / rng.gen::<i64>(), rng.gen::<u32>());
            }
            self.vector.append(&mut vec![square]);
        }
    }
}

struct Settings {
    pub threads: i32,
    pub time: f64,
    pub float: bool,
}

impl Settings {
    fn new(threads: i32, time: f64, float: bool) -> Self {
        Settings {
            threads,
            time,
            float,
        }
    }
}

fn runner(settings: Settings) -> i32 {
    println!(
        "starting benchmark on settings: \nfloat:{}\ttime:{},\tcores:{}",
        settings.float, settings.time, settings.threads
    );
    let mut counter = 0;
    let start = Instant::now();
    loop {
        let mut handles = vec![];
        for _ in 0..settings.threads {
            if settings.float {
                let handle = thread::spawn(|| {
                    Arr64::<f64> { vector: vec![] }.run();
                });
                handles.push(handle);
            } else {
                let handle = thread::spawn(|| {
                    Arr64::<i64> { vector: vec![] }.run();
                });
                handles.push(handle);
            }
        }
        for handle in handles {
            handle.join().unwrap();
            counter += 1;
        }
        if start.elapsed() > Duration::from_secs_f64(settings.time) {
            break;
        }
    }
    return counter;
}

fn arg_parser() {
    let mut settings = Settings::new(8, 120.0, false);
    for arg in ARGS[1..].iter() {
        match arg.as_str() {
            "--help" | "-h" => {
                println!(
                    "
-f=(bool) => sets if using float or just int calculations (default false)
-t=(float) => sets time of benchmark (default 120s)
-c=(int) => number of threads (default 8)
"
                );
                return ();
            }
            &_ => match &arg.as_str()[..2] {
                "-f" => {
                    let float: Result<bool, _> = arg[3..].parse();
                    match float {
                        Ok(res) => {
                            settings.float = res;
                        }
                        Err(_) => {
                            println!("wrong parameter in argument -f=");
                        }
                    }
                }
                "-t" => {
                    let time: Result<f64, _> = arg[3..].parse();
                    match time {
                        Ok(res) => {
                            settings.time = res;
                        }
                        Err(_) => {
                            println!("wrong parameter in argument -t=");
                        }
                    }
                }
                "-c" => {
                    let cores: Result<i32, _> = arg[3..].parse();
                    match cores {
                        Ok(res) => {
                            settings.threads = res;
                        }
                        Err(_) => {
                            println!("wrong parameter in argument -c=");
                        }
                    }
                }
                &_ => {}
            },
        };
    }
    println!("{}", runner(settings));
}

fn main() {
    if ARGS.len() < 2 {
        println!("no arguments passed, see --help");
    } else {
        arg_parser();
    }
}
