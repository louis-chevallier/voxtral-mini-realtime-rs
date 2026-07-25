use voxtral_mini_realtime::EKO;
//use lib_epub::{epub::EpubDoc};
use anyhow::{Result};
use cli_epub_to_text::epub_to_text;

use std::fmt::{Error }; //, Write};
//use std::io::{Write};
//use crossterm::style::Stylize;
use tracing::{subscriber::Subscriber, Event};
use tracing_log::NormalizeEvent;

use tracing_subscriber::{
    fmt::{
        format::{Writer},
        //time::{ChronoLocal, FormatTime},
        FmtContext, FormatEvent, FormatFields
    },
    registry::LookupSpan,
};

use tracing::info;

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
        let mut time_format = "%I:%M:%S %p";
        //time_format = "%I:%M:%S%";
        time_format = "%H:%M:%S";        
        //let mut time_now = String::new();
        EKO!();
        let time_now1 = chrono::Local::now();
        EKO!();

        //let mut formatted1 = String::new();
        
        let formatted = format!("{}", time_now1.format(time_format));
        EKO!(formatted);


        let fr = time_now1.format(time_format);

        //let string33: String::from(fr);
        //EKO!(string33);
        

        //EKO!(&fr);

        let _tn = fr.to_string();

        //EKO!(tn);
        //let time_now = "xxx";
        //ChronoLocal::new(time_format.into()).format_time(&mut time_now)?;
        //EKO!();

        // Get line numbers from log crate events
        let normalized_meta = event.normalized_metadata();
        let meta = normalized_meta.as_ref().unwrap_or_else(|| event.metadata());
        //EKO!();

        // Write formatted log record
        let message = format!(
            "{}:{}: [{} {}]",
            meta.file().unwrap_or("").to_string(), //.yellow(),
            //String::from(":"), //.yellow(),
            meta.line().unwrap_or(0).to_string(), //.yellow(),
            meta.level().to_string(), //.blue(),
            _tn, //.grey(),
            
        );
        //EKO!();
        
        write!(writer, "{}", message).unwrap();
        ctx.format_fields(writer.by_ref(), event)?;
        //EKO!();
        
        writeln!(writer)
    }
}

use epub::doc::EpubDoc;

pub fn xx()  -> Result<()> {

    let mut epub_file = "/mnt/NUC/www/books/Les camarades  -- Remarque, Erich Maria -- 2013 -- Folio.epub";
    epub_file = "Loti-prime-jeunesse.epub";
    EKO!(epub_file);

    match epub_to_text(epub_file) {
        Ok(text) => println!("Extracted text: {}", text),
        Err(e) => eprintln!("Error: {}", e),
    }

    let doc = EpubDoc::new(epub_file);
    assert!(doc.is_ok());
    let mut doc = doc.unwrap();



    doc.go_next();
    EKO!(doc.get_current_id().unwrap());
    while 1>0 {
        doc.go_next();
        EKO!(doc.get_current());
        let bytes = doc.get_current().unwrap().0.to_vec();
        let string = String::from_utf8(bytes).expect("Our bytes should be valid utf8");
        EKO!(&string);
        println!("{string}");
    }
    /*
    let result = EpubDoc::new(epub_file);
    
    match result {
        Ok(_value) => println!("ok"),
        Err(e) => println!("Error: {}", e),
    }
    EKO!();
    let doc = EpubDoc::new(epub_file)?;    
    // Get metadata
    EKO!(doc.get_title());
    EKO!(doc.get_metadata_value("creator"));
    
    // Read content
    if let Some((_content, _mime)) = doc.spine_current() { EKO!(_content); };
    if let Some((_content, _mime)) = doc.spine_next() { EKO!(_content);  };
    */
    Ok(())        

}
fn main() {
    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    //tracing_subscriber::fmt().with_ansi(false).init();
    tracing_subscriber::fmt().with_ansi(false).event_format(SimpleFmt).init();


    
    //let _ = xx();
    
    let i = 123;
    let j = 456;
    let abc = "abc";

    info!("starting");
    info!(i);



    
    EKO!();
    EKO!(i);
    EKO!("xyz");
    EKO!([i, j]);
    EKO!(abc);
    EKO!(abc, i);
    EKO!(abc, i, "toto", j);
    EKO!("Hello World!");
}
