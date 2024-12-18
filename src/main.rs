use adventofcode2024::*;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Day of the month to run for, or "all".
    #[arg(short, long)]
    day: String,

    /// Folder path for input files.
    #[arg(short, long, default_value = "./inputs")]
    folder: String,
}


fn main() {

    let args = Args::parse();
    if args.day != "all" {
        let day_num = args.day.parse::<i32>()
            .expect("Please select a value between 1-25 or \"all\".");
        if !(day_num >= 0 && day_num <= 25) {
            panic!("Please select a value between 1-25 or \"all\".");
        }
    }

    if args.day == "0" { day0::run(&args.folder).values(); }
    if args.day == "1" || args.day == "all" { day1::run(&args.folder).values(); }
    if args.day == "2" || args.day == "all" { day2::run(&args.folder).values(); }
    if args.day == "3" || args.day == "all" { day3::run(&args.folder).values(); }
    if args.day == "4" || args.day == "all" { day4::run(&args.folder).values(); }
    if args.day == "5" || args.day == "all" { day5::run(&args.folder).values(); }
    if args.day == "6" || args.day == "all" { day6::run(&args.folder).values(); }
}
