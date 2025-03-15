use joylight_backend::setup_logger;
use reedline::{DefaultPrompt, Reedline, Signal};
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::prelude::*;
use log::*;

#[derive(Debug, Clone)]
enum Command {
    // Select(String),
    // Set(String, f64),
    Select(Vec<String>),
    Set(String),
    Toast,
    Numero(f64)
    // Toast(String),
}

fn parser() -> impl Parser<char, Command, Error = Simple<char>> {
    choice((
        text::keyword("select").padded().then(text::ident().padded().repeated().at_least(1)).map(|(_, s)| Command::Select(s)),
        text::keyword("set").padded().then(text::ident()).map(|(_, s)| Command::Set(s)),
    ))
}

fn main() {
    setup_logger();

    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::default();

    loop {
        let sig = line_editor.read_line(&prompt);
        match sig {
            Ok(Signal::Success(buffer)) => {
                debug!("Read line: {:?}", buffer);
                let result = parser().parse(buffer.clone());
                
                match result {
                    Ok(command) => {
                        println!("Command: {:#?}", command);
                    }
                    Err(errs) => {
                        for err in errs {
                            Report::build(ReportKind::Error, err.span())
                                .with_message(err.to_string())
                                .with_label(
                                    Label::new(err.span())
                                        .with_message::<String>(format!("{:?}", err.reason()))
                                        .with_color(Color::Red)
                                )
                                .finish()
                                .eprint(Source::from(buffer.clone()))
                                .unwrap();
                        }
                    }
                }

            }
            Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                println!("\nAborted!");
                break;
            }
            x => {
                println!("Event: {:?}", x);
            }
        }
    }
}