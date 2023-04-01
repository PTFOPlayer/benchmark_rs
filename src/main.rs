#[macro_use]
extern crate lazy_static;
use rand::Rng;
use std::{
    env, thread,
    time::{Duration, Instant},
};

lazy_static! {
    static ref ARGS: Vec<String> = env::args().collect();
    static ref PAGE: i32 = 1024;
    static ref DEFAULT_SETTINGS: Settings = Settings {
        threads: 8,
        time: 180.0,
        float: false
    };
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
            self.vector.push(square);
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
            self.vector.push(square);
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Settings {
    pub threads: u32,
    pub time: f64,
    pub float: bool,
}

impl Settings {
    fn new() -> Self {
        *DEFAULT_SETTINGS
    }
}

fn runner(settings: Settings) -> f64 {
    println!(
        "starting benchmark on settings: \nfloat:{}\ttime:{},\tcores:{}",
        settings.float, settings.time, settings.threads
    );

    let mut counter: u64 = 0;
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
            match handle.join() {
                Ok(_) => {
                    counter += 1;
                }
                Err(_) => {}
            };
        }
        if start.elapsed() > Duration::from_secs_f64(settings.time) {
            break;
        }
    }

    return counter as f64 / settings.time;
}

fn arg_parser() {
    let mut settings = Settings::new();
    for arg in ARGS[1..].iter() {
        if let "--help" | "-h" = arg.as_str() {
            println!(
                "
-f=(bool) => sets if using float or just int calculations (default false)
-t=(float) => sets time of benchmark (default 120s)
-c=(int) => number of threads (default 8)
"
            );
            return ();
        } else {
            match &arg[..2] {
                "-f" => {
                    let float: Result<bool, _> = arg[3..].parse();
                    if let Ok(res) = float {
                        settings.float = res;
                    } else {
                        println!("wrong parameter in argument -f=");
                    }
                }
                "-t" => {
                    let time: Result<f64, _> = arg[3..].parse();
                    if let Ok(res) = time {
                        settings.time = res;
                    } else {
                        println!("wrong parameter in argument -t=");
                    }
                }
                "-c" => {
                    let cores: Result<u32, _> = arg[3..].parse();
                    if let Ok(res) = cores {
                        settings.threads = res;
                    } else {
                        println!("wrong parameter in argument -c=");
                    }
                }
                &_ => {}
            }
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
