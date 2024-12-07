use std::{borrow::Cow, process};

use clap::{value_parser, Arg, ArgMatches, Command};

use crate::{Day, Variant};

pub fn main<D: Day>(default_input: &str) -> ! {
    let mut part1 = Command::new("part1").about("Runs the part 1 program");
    let mut part2 = Command::new("part2").about("Runs the part 2 program");

    part1 = create_variant_subcommands(part1, &D::part1().variants);
    part2 = create_variant_subcommands(part2, &D::part2().variants);

    let mut typename = std::any::type_name::<D>().split("::").collect::<Vec<_>>();
    let typename = typename.pop().unwrap();
    let cmd = Command::new(typename.to_lowercase())
        .about(format!(
            "Program to run the AOC answer for day {}",
            typename.strip_prefix("Day").unwrap()
        ))
        .subcommand_required(true)
        .subcommand(part1)
        .subcommand(part2);

    let matches = cmd.clone().get_matches();

    match matches.subcommand() {
        Some(("part1", matches)) => {
            let variants = D::part1().variants;
            dispatch_root_subcommand::<D>(default_input, &variants, matches);
        }
        Some(("part2", matches)) => {
            let variants = D::part2().variants;
            dispatch_root_subcommand::<D>(default_input, &variants, matches);
        }
        _ => {
            unreachable!("subcommand_required")
        }
    }
}

fn create_variant_subcommands(mut part: Command, variants: &[Variant]) -> Command {
    if variants.len() > 1 {
        part = part.subcommand_required(true);

        variants
            .iter()
            .map(|v| {
                Command::new(v.name)
                    .about(format!("Run the {} variant", v.name))
                    .arg(Arg::new("input").short('i').long("input"))
                    .arg(Arg::new("iter").long("iter").value_parser(value_parser!(usize)))
            })
            .for_each(|cmd| part = part.clone().subcommand(cmd));
    } else {
        part = part
            .arg(Arg::new("input").short('i').long("input"))
            .arg(Arg::new("iter").long("iter").value_parser(value_parser!(usize)));
    }

    part
}

fn dispatch_root_subcommand<D: Day>(
    default_input: &str,
    variants: &[Variant],
    matches: &ArgMatches,
) -> ! {
    let iter = matches.get_one::<usize>("iter").unwrap_or(&1);

    if variants.len() > 1 {
        let subcommand = matches.subcommand().unwrap();
        let variant = variants.iter().find(|v| v.name == subcommand.0).unwrap();
        let input = get_input(subcommand.1, default_input);
        execute::<D>(variant, &input, *iter);
    } else {
        let input = get_input(matches, default_input);
        execute::<D>(&variants[0], &input, *iter);
    }
}

fn execute<D: Day>(variant: &Variant, input: &str, iter: usize) -> ! {
    use std::io::Write;
    let input = D::pad_input(input);
    let mut result = 0;
    for _ in 0..iter {
        result = (variant.f)(&input);
    }
    let err = writeln!(std::io::stdout(), "{result}");
    if let Err(err) = err {
        if err.kind() != std::io::ErrorKind::BrokenPipe {
            eprintln!("error: {err}");
            process::exit(1);
        }
    }
    process::exit(0);
}

fn get_input<'a>(matches: &ArgMatches, default: &'a str) -> Cow<'a, str> {
    matches
        .get_one::<String>("input")
        .map(|input| {
            Cow::Owned(std::fs::read_to_string(input).unwrap_or_else(|err| {
                eprintln!("error: failed to read file {input}: {err}");
                process::exit(1);
            }))
        })
        .unwrap_or(Cow::Borrowed(default))
}
