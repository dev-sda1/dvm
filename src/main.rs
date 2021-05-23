use dvm::{Res, error, r#type::Type, install::install, remove::remove, update::update, show::show};
use clap::{AppSettings, Clap};

const POSSIBLE_VALUES: &[&str] = &["stable", "discord-stable", "s", "canary", "discord-canary", "c", "ptb", "discord-ptb", "p", "development", "dev", "discord-development", "d"];

#[derive(Clap, Debug)]
#[clap(version = "0.1.1", setting = AppSettings::ColoredHelp)]
struct Opts {
  #[clap(subcommand)]
  command: Command,
}

#[derive(Clap, Debug)]
enum Command {
  #[clap(about = "install the latest <type> of discord", aliases = &["i", "in", "get"])]
  Install(InstallOption),

  #[clap(about = "update to the latest <type> of discord", aliases = &["u", "up", "upgrade"])]
  Update(UpdateOption),

  #[clap(about = "remove the installed <type> of discord", aliases = &["r", "rm", "d", "del", "un", "uninstall"])]
  Remove(RemoveOption),

  #[clap(about = "show all installed versions", aliases = &["s", "installed", "all", "versions", "types"])]
  Show
}

#[derive(Clap, Debug)]
struct InstallOption {
  #[clap(possible_values = POSSIBLE_VALUES)]
  r#type: Vec<String>
}

#[derive(Clap, Debug)]
struct UpdateOption {
  #[clap(possible_values = POSSIBLE_VALUES)]
  r#type: Vec<String>
}

#[derive(Clap, Debug)]
struct RemoveOption {
  #[clap(possible_values = POSSIBLE_VALUES)]
  r#type: Vec<String>
}

fn str_to_type(s: String) -> Type {
  match s.as_str() {
    "stable" | "discord-stable" | "s" => Type::STABLE,
    "canary" | "discord-canary" | "c" => Type::CANARY,
    "ptb" | "discord-ptb" | "p" => Type::PTB,
    "development" | "dev" | "discord-development" | "d" => Type::DEVELOPMENT,
    _ => {
      error(format!("type \"{}\" does not exist", s));
      std::process::exit(1);
    }
  }
}


#[tokio::main]
async fn main() -> Res<()> {
  let opts = Opts::parse();

  match opts.command {
    Command::Install(opt) => {
      for r#type in opt.r#type {
        install(str_to_type(r#type)).await?;
      }
    }
    Command::Update(opt) => {
      for r#type in opt.r#type {
        update(str_to_type(r#type)).await?;
      }
    }
    Command::Remove(opt) => {
      for r#type in opt.r#type {
        remove(str_to_type(r#type)).await?;
      }
    }
    Command::Show => {
      show().await?;
    }
  };

  Ok(())
}
