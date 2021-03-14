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
            .version("0.1.0-20210307")
            .about("Enables bots to fetch information and perform actions in Google Chat.")
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
        let mut dms0 = SubCommand::with_name("dms")
            .setting(AppSettings::ColoredHelp)
            .about("methods: messages and webhooks");
        {
            let mcmd = SubCommand::with_name("messages").about("Legacy path for creating message. Calling these will result in a BadRequest response.");
            dms0 = dms0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("webhooks").about("Legacy path for creating message. Calling these will result in a BadRequest response.");
            dms0 = dms0.subcommand(mcmd);
        }
        let mut media0 = SubCommand::with_name("media")
            .setting(AppSettings::ColoredHelp)
            .about("methods: download");
        {
            let mcmd = SubCommand::with_name("download").about(
                "Downloads media. Download is supported on the URI `/v1/media/{+name}?alt=media`.",
            );
            media0 = media0.subcommand(mcmd);
        }
        let mut rooms0 = SubCommand::with_name("rooms")
            .setting(AppSettings::ColoredHelp)
            .about("methods: messages and webhooks");
        {
            let mcmd = SubCommand::with_name("messages").about("Legacy path for creating message. Calling these will result in a BadRequest response.");
            rooms0 = rooms0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("webhooks").about("Legacy path for creating message. Calling these will result in a BadRequest response.");
            rooms0 = rooms0.subcommand(mcmd);
        }
        let mut spaces0 = SubCommand::with_name("spaces")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list and webhooks");
        {
            let mcmd = SubCommand::with_name("get").about("Returns a space.");
            spaces0 = spaces0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists spaces the caller is a member of.");
            spaces0 = spaces0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("webhooks").about("Legacy path for creating message. Calling these will result in a BadRequest response.");
            spaces0 = spaces0.subcommand(mcmd);
        }
        let mut conversations1 = SubCommand::with_name("conversations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: messages");
        {
            let mcmd = SubCommand::with_name("messages").about("Legacy path for creating message. Calling these will result in a BadRequest response.");
            conversations1 = conversations1.subcommand(mcmd);
        }
        let mut conversations1 = SubCommand::with_name("conversations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: messages");
        {
            let mcmd = SubCommand::with_name("messages").about("Legacy path for creating message. Calling these will result in a BadRequest response.");
            conversations1 = conversations1.subcommand(mcmd);
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
        let mut attachments2 = SubCommand::with_name("attachments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the metadata of a message attachment. The attachment data is fetched using the media API.");
            attachments2 = attachments2.subcommand(mcmd);
        }
        messages1 = messages1.subcommand(attachments2);
        spaces0 = spaces0.subcommand(messages1);
        spaces0 = spaces0.subcommand(members1);
        rooms0 = rooms0.subcommand(conversations1);
        dms0 = dms0.subcommand(conversations1);
        app = app.subcommand(spaces0);
        app = app.subcommand(rooms0);
        app = app.subcommand(media0);
        app = app.subcommand(dms0);

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
