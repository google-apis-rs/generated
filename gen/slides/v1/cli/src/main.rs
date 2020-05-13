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
        let mut app = App::new("slides1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200508")
            .about("Reads and writes Google Slides presentations.")
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
        let mut presentations0 = SubCommand::with_name("presentations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_update, create and get");
        {
            let mcmd = SubCommand::with_name("batch_update").about("Applies one or more updates to the presentation.\n\nEach request is validated before\nbeing applied. If any request is not valid, then the entire request will\nfail and nothing will be applied.\n\nSome requests have replies to\ngive you some information about how they are applied. Other requests do\nnot need to return information; these each return an empty reply.\nThe order of replies matches that of the requests.\n\nFor example, suppose you call batchUpdate with four updates, and only the\nthird one returns information. The response would have two empty replies:\nthe reply to the third request, and another empty reply, in that order.\n\nBecause other users may be editing the presentation, the presentation\nmight not exactly reflect your changes: your changes may\nbe altered with respect to collaborator changes. If there are no\ncollaborators, the presentation should reflect your changes. In any case,\nthe updates in your request are guaranteed to be applied together\natomically.");
            presentations0 = presentations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a blank presentation using the title given in the request. If a\n`presentationId` is provided, it is used as the ID of the new presentation.\nOtherwise, a new ID is generated. Other fields in the request, including\nany provided content, are ignored.\nReturns the created presentation.");
            presentations0 = presentations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the latest version of the specified presentation.");
            presentations0 = presentations0.subcommand(mcmd);
        }
        let mut pages1 = SubCommand::with_name("pages")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and get_thumbnail");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the latest version of the specified page in the presentation.");
            pages1 = pages1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_thumbnail").about("Generates a thumbnail of the latest version of the specified page in the\npresentation and returns a URL to the thumbnail image.\n\nThis request counts as an [expensive read request](/slides/limits) for\nquota purposes.");
            pages1 = pages1.subcommand(mcmd);
        }
        presentations0 = presentations0.subcommand(pages1);
        app = app.subcommand(presentations0);

        Self { app }
    }
}
use google_slides1 as api;

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
