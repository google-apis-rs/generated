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
        let mut app = App::new("pubsub1_beta1a")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200505")
            .about("Provides reliable, many-to-many, asynchronous messaging between applications.\n")
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
        let mut subscriptions0 = SubCommand::with_name("subscriptions")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: acknowledge, create, delete, get, list, modify_ack_deadline, modify_push_config, pull and pull_batch");
        {
            let mcmd = SubCommand::with_name("acknowledge").about("Acknowledges a particular received message: the Pub/Sub system can remove\nthe given message from the subscription. Acknowledging a message whose\nAck deadline has expired may succeed, but the message could have been\nalready redelivered. Acknowledging a message more than once will not\nresult in an error. This is only used for messages received via pull.");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a subscription on a given topic for a given subscriber.\nIf the subscription already exists, returns ALREADY_EXISTS.\nIf the corresponding topic doesn\'t exist, returns NOT_FOUND.\n\nIf the name is not provided in the request, the server will assign a random\nname for this subscription on the same project as the topic.");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an existing subscription. All pending messages in the subscription\nare immediately dropped. Calls to Pull after deletion will return\nNOT_FOUND.");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the configuration details of a subscription.");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists matching subscriptions.");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("modify_ack_deadline")
                .about("Modifies the Ack deadline for a message received from a pull request.");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("modify_push_config").about("Modifies the <code>PushConfig</code> for a specified subscription.\nThis method can be used to suspend the flow of messages to an endpoint\nby clearing the <code>PushConfig</code> field in the request. Messages\nwill be accumulated for delivery even if no push configuration is\ndefined or while the configuration is modified.");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("pull").about("Pulls a single message from the server.\nIf return_immediately is true, and no messages are available in the\nsubscription, this method returns FAILED_PRECONDITION. The system is free\nto return an UNAVAILABLE error if no messages are available in a\nreasonable amount of time (to reduce system load).");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("pull_batch").about("Pulls messages from the server. Returns an empty list if there are no\nmessages available in the backlog. The system is free to return UNAVAILABLE\nif there are too many pull requests outstanding for the given subscription.");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        let mut topics0 = SubCommand::with_name("topics")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, publish and publish_batch");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates the given topic with the given name.");
            topics0 = topics0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the topic with the given name. Returns NOT_FOUND if the topic does\nnot exist. After a topic is deleted, a new topic may be created with the\nsame name.");
            topics0 = topics0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the configuration of a topic. Since the topic only has the name\nattribute, this method is only useful to check the existence of a topic.\nIf other attributes are added in the future, they will be returned here.");
            topics0 = topics0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists matching topics.");
            topics0 = topics0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("publish").about(
                "Adds a message to the topic.  Returns NOT_FOUND if the topic does not\nexist.",
            );
            topics0 = topics0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("publish_batch").about("Adds one or more messages to the topic. Returns NOT_FOUND if the topic does\nnot exist.");
            topics0 = topics0.subcommand(mcmd);
        }
        app = app.subcommand(topics0);
        app = app.subcommand(subscriptions0);

        Self { app }
    }
}
use google_pubsub1_beta1a as api;

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
