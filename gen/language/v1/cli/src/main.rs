use clap::{App, AppSettings, Arg, SubCommand};
use default_boxed::DefaultBoxed;

#[derive(DefaultBoxed)]
struct Outer<'a, 'b> {
    inner: HeapApp<'a, 'b>,
}

struct HeapApp<'a, 'b> {
    app: App<'a, 'b>,
}

impl<'a, 'b> Default for HeapApp<'a, 'b> {
    fn default() -> Self {
        let mut app = App::new("language1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190913")
            .about("Provides natural language understanding technologies, such as sentiment analysis, entity recognition, entity sentiment analysis, and other text annotations, to developers.")
            .after_help("All documentation details can be found at <TODO figure out URL>")
            .arg(Arg::with_name("scope")
                .long("scope")
                .help("Specify the authentication method should be executed in. Each scope requires the user to grant this application permission to use it. If unset, it defaults to the shortest scope url for a particular method.")
                .multiple(true)
                .takes_value(true))
            .arg(Arg::with_name("folder")
                .long("config-dir")
                .help("A directory into which we will store our persistent data. Defaults to a user-writable directory that we will create during the first invocation." )
                .multiple(false)
                .takes_value(true))
            .arg(Arg::with_name("debug")
                .long("debug")
                .help("Provide more output to aid with debugging")
                .multiple(false)
                .takes_value(false));
        let mut documents0 = SubCommand::with_name("documents")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: analyze_entities, analyze_entity_sentiment, analyze_sentiment, analyze_syntax, annotate_text and classify_text");
        {
            let mcmd = SubCommand::with_name("analyze_entities").about("Finds named entities (currently proper names and common nouns) in the text\nalong with entity types, salience, mentions for each entity, and\nother properties.");
            documents0 = documents0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("analyze_entity_sentiment").about("Finds entities, similar to AnalyzeEntities in the text and analyzes\nsentiment associated with each entity and its mentions.");
            documents0 = documents0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("analyze_sentiment")
                .about("Analyzes the sentiment of the provided text.");
            documents0 = documents0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("analyze_syntax").about("Analyzes the syntax of the text and provides sentence boundaries and\ntokenization along with part of speech tags, dependency trees, and other\nproperties.");
            documents0 = documents0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("annotate_text").about("A convenience method that provides all the features that analyzeSentiment,\nanalyzeEntities, and analyzeSyntax provide in one call.");
            documents0 = documents0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("classify_text")
                .about("Classifies a document into categories.");
            documents0 = documents0.subcommand(mcmd);
        }
        app = app.subcommand(documents0);

        Self { app }
    }
}
use google_language1 as api;

fn main() {
    // TODO: set homedir afterwards, once the address is unmovable, or use Pin for the very first time
    // to allow a self-referential structure :D!
    let _home_dir = dirs::config_dir()
        .expect("configuration directory can be obtained")
        .join("google-service-cli");
    let outer = Outer::default_boxed();
    let app = outer.inner.app;
    let _matches = app.get_matches();
}
