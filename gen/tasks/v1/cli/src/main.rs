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
        let mut app = App::new("tasks1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210316")
            .about("The Google Tasks API lets you manage your tasks and task lists.")
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
        let mut tasklists0 = SubCommand::with_name("tasklists")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the authenticated user\'s specified task list.");
            tasklists0 = tasklists0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns the authenticated user\'s specified task list.");
            tasklists0 = tasklists0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about(
                "Creates a new task list and adds it to the authenticated user\'s task lists.",
            );
            tasklists0 = tasklists0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns all the authenticated user\'s task lists.");
            tasklists0 = tasklists0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the authenticated user\'s specified task list. This method supports patch semantics.");
            tasklists0 = tasklists0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates the authenticated user\'s specified task list.");
            tasklists0 = tasklists0.subcommand(mcmd);
        }
        let mut tasks0 = SubCommand::with_name("tasks")
            .setting(AppSettings::ColoredHelp)
            .about("methods: clear, delete, get, insert, list, r#move, patch and update");
        {
            let mcmd = SubCommand::with_name("clear").about("Clears all completed tasks from the specified task list. The affected tasks will be marked as \'hidden\' and no longer be returned by default when retrieving all tasks for a task list.");
            tasks0 = tasks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified task from the task list.");
            tasks0 = tasks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified task.");
            tasks0 = tasks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Creates a new task on the specified task list.");
            tasks0 = tasks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns all tasks in the specified task list.");
            tasks0 = tasks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("r#move").about("Moves the specified task to another position in the task list. This can include putting it as a child task under a new parent and/or move it to a different position among its sibling tasks.");
            tasks0 = tasks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates the specified task. This method supports patch semantics.");
            tasks0 = tasks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the specified task.");
            tasks0 = tasks0.subcommand(mcmd);
        }
        app = app.subcommand(tasks0);
        app = app.subcommand(tasklists0);

        Self { app }
    }
}
use google_tasks1 as api;

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
