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
        let mut app = App::new("mirror1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190424")
            .about("Interacts with Glass users via the timeline.")
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
        let mut accounts0 = SubCommand::with_name("accounts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: insert");
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a new account for a user");
            accounts0 = accounts0.subcommand(mcmd);
        }
        let mut contacts0 = SubCommand::with_name("contacts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a contact.");
            contacts0 = contacts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a single contact by ID.");
            contacts0 = contacts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a new contact.");
            contacts0 = contacts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of contacts for the authenticated user.");
            contacts0 = contacts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates a contact in place. This method supports patch semantics.");
            contacts0 = contacts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a contact in place.");
            contacts0 = contacts0.subcommand(mcmd);
        }
        let mut locations0 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a single location by ID.");
            locations0 = locations0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Retrieves a list of locations for the user.");
            locations0 = locations0.subcommand(mcmd);
        }
        let mut settings0 = SubCommand::with_name("settings")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a single setting by ID.");
            settings0 = settings0.subcommand(mcmd);
        }
        let mut subscriptions0 = SubCommand::with_name("subscriptions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, insert, list and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a subscription.");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a new subscription.");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of subscriptions for the authenticated user and service.");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update").about("Updates an existing subscription in place.");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        let mut timeline0 = SubCommand::with_name("timeline")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a timeline item.");
            timeline0 = timeline0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a single timeline item by ID.");
            timeline0 = timeline0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("insert").about("Inserts a new item into the timeline.");
            timeline0 = timeline0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of timeline items for the authenticated user.");
            timeline0 = timeline0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates a timeline item in place. This method supports patch semantics.");
            timeline0 = timeline0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a timeline item in place.");
            timeline0 = timeline0.subcommand(mcmd);
        }
        let mut attachments1 = SubCommand::with_name("attachments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert and list");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes an attachment from a timeline item.");
            attachments1 = attachments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves an attachment on a timeline item by item ID and attachment ID.");
            attachments1 = attachments1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("insert").about("Adds a new attachment to a timeline item.");
            attachments1 = attachments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns a list of attachments for a timeline item.");
            attachments1 = attachments1.subcommand(mcmd);
        }
        timeline0 = timeline0.subcommand(attachments1);
        app = app.subcommand(timeline0);
        app = app.subcommand(subscriptions0);
        app = app.subcommand(settings0);
        app = app.subcommand(locations0);
        app = app.subcommand(contacts0);
        app = app.subcommand(accounts0);

        Self { app }
    }
}
use google_mirror1 as api;

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
