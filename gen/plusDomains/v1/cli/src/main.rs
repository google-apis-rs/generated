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
        let mut app = App::new("plusDomains1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190616")
            .about("Builds on top of the Google+ platform for Google Apps Domains.")
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
        let mut activities0 = SubCommand::with_name("activities")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about(
                "Shut down. See https://developers.google.com/+/api-shutdown for more details.",
            );
            activities0 = activities0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Shut down. See https://developers.google.com/+/api-shutdown for more details.",
            );
            activities0 = activities0.subcommand(mcmd);
        }
        let mut audiences0 = SubCommand::with_name("audiences")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about(
                "Shut down. See https://developers.google.com/+/api-shutdown for more details.",
            );
            audiences0 = audiences0.subcommand(mcmd);
        }
        let mut circles0 = SubCommand::with_name("circles")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about(
                "Shut down. See https://developers.google.com/+/api-shutdown for more details.",
            );
            circles0 = circles0.subcommand(mcmd);
        }
        let mut comments0 = SubCommand::with_name("comments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about(
                "Shut down. See https://developers.google.com/+/api-shutdown for more details.",
            );
            comments0 = comments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Shut down. See https://developers.google.com/+/api-shutdown for more details.",
            );
            comments0 = comments0.subcommand(mcmd);
        }
        let mut media0 = SubCommand::with_name("media")
            .setting(AppSettings::ColoredHelp)
            .about("methods: insert");
        {
            let mcmd = SubCommand::with_name("insert").about(
                "Shut down. See https://developers.google.com/+/api-shutdown for more details.",
            );
            media0 = media0.subcommand(mcmd);
        }
        let mut people0 = SubCommand::with_name("people")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list and list_by_activity");
        {
            let mcmd = SubCommand::with_name("get").about("Get a person\'s profile.");
            people0 = people0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all of the people in the specified collection.");
            people0 = people0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_by_activity").about(
                "Shut down. See https://developers.google.com/+/api-shutdown for more details.",
            );
            people0 = people0.subcommand(mcmd);
        }
        app = app.subcommand(people0);
        app = app.subcommand(media0);
        app = app.subcommand(comments0);
        app = app.subcommand(circles0);
        app = app.subcommand(audiences0);
        app = app.subcommand(activities0);

        Self { app }
    }
}
use google_plusDomains1 as api;

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
