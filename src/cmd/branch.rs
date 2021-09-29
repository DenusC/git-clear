use clap::{App, Arg, ArgMatches, SubCommand, AppSettings};
use git2::{BranchType, Repository};

pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("branch")
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::ColoredHelp)
        .arg(Arg::with_name("force")
            .short("f")
            .long("force")
            .takes_value(false)
            .required(false)
            .help("use -f to actually do something, else run on dry-run mode!")
        )
}

pub fn execute(args: &ArgMatches) {
    do_clear(args).unwrap();
}

fn do_clear(args: &ArgMatches) -> crate::Result<()>{
    let repo = Repository::open(".")?;
    let merged_branches: String = run_fun!(git branch --merged)?;
    for branch in merged_branches.split("\n") {
        if !branch.contains("*") {
            let mut b = repo.find_branch(branch.replace(" ", "").as_str(), BranchType::Local)?;
            if args.is_present("force") {
                b.delete().unwrap();
                println!("delete branch: {}", branch);
            } else {
                println!("{}", branch);
            }
        }
    }
    Ok(())
}

// #[cfg(target_os = "windows")]
// fn do_clear(args: &ArgMatches){
//     panic!("unsupported!")
// }
//
// #[cfg(not(target_os = "windows"))]
// fn do_clear(args: &ArgMatches) {
//     if args.is_present("dry-run") {
//         run_cmd!(git branch --merged| grep -v "\\*");
//     } else {
//         run_cmd!(git branch --merged| grep -v "\\*" | xargs -n 1 git branch -d);
//     }
// }

