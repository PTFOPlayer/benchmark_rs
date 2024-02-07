#[macro_use]
extern crate lazy_static;
use rand::Rng;
use std::{
    env, thread,
    time::{Duration, Instant},
};

const PAGE: usize = 2048;

lazy_static! {
    static ref ARGS: Vec<String> = env::args().collect();
    static ref DATA: Vec<Vec<f64>> = {
        let mut rng = rand::thread_rng();
        let mut temp = vec![vec![0.0f64; PAGE]; PAGE];
        for i in 0..PAGE {
            for j in 0..PAGE {
                temp[i][j] = rng.gen::<f64>();
            }
        }

        return temp;
    };
}

pub struct Arr64 {
    pub vector: Vec<f64>,
}

impl Arr64 {
    fn run(&mut self) {
        for i in 0..PAGE {
            let mut sqrt: f64 = 0.0;
            for j in 0..PAGE {
                sqrt += f64::sqrt(DATA[i][j] / DATA[j][i]);
            }
            self.vector.push(sqrt);
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Settings {
    pub threads: u32,
    pub time: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            threads: 8,
            time: 180.0,
        }
    }
}

fn runner(settings: Settings) -> f64 {
    println!(
        "starting benchmark on settings: \ntime:{},\tcores:{}",
        settings.time, settings.threads
    );

    println!("preparing data...");
    let p_start = Instant::now();
    let dummy = DATA[0][0];
    println!(
        "(dummy: {:?}), data prepared in: {:?}",
        dummy,
        p_start.elapsed()
    );
    println!("starting test...");

    let mut counter: u64 = 0;
    let start = Instant::now();
    let duration = Duration::from_secs_f64(settings.time);

    let mut handles = vec![];
    loop {
        if handles.len() < settings.threads as usize {
            let handle = thread::spawn(|| Arr64 { vector: vec![] }.run());
            handles.push(handle);
        } else {
            let mut len = handles.len() - 1;
            while len > 0 {
                if handles[len].is_finished() {
                    counter += 1;
                    handles.remove(len);
                }
                len -= 1;
            }
        }

        if start.elapsed() > duration {
            break;
        }
    }
    println!("finised with score:");
    return counter as f64 / settings.time;
}

fn arg_parser() {
    let mut settings = Settings::default();
    for arg in ARGS[1..].iter() {
        if let "--help" | "-h" = arg.as_str() {
            println!(
                "
-t=(float) => sets time of benchmark (default 120s)
-c=(int) => number of threads (default 8)
"
            );
            return ();
        } else {
            match &arg[..2] {
                "-t" => {
                    let time: Result<f64, _> = arg[3..].parse();
                    if let Ok(res) = time {
                        settings.time = res;
                    } else {
                        println!("wrong parameter in argument -t=?");
                    }
                }
                "-c" => {
                    let cores: Result<u32, _> = arg[3..].parse();
                    if let Ok(res) = cores {
                        settings.threads = res;
                    } else {
                        println!("wrong parameter in argument -c=?");
                    }
                }
                &_ => {}
            }
        };
    }
    println!("{}", runner(settings));
}

fn main() {
    if ARGS.len() < 1 {
        println!("no arguments passed, see --help");
    } else {
        arg_parser();
    }
}
