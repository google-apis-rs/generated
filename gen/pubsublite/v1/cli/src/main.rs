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
        let mut app = App::new("pubsublite1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210308")
            .about("")
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
        let mut admin0 = SubCommand::with_name("admin")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: projects");
        let mut cursor0 = SubCommand::with_name("cursor")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: projects");
        let mut topic_stats0 = SubCommand::with_name("topic_stats")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: projects");
        let mut projects1 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations");
        let mut projects1 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations");
        let mut projects1 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations");
        let mut locations2 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: subscriptions and topics");
        let mut locations2 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: subscriptions");
        let mut locations2 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: topics");
        let mut subscriptions3 = SubCommand::with_name("subscriptions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new subscription.");
            subscriptions3 = subscriptions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified subscription.");
            subscriptions3 = subscriptions3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Returns the subscription configuration.");
            subscriptions3 = subscriptions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of subscriptions for the given project.");
            subscriptions3 = subscriptions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates properties of the specified subscription.");
            subscriptions3 = subscriptions3.subcommand(mcmd);
        }
        let mut topics3 = SubCommand::with_name("topics")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, get_partitions, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new topic.");
            topics3 = topics3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified topic.");
            topics3 = topics3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the topic configuration.");
            topics3 = topics3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_partitions")
                .about("Returns the partition information for the requested topic.");
            topics3 = topics3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of topics for the given project.");
            topics3 = topics3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Updates properties of the specified topic.");
            topics3 = topics3.subcommand(mcmd);
        }
        let mut subscriptions3 = SubCommand::with_name("subscriptions")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: cursors");
        let mut topics3 = SubCommand::with_name("topics")
            .setting(AppSettings::ColoredHelp)
            .about("methods: compute_head_cursor and compute_message_stats");
        {
            let mcmd = SubCommand::with_name("compute_head_cursor").about("Compute the head cursor for the partition. The head cursor\'s offset is guaranteed to be less than or equal to all messages which have not yet been acknowledged as published, and greater than the offset of any message whose publish has already been acknowledged. It is zero if there have never been messages in the partition.");
            topics3 = topics3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("compute_message_stats").about(
                "Compute statistics about a range of messages in a given topic and partition.",
            );
            topics3 = topics3.subcommand(mcmd);
        }
        let mut subscriptions4 = SubCommand::with_name("subscriptions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the subscriptions attached to the specified topic.");
            subscriptions4 = subscriptions4.subcommand(mcmd);
        }
        let mut cursors4 = SubCommand::with_name("cursors")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns all committed cursor information for a subscription.");
            cursors4 = cursors4.subcommand(mcmd);
        }
        subscriptions3 = subscriptions3.subcommand(cursors4);
        topics3 = topics3.subcommand(subscriptions4);
        locations2 = locations2.subcommand(topics3);
        locations2 = locations2.subcommand(subscriptions3);
        locations2 = locations2.subcommand(topics3);
        locations2 = locations2.subcommand(subscriptions3);
        projects1 = projects1.subcommand(locations2);
        projects1 = projects1.subcommand(locations2);
        projects1 = projects1.subcommand(locations2);
        topic_stats0 = topic_stats0.subcommand(projects1);
        cursor0 = cursor0.subcommand(projects1);
        admin0 = admin0.subcommand(projects1);
        app = app.subcommand(topic_stats0);
        app = app.subcommand(cursor0);
        app = app.subcommand(admin0);

        Self { app }
    }
}
use google_pubsublite1 as api;

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
