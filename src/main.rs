#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate log;
extern crate loggerv;
extern crate clipboard;

use clap::{App, Arg};
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use std::thread;
use std::time::Duration;

error_chain! {
    foreign_links {
        Cli(::clap::Error);
        Logging(::log::SetLoggerError);
    }
}

fn main() {
    if let Err(error) = try_main() {
        error!("{}", error);

        for error in error.iter().skip(1) {
            error!("Caused by: {}", error);
        }

        if let Some(backtrace) = error.backtrace() {
            error!("Backtrace: {:?}", backtrace);
        }
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    let args = app().get_matches();

    loggerv::init_with_verbosity(args.occurrences_of("v"))?;

    let delay_ms = value_t!(args, "delay", u64)?;
    let delay = Duration::from_millis(delay_ms);
    debug!("Polling delay: {:?}", delay);

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    let mut last = get_content(&mut ctx).unwrap_or_default();
    loop {
        if let Some(next) = get_content(&mut ctx) {
            if next != last {
                println!("{}", next);
                last = next;
            }
        }

        thread::sleep(delay);
    }
}

fn get_content(ctx: &mut ClipboardContext) -> Option<String> {
    match ctx.get_contents() {
        Ok(content) => Some(content),
        Err(e) => {
            info!("Unable to get content from clipboard: {:?}", e);
            None
        }
    }
}

fn app() -> App<'static, 'static> {
    App::new("watch-clipboard")
        .version(crate_version!())
        .about("Monitor clipboard for changes and print them to stdout")
        .arg(Arg::with_name("delay")
                 .short("d")
                 .long("delay")
                 .help("Delay in milliseconds before polling clipboard for new content")
                 .default_value("250"))
        .arg(Arg::with_name("v")
                 .short("v")
                 .help("Enable logging, use multiple `v`s to increase verbosity")
                 .multiple(true))
}
