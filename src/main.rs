use clap::Clap;
use std::io::Result;
use std::process::{Command, ExitStatus};

fn main() {
    let opts = Opts::parse();

    let exit_status = if opts.list {
        execute("tmux", &["ls"]).unwrap()
    } else if opts.delete.is_some() {
        kill_session(&opts.delete.unwrap()).unwrap()
    } else if opts.session.is_some() {
        new_session(&opts.session.unwrap()).unwrap()
    } else {
        new_session("TMUX").unwrap()
    };

    std::process::exit(exit_status.code().unwrap_or(0))
}

fn new_session(name: &str) -> Result<ExitStatus> {
    execute(
        "tmux",
        &[
            "new-session",
            "-A", // The -A flag makes new-session behave like attach-session if session-name already exists
            "-s", // Specifies the target session
            name,
        ],
    )
}

fn kill_session(name: &str) -> Result<ExitStatus> {
    execute("tmux", &["kill-session", "-t", name])
}

fn execute(exe: &str, args: &[&str]) -> Result<ExitStatus> {
    Command::new(exe).args(args).spawn()?.wait()
}

#[derive(Clap)]
struct Opts {
    /// List running Tmux sessions
    #[clap(short, long)]
    pub list: bool,

    /// Delete a Tmux session
    #[clap(short, long)]
    pub delete: Option<String>,

    /// Create or Attach tmux session
    pub session: Option<String>,
}
