//! This binary takes all profiling results within the `INPUT_DIR` directory and
//! generates per-frame stats: (frame_num, width, skip, quant, true_positive,
//! false_positive, false_negative).

extern crate evaluation;
extern crate rayon;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use evaluation::{Profile, VideoConfig, FrameStat};
use rayon::prelude::*;
use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();

    let configurations = match &opt.profile_path {
        &Some(ref path) => {
            let profile: Profile<VideoConfig> = Profile::new(&path);
            profile.all_params()
        }
        &None => evaluation::all_configurations(),
    };

    let vec_frame_stat = configurations
        .par_iter()
        .map(|&vc| {
            println!("running for {}", vc);
            evaluation::get_frame_stats(&opt.input_dir, vc, opt.limit)
        })
        .flat_map(|s| s)
        .collect::<Vec<_>>();

    let cwd = ".".to_string();
    let outfile = format!("{}/stat.csv", opt.output_dir.unwrap_or(cwd));

    FrameStat::to_csv(vec_frame_stat, outfile);
}

#[derive(StructOpt, Debug)]
#[structopt(name = "stat")]
#[structopt(about = "Generate per-frame stat from profile output folder.")]
struct Opt {
    /// The folder that contains profiling measurement.
    #[structopt(help = "Input Directory")]
    input_dir: String,

    /// A profile that limits what configuration to choose when generating stats.
    #[structopt(short = "p", long = "profile")]
    #[structopt(help = "The path to the profile")]
    profile_path: Option<String>,

    /// The folder that contains profiling measurement.
    #[structopt(short = "o", long = "out")]
    #[structopt(help = "Output directory, current directory if empty")]
    output_dir: Option<String>,

    /// The limit of frames to process
    #[structopt(short = "l", long = "limit")]
    #[structopt(help = "Number of frames to process")]
    limit: Option<usize>,
}
