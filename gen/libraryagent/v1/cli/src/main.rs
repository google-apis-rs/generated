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
        let mut app = App::new("libraryagent1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190919")
            .about("A simple Google Example Library API.")
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
        let mut shelves0 = SubCommand::with_name("shelves")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets a shelf. Returns NOT_FOUND if the shelf does not exist.");
            shelves0 = shelves0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists shelves. The order is unspecified but deterministic. Newly created\nshelves will not necessarily be added to the end of this list.");
            shelves0 = shelves0.subcommand(mcmd);
        }
        let mut books1 = SubCommand::with_name("books")
            .setting(AppSettings::ColoredHelp)
            .about("methods: borrow, get, list and r#return");
        {
            let mcmd = SubCommand::with_name("borrow").about("Borrow a book from the library. Returns the book if it is borrowed\nsuccessfully. Returns NOT_FOUND if the book does not exist in the library.\nReturns quota exceeded error if the amount of books borrowed exceeds\nallocation quota in any dimensions.");
            books1 = books1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets a book. Returns NOT_FOUND if the book does not exist.");
            books1 = books1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists books in a shelf. The order is unspecified but deterministic. Newly\ncreated books will not necessarily be added to the end of this list.\nReturns NOT_FOUND if the shelf does not exist.");
            books1 = books1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("r#return").about("Return a book to the library. Returns the book if it is returned to the\nlibrary successfully.\nReturns error if the book does not belong to the library\nor the users didn\'t borrow before.");
            books1 = books1.subcommand(mcmd);
        }
        shelves0 = shelves0.subcommand(books1);
        app = app.subcommand(shelves0);

        Self { app }
    }
}
use google_libraryagent1 as api;

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
