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
        let mut app = App::new("manufacturers1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200428")
            .about("Public API for managing Manufacturer Center related data.")
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
            .about("sub-resources: products");
        let mut products1 = SubCommand::with_name("products")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the product from a Manufacturer Center account.");
            products1 = products1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the product from a Manufacturer Center account, including product\nissues.\n\nA recently updated product takes around 15 minutes to process. Changes are\nonly visible after it has been processed. While some issues may be\navailable once the product has been processed, other issues may take days\nto appear.");
            products1 = products1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all the products in a Manufacturer Center account.");
            products1 = products1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Inserts or updates the attributes of the product in a Manufacturer Center\naccount.\n\nCreates a product with the provided attributes. If the product already\nexists, then all attributes are replaced with the new ones. The checks at\nupload time are minimal. All required attributes need to be present for a\nproduct to be valid. Issues may show up later after the API has accepted a\nnew upload for a product and it is possible to overwrite an existing valid\nproduct with an invalid product. To detect this, you should retrieve the\nproduct and check it for issues once the new version is available.\n\nUploaded attributes first need to be processed before they can be\nretrieved. Until then, new products will be unavailable, and retrieval\nof previously uploaded products will return the original state of the\nproduct.");
            products1 = products1.subcommand(mcmd);
        }
        accounts0 = accounts0.subcommand(products1);
        app = app.subcommand(accounts0);

        Self { app }
    }
}
use google_manufacturers1 as api;

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
