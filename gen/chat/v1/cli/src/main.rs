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
        let mut app = App::new("chat1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200508")
            .about("Enables bots to fetch information and perform actions in Hangouts Chat.")
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
        let mut spaces0 = SubCommand::with_name("spaces")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Returns a space.");
            spaces0 = spaces0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists spaces the caller is a member of.");
            spaces0 = spaces0.subcommand(mcmd);
        }
        let mut members1 = SubCommand::with_name("members")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Returns a membership.");
            members1 = members1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists human memberships in a space.");
            members1 = members1.subcommand(mcmd);
        }
        let mut messages1 = SubCommand::with_name("messages")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a message.");
            messages1 = messages1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a message.");
            messages1 = messages1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns a message.");
            messages1 = messages1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a message.");
            messages1 = messages1.subcommand(mcmd);
        }
        spaces0 = spaces0.subcommand(messages1);
        spaces0 = spaces0.subcommand(members1);
        app = app.subcommand(spaces0);

        Self { app }
    }
}
use google_chat1 as api;

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
