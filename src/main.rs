#[macro_use]
extern crate clap;

#[macro_use]
extern crate cmd_lib;

extern crate git2;

use clap::{App, AppSettings};

mod cmd;

pub(crate) type Result<T> = anyhow::Result<T>;

fn main() -> Result<()> {
    let app = App::new("git clear tools")
        .about(crate_description!())
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .version(crate_version!())
        .about("about something")
        .subcommand(cmd::branch::make_subcommand());

    match app.get_matches().subcommand() {
        ("branch", Some(sub_matches)) => cmd::branch::execute(sub_matches),
        (_, _) => unreachable!(),
    };
    Ok(())
}