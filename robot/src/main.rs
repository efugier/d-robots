use std::fs::OpenOptions;
use std::path::PathBuf;

use gag::Redirect;
use log;
use structopt::StructOpt;

mod ai;
mod app;
mod events;
mod messages;
mod robot;

use app::{App, AppId};

#[derive(StructOpt, Debug)]
#[structopt(name = "netchat")]
pub struct Opt {
    /// Input file
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    input: PathBuf,

    /// Output file
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: PathBuf,

    //Application Identifier
    #[structopt(short = "n", long = "name")]
    app_id: Option<AppId>,

    //Application Identifier
    #[structopt(short = "l", long = "logfile")]
    logfile: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();

    color_backtrace::install();
    env_logger::init();

    // Open a log file
    let mut _stderr_redirect_handle = None;
    if let Some(logfile) = opt.logfile {
        let log = OpenOptions::new()
            .truncate(true)
            .read(true)
            .create(true)
            .write(true)
            .open(logfile)
            .unwrap();
        _stderr_redirect_handle = Some(Redirect::stderr(log).unwrap());
    }

    let mut app = App::new(
        opt.app_id.unwrap_or_else(|| rand::random()),
        opt.output,
        opt.input,
    );

    if let Err(e) = app.run() {
        log::error!("Something went wrong {}", e);
    }
}
