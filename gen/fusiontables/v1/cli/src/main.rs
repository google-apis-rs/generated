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
        let mut app = App::new("fusiontables1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20171117")
            .about("API for working with Fusion Tables data.")
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
        let mut column0 = SubCommand::with_name("column")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the column.");
            column0 = column0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a specific column by its id.");
            column0 = column0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Adds a new column to the table.");
            column0 = column0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of columns.");
            column0 = column0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the name or type of an existing column. This method supports patch semantics.");
            column0 = column0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates the name or type of an existing column.");
            column0 = column0.subcommand(mcmd);
        }
        let mut query0 = SubCommand::with_name("query")
            .setting(AppSettings::ColoredHelp)
            .about("methods: sql and sql_get");
        {
            let mcmd = SubCommand::with_name("sql").about(
                "Executes an SQL SELECT/INSERT/UPDATE/DELETE/SHOW/DESCRIBE/CREATE statement.",
            );
            query0 = query0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("sql_get")
                .about("Executes an SQL SELECT/SHOW/DESCRIBE statement.");
            query0 = query0.subcommand(mcmd);
        }
        let mut style0 = SubCommand::with_name("style")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a style.");
            style0 = style0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a specific style.");
            style0 = style0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Adds a new style for the table.");
            style0 = style0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of styles.");
            style0 = style0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates an existing style. This method supports patch semantics.");
            style0 = style0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing style.");
            style0 = style0.subcommand(mcmd);
        }
        let mut table0 = SubCommand::with_name("table")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: copy, delete, get, import_rows, import_table, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("copy").about("Copies a table.");
            table0 = table0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a table.");
            table0 = table0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a specific table by its id.");
            table0 = table0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import_rows").about("Import more rows into a table.");
            table0 = table0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import_table").about("Import a new table.");
            table0 = table0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a new table.");
            table0 = table0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Retrieves a list of tables a user owns.");
            table0 = table0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing table. Unless explicitly requested, only the name, description, and attribution will be updated. This method supports patch semantics.");
            table0 = table0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing table. Unless explicitly requested, only the name, description, and attribution will be updated.");
            table0 = table0.subcommand(mcmd);
        }
        let mut task0 = SubCommand::with_name("task")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get and list");
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes the task, unless already started.");
            task0 = task0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a specific task by its id.");
            task0 = task0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of tasks.");
            task0 = task0.subcommand(mcmd);
        }
        let mut template0 = SubCommand::with_name("template")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a template");
            template0 = template0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Retrieves a specific template by its id");
            template0 = template0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("insert").about("Creates a new template for the table.");
            template0 = template0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of templates.");
            template0 = template0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates an existing template. This method supports patch semantics.");
            template0 = template0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing template");
            template0 = template0.subcommand(mcmd);
        }
        app = app.subcommand(template0);
        app = app.subcommand(task0);
        app = app.subcommand(table0);
        app = app.subcommand(style0);
        app = app.subcommand(query0);
        app = app.subcommand(column0);

        Self { app }
    }
}
use google_fusiontables1 as api;

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
