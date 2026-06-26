//! Unified CLI for Voxtral ASR and TTS.
//!
//! ```text
//! voxtral transcribe --audio file.wav [--gguf model.gguf | --model dir/]
//! voxtral speak --text "Hello" --voice casual_female [--gguf model.gguf | --model dir/]
//! ```

mod speak;
mod transcribe;
//use std::fmt::{Error }; //, Write};
//use std::io::{Write};

//use crossterm::style::Stylize;
//use tracing::{subscriber::Subscriber, Event};
//use tracing_log::NormalizeEvent;
/*
use tracing_subscriber::{
    fmt::{
        format::{Writer},
        //time::{ChronoLocal, FormatTime},
        FmtContext, FormatEvent, FormatFields
    },
    registry::LookupSpan,
};
*/
use clap::{Parser, Subcommand};

#[macro_use]
mod eko;


#[derive(Parser)]
#[command(name = "voxtral")]
#[command(about = "Voxtral speech recognition and synthesis")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Transcribe audio to text (ASR)
    Transcribe(transcribe::Args),
    /// Synthesize speech from text (TTS)
    Speak(speak::Args),
}

fn main() -> anyhow::Result<()> {
    /*
    tracing_subscriber::fmt()
        .with_target(false)
        .pretty()        
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .init();
    */
    //tracing_subscriber::fmt().event_format(SimpleFmt).init();
    let i = 123;
    EKO!("abc");
    EKO!(i);
    let cli = Cli::parse();
    match cli.command {
        Command::Transcribe(args) => transcribe::run(args),
        Command::Speak(args) => speak::run(args),
    }
}

/*
struct SimpleFmt;

impl<S, N> FormatEvent<S, N> for SimpleFmt
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer:   Writer<'_>,
        event: &Event<'_>,
    ) -> Result<(), Error> {
        // Create timestamp
        //let time_format = "%b %d %I:%M:%S%.6f %p";
        let time_format = "%I:%M:%S% %p";
        //let mut time_now = String::new();

        let time_now = chrono::Local::now()
            .format(time_format)
            .to_string();


        
        //ChronoLocal::new(time_format.into()).format_time(&mut time_now)?;

        // Get line numbers from log crate events
        let normalized_meta = event.normalized_metadata();
        let meta = normalized_meta.as_ref().unwrap_or_else(|| event.metadata());

        // Write formatted log record
        let message = format!(
            "{}:{}: [{} {}]",
            meta.file().unwrap_or("").to_string(), //.yellow(),
            //String::from(":"), //.yellow(),
            meta.line().unwrap_or(0).to_string(), //.yellow(),
            meta.level().to_string(), //.blue(),
            time_now, //.grey(),
            
        );
        write!(writer, "{}", message).unwrap();
        ctx.format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}
*/
