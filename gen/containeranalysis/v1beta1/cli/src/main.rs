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
        let mut app = App::new("containeranalysis1_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210305")
            .about("An implementation of the Grafeas API, which stores, and enables querying and retrieval of critical metadata about all of your software artifacts.")
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
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: notes, occurrences and scan_configs");
        let mut notes1 = SubCommand::with_name("notes")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: batch_create, create, delete, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("batch_create").about("Creates new notes in batch.");
            notes1 = notes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new note.");
            notes1 = notes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified note.");
            notes1 = notes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified note.");
            notes1 = notes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a note or an occurrence resource. Requires `containeranalysis.notes.setIamPolicy` or `containeranalysis.occurrences.setIamPolicy` permission if the resource is a note or occurrence, respectively. The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for occurrences.");
            notes1 = notes1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists notes for the specified project.");
            notes1 = notes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified note.");
            notes1 = notes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified note or occurrence. Requires `containeranalysis.notes.setIamPolicy` or `containeranalysis.occurrences.setIamPolicy` permission if the resource is a note or an occurrence, respectively. The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for occurrences.");
            notes1 = notes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns the permissions that a caller has on the specified note or occurrence. Requires list permission on the project (for example, `containeranalysis.notes.list`). The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for occurrences.");
            notes1 = notes1.subcommand(mcmd);
        }
        let mut occurrences1 = SubCommand::with_name("occurrences")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: batch_create, create, delete, get, get_iam_policy, get_notes, get_vulnerability_summary, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd =
                SubCommand::with_name("batch_create").about("Creates new occurrences in batch.");
            occurrences1 = occurrences1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new occurrence.");
            occurrences1 = occurrences1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified occurrence. For example, use this method to delete an occurrence when the occurrence is no longer applicable for the given resource.");
            occurrences1 = occurrences1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified occurrence.");
            occurrences1 = occurrences1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a note or an occurrence resource. Requires `containeranalysis.notes.setIamPolicy` or `containeranalysis.occurrences.setIamPolicy` permission if the resource is a note or occurrence, respectively. The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for occurrences.");
            occurrences1 = occurrences1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_notes").about("Gets the note attached to the specified occurrence. Consumer projects can use this method to get a note that belongs to a provider project.");
            occurrences1 = occurrences1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_vulnerability_summary")
                .about("Gets a summary of the number and severity of occurrences.");
            occurrences1 = occurrences1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists occurrences for the specified project.");
            occurrences1 = occurrences1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified occurrence.");
            occurrences1 = occurrences1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified note or occurrence. Requires `containeranalysis.notes.setIamPolicy` or `containeranalysis.occurrences.setIamPolicy` permission if the resource is a note or an occurrence, respectively. The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for occurrences.");
            occurrences1 = occurrences1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns the permissions that a caller has on the specified note or occurrence. Requires list permission on the project (for example, `containeranalysis.notes.list`). The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for occurrences.");
            occurrences1 = occurrences1.subcommand(mcmd);
        }
        let mut scan_configs1 = SubCommand::with_name("scan_configs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list and update");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified scan configuration.");
            scan_configs1 = scan_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists scan configurations for the specified project.");
            scan_configs1 = scan_configs1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update").about("Updates the specified scan configuration.");
            scan_configs1 = scan_configs1.subcommand(mcmd);
        }
        let mut occurrences2 = SubCommand::with_name("occurrences")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists occurrences referencing the specified note. Provider projects can use this method to get all occurrences across consumer projects referencing the specified note.");
            occurrences2 = occurrences2.subcommand(mcmd);
        }
        notes1 = notes1.subcommand(occurrences2);
        projects0 = projects0.subcommand(scan_configs1);
        projects0 = projects0.subcommand(occurrences1);
        projects0 = projects0.subcommand(notes1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_containeranalysis1_beta1 as api;

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
