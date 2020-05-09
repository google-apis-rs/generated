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
        let mut app = App::new("digitalassetlinks1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200505")
            .about("Discovers relationships between online assets such as websites or mobile apps.")
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
        let mut assetlinks0 = SubCommand::with_name("assetlinks")
            .setting(AppSettings::ColoredHelp)
            .about("methods: check");
        {
            let mcmd = SubCommand::with_name("check").about("Determines whether the specified (directional) relationship exists between\nthe specified source and target assets.\n\nThe relation describes the intent of the link between the two assets as\nclaimed by the source asset.  An example for such relationships is the\ndelegation of privileges or permissions.\n\nThis command is most often used by infrastructure systems to check\npreconditions for an action.  For example, a client may want to know if it\nis OK to send a web URL to a particular mobile app instead. The client can\ncheck for the relevant asset link from the website to the mobile app to\ndecide if the operation should be allowed.\n\nA note about security: if you specify a secure asset as the source, such as\nan HTTPS website or an Android app, the API will ensure that any\nstatements used to generate the response have been made in a secure way by\nthe owner of that asset.  Conversely, if the source asset is an insecure\nHTTP website (that is, the URL starts with `http://` instead of\n`https://`), the API cannot verify its statements securely, and it is not\npossible to ensure that the website\'s statements have not been altered by a\nthird party.  For more information, see the [Digital Asset Links technical\ndesign\nspecification](https://github.com/google/digitalassetlinks/blob/master/well-known/details.md).");
            assetlinks0 = assetlinks0.subcommand(mcmd);
        }
        let mut statements0 = SubCommand::with_name("statements")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of all statements from a given source that match the\nspecified target and statement string.\n\nThe API guarantees that all statements with secure source assets, such as\nHTTPS websites or Android apps, have been made in a secure way by the owner\nof those assets, as described in the [Digital Asset Links technical design\nspecification](https://github.com/google/digitalassetlinks/blob/master/well-known/details.md).\nSpecifically, you should consider that for insecure websites (that is,\nwhere the URL starts with `http://` instead of `https://`), this guarantee\ncannot be made.\n\nThe `List` command is most useful in cases where the API client wants to\nknow all the ways in which two assets are related, or enumerate all the\nrelationships from a particular source asset.  Example: a feature that\nhelps users navigate to related items.  When a mobile app is running on a\ndevice, the feature would make it easy to navigate to the corresponding web\nsite or Google+ profile.");
            statements0 = statements0.subcommand(mcmd);
        }
        app = app.subcommand(statements0);
        app = app.subcommand(assetlinks0);

        Self { app }
    }
}
use google_digitalassetlinks1 as api;

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
