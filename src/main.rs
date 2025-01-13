use std::fs::File;
use std::io::{BufRead, BufReader};
use flate2::read::GzDecoder;
use rayon::prelude::*;
use clap::{Arg, Command};
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};

fn main() {
    let matches = Command::new("Fastq Processor")
        .arg(Arg::new("fastqgz")
            .short('f')
            .long("fastqgz")
            .value_name("FILE")
            .help("Path to the fastq.gz file")
            .required(true))
        .arg(Arg::new("threads")
            .short('t')
            .long("threads")
            .value_name("THREADS")
            .help("Number of threads to use")
            .default_value("4"))
        .arg(Arg::new("early_exit")
            .short('e')
            .long("early-exit")
            .value_name("EARLY_EXIT")
            .help("Whether to exit early when clean data is detected")
            .default_value("true"))
        .get_matches();

    let fastq_file = matches.get_one::<String>("fastqgz").unwrap();
    let num_threads: usize = matches.get_one::<String>("threads").unwrap().parse().expect("Invalid number of threads");
    let early_exit: bool = matches.get_one::<String>("early_exit").unwrap().parse().expect("Invalid early exit value");

    rayon::ThreadPoolBuilder::new().num_threads(num_threads).build_global().unwrap();

    let file = File::open(fastq_file).expect("无法打开文件");
    let decoder = GzDecoder::new(file);
    let reader = BufReader::new(decoder);

    let read_count = AtomicU64::new(0);
    let total_bases = AtomicU64::new(0);
    let is_clean = AtomicBool::new(false);
    let first_length = AtomicU64::new(0);

    let lines: Vec<_> = reader.lines().collect();
    lines.par_iter().enumerate().for_each(|(i, line)| {
        if i % 4 == 1 {
            let line = line.as_ref().expect("无法读取行");
            let length = line.len() as u64;

            read_count.fetch_add(1, Ordering::Relaxed);
            total_bases.fetch_add(length, Ordering::Relaxed);

            if first_length.load(Ordering::Relaxed) == 0 {
                first_length.compare_exchange(0, length, Ordering::Relaxed, Ordering::Relaxed).ok();
            } else if length != first_length.load(Ordering::Relaxed) {
                is_clean.store(true, Ordering::Relaxed);
                if early_exit {
                    println!("文件名: {}, 读取的reads数: {}, 数据量: {} bp, 状态: clean",
                             fastq_file, read_count.load(Ordering::Relaxed), total_bases.load(Ordering::Relaxed));
                    std::process::exit(0);
                }
            }
        }
    });

    let status = if is_clean.load(Ordering::Relaxed) { "clean" } else { "raw" };
    println!("文件名: {}, 读取的reads数: {}, 数据量: {} bp, 状态: {}",
             fastq_file, read_count.load(Ordering::Relaxed), total_bases.load(Ordering::Relaxed), status);
}
