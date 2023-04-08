use std::fs;

use clap::{Arg, ArgAction, Command};
use colorful::{Color, Colorful};
use netwide_compiler::NetWideCompiler;

#[tokio::main]
async fn main() {
    let matches = Command::new("Netwide Compiler")
        .about("Execute your code on the cloud")
        .version("v1.0")
        .arg_required_else_help(true)
        .arg(
            Arg::new("language")
                .action(ArgAction::Set)
                .required(true)
                .help("Language of the code type list for getting a list of languages"),
        )
        .arg(
            Arg::new("file path")
                .short('f')
                .long("file")
                .action(ArgAction::Set)
                .required(false)
                .help("[required] Input code file"),
        )
        .arg(
            Arg::new("target")
                .short('t')
                .long("target")
                .action(ArgAction::Set)
                .required(false)
                .help("Target"),
        )
        .arg(
            Arg::new("list targets")
                .long("list-targets")
                .required(false)
                .action(ArgAction::SetTrue)
                .help("List all the available targets for a specific language"),
        )
        .get_matches();

    let lang = matches.get_one::<String>("language").unwrap();
    let file_path = matches.get_one::<String>("file path");
    let target = matches.get_one::<String>("target");
    let list_target = *matches.get_one::<bool>("list targets").unwrap();

    let com = NetWideCompiler::new().await;

    if lang == "list" {
        println!("List of languages:");
        com.get_langs().iter().for_each(|l| println!("{l}"));
    } else if list_target {
        println!("List of targets for '{lang}':");
        com.get_targets(lang)
            .unwrap()
            .iter()
            .for_each(|l| println!("{l}"));
    } else {
        if file_path.is_none() {
            eprintln!("No input file, consider giving a file `--file <file path>`");
            std::process::exit(1);
        }

        let file_contents = fs::read_to_string(file_path.unwrap()).unwrap_or_else(|_| {
            eprintln!(
                "[{}]: File '{}' not found",
                "Error".color(Color::Red),
                file_path.unwrap()
            );
            std::process::exit(1);
        });

        let target = match target {
            Some(x) => Some(x.as_str()),
            None => None,
        };

        let output = com.run(&lang, target, file_contents).await.unwrap();

        println!(
            "Program output =====\n{}\n=====\n",
            if output.program_stdout.is_empty() {
                "No output".color(Color::Grey100).to_string()
            } else {
                output.program_stdout
            }
            .trim()
                .bold()
        );

        if !output.program_stderr.is_empty() {
            println!(
                "Program stderr =====\n{}\n=====\n",
                output.program_stderr.color(Color::LightRed)
            );
        }

        if !output.compiler_stdout.is_empty() || !output.compiler_stderr.is_empty() {
            println!(
                "Compiler output =====\n{}\n{}\n=====",
                output.compiler_stderr.color(Color::Red),
                output.compiler_stdout
            );
        }
    }
}
