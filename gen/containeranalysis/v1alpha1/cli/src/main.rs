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
        let mut app = App::new("containeranalysis1_alpha1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190913")
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
            .about("sub-resources: notes, occurrences, operations and scan_configs");
        let mut providers0 = SubCommand::with_name("providers")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: notes");
        let mut notes1 = SubCommand::with_name("notes")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new `Note`.");
            notes1 = notes1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes the given `Note` from the system.");
            notes1 = notes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the requested `Note`.");
            notes1 = notes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a note or an `Occurrence` resource.\nRequires `containeranalysis.notes.setIamPolicy` or\n`containeranalysis.occurrences.setIamPolicy` permission if the resource is\na note or occurrence, respectively.\nAttempting to call this method on a resource without the required\npermission will result in a `PERMISSION_DENIED` error. Attempting to call\nthis method on a non-existent resource will result in a `NOT_FOUND` error\nif the user has list permission on the project, or a `PERMISSION_DENIED`\nerror otherwise. The resource takes the following formats:\n`projects/{PROJECT_ID}/occurrences/{OCCURRENCE_ID}` for occurrences and\nprojects/{PROJECT_ID}/notes/{NOTE_ID} for notes");
            notes1 = notes1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists all `Notes` for a given project.");
            notes1 = notes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing `Note`.");
            notes1 = notes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified `Note` or `Occurrence`.\nRequires `containeranalysis.notes.setIamPolicy` or\n`containeranalysis.occurrences.setIamPolicy` permission if the resource is\na `Note` or an `Occurrence`, respectively.\nAttempting to call this method without these permissions will result in a `\n`PERMISSION_DENIED` error.\nAttempting to call this method on a non-existent resource will result in a\n`NOT_FOUND` error if the user has `containeranalysis.notes.list` permission\non a `Note` or `containeranalysis.occurrences.list` on an `Occurrence`, or\na `PERMISSION_DENIED` error otherwise. The resource takes the following\nformats: `projects/{projectid}/occurrences/{occurrenceid}` for occurrences\nand projects/{projectid}/notes/{noteid} for notes");
            notes1 = notes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns the permissions that a caller has on the specified note or\noccurrence resource. Requires list permission on the project (for example,\n\"storage.objects.list\" on the containing bucket for testing permission of\nan object). Attempting to call this method on a non-existent resource will\nresult in a `NOT_FOUND` error if the user has list permission on the\nproject, or a `PERMISSION_DENIED` error otherwise. The resource takes the\nfollowing formats: `projects/{PROJECT_ID}/occurrences/{OCCURRENCE_ID}` for\n`Occurrences` and `projects/{PROJECT_ID}/notes/{NOTE_ID}` for `Notes`");
            notes1 = notes1.subcommand(mcmd);
        }
        let mut occurrences1 = SubCommand::with_name("occurrences")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, get_notes, get_vulnerability_summary, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new `Occurrence`. Use this method to create `Occurrences`\nfor a resource.");
            occurrences1 = occurrences1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the given `Occurrence` from the system. Use this when\nan `Occurrence` is no longer applicable for the given resource.");
            occurrences1 = occurrences1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the requested `Occurrence`.");
            occurrences1 = occurrences1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a note or an `Occurrence` resource.\nRequires `containeranalysis.notes.setIamPolicy` or\n`containeranalysis.occurrences.setIamPolicy` permission if the resource is\na note or occurrence, respectively.\nAttempting to call this method on a resource without the required\npermission will result in a `PERMISSION_DENIED` error. Attempting to call\nthis method on a non-existent resource will result in a `NOT_FOUND` error\nif the user has list permission on the project, or a `PERMISSION_DENIED`\nerror otherwise. The resource takes the following formats:\n`projects/{PROJECT_ID}/occurrences/{OCCURRENCE_ID}` for occurrences and\nprojects/{PROJECT_ID}/notes/{NOTE_ID} for notes");
            occurrences1 = occurrences1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_notes")
                .about("Gets the `Note` attached to the given `Occurrence`.");
            occurrences1 = occurrences1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_vulnerability_summary")
                .about("Gets a summary of the number and severity of occurrences.");
            occurrences1 = occurrences1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists active `Occurrences` for a given project matching the filters.");
            occurrences1 = occurrences1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing occurrence.");
            occurrences1 = occurrences1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified `Note` or `Occurrence`.\nRequires `containeranalysis.notes.setIamPolicy` or\n`containeranalysis.occurrences.setIamPolicy` permission if the resource is\na `Note` or an `Occurrence`, respectively.\nAttempting to call this method without these permissions will result in a `\n`PERMISSION_DENIED` error.\nAttempting to call this method on a non-existent resource will result in a\n`NOT_FOUND` error if the user has `containeranalysis.notes.list` permission\non a `Note` or `containeranalysis.occurrences.list` on an `Occurrence`, or\na `PERMISSION_DENIED` error otherwise. The resource takes the following\nformats: `projects/{projectid}/occurrences/{occurrenceid}` for occurrences\nand projects/{projectid}/notes/{noteid} for notes");
            occurrences1 = occurrences1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns the permissions that a caller has on the specified note or\noccurrence resource. Requires list permission on the project (for example,\n\"storage.objects.list\" on the containing bucket for testing permission of\nan object). Attempting to call this method on a non-existent resource will\nresult in a `NOT_FOUND` error if the user has list permission on the\nproject, or a `PERMISSION_DENIED` error otherwise. The resource takes the\nfollowing formats: `projects/{PROJECT_ID}/occurrences/{OCCURRENCE_ID}` for\n`Occurrences` and `projects/{PROJECT_ID}/notes/{NOTE_ID}` for `Notes`");
            occurrences1 = occurrences1.subcommand(mcmd);
        }
        let mut operations1 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new `Operation`.");
            operations1 = operations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing operation returns an error if operation\n does not exist. The only valid operations are to update mark the done bit\nchange the result.");
            operations1 = operations1.subcommand(mcmd);
        }
        let mut scan_configs1 = SubCommand::with_name("scan_configs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list and patch");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets a specific scan configuration for a project.");
            scan_configs1 = scan_configs1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists scan configurations for a project.");
            scan_configs1 = scan_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates the scan configuration to a new value.");
            scan_configs1 = scan_configs1.subcommand(mcmd);
        }
        let mut notes1 = SubCommand::with_name("notes")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new `Note`.");
            notes1 = notes1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes the given `Note` from the system.");
            notes1 = notes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the requested `Note`.");
            notes1 = notes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a note or an `Occurrence` resource.\nRequires `containeranalysis.notes.setIamPolicy` or\n`containeranalysis.occurrences.setIamPolicy` permission if the resource is\na note or occurrence, respectively.\nAttempting to call this method on a resource without the required\npermission will result in a `PERMISSION_DENIED` error. Attempting to call\nthis method on a non-existent resource will result in a `NOT_FOUND` error\nif the user has list permission on the project, or a `PERMISSION_DENIED`\nerror otherwise. The resource takes the following formats:\n`projects/{PROJECT_ID}/occurrences/{OCCURRENCE_ID}` for occurrences and\nprojects/{PROJECT_ID}/notes/{NOTE_ID} for notes");
            notes1 = notes1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists all `Notes` for a given project.");
            notes1 = notes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing `Note`.");
            notes1 = notes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified `Note` or `Occurrence`.\nRequires `containeranalysis.notes.setIamPolicy` or\n`containeranalysis.occurrences.setIamPolicy` permission if the resource is\na `Note` or an `Occurrence`, respectively.\nAttempting to call this method without these permissions will result in a `\n`PERMISSION_DENIED` error.\nAttempting to call this method on a non-existent resource will result in a\n`NOT_FOUND` error if the user has `containeranalysis.notes.list` permission\non a `Note` or `containeranalysis.occurrences.list` on an `Occurrence`, or\na `PERMISSION_DENIED` error otherwise. The resource takes the following\nformats: `projects/{projectid}/occurrences/{occurrenceid}` for occurrences\nand projects/{projectid}/notes/{noteid} for notes");
            notes1 = notes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns the permissions that a caller has on the specified note or\noccurrence resource. Requires list permission on the project (for example,\n\"storage.objects.list\" on the containing bucket for testing permission of\nan object). Attempting to call this method on a non-existent resource will\nresult in a `NOT_FOUND` error if the user has list permission on the\nproject, or a `PERMISSION_DENIED` error otherwise. The resource takes the\nfollowing formats: `projects/{PROJECT_ID}/occurrences/{OCCURRENCE_ID}` for\n`Occurrences` and `projects/{PROJECT_ID}/notes/{NOTE_ID}` for `Notes`");
            notes1 = notes1.subcommand(mcmd);
        }
        let mut occurrences2 = SubCommand::with_name("occurrences")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists `Occurrences` referencing the specified `Note`. Use this method to\nget all occurrences referencing your `Note` across all your customer\nprojects.");
            occurrences2 = occurrences2.subcommand(mcmd);
        }
        let mut occurrences2 = SubCommand::with_name("occurrences")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists `Occurrences` referencing the specified `Note`. Use this method to\nget all occurrences referencing your `Note` across all your customer\nprojects.");
            occurrences2 = occurrences2.subcommand(mcmd);
        }
        notes1 = notes1.subcommand(occurrences2);
        notes1 = notes1.subcommand(occurrences2);
        providers0 = providers0.subcommand(notes1);
        projects0 = projects0.subcommand(scan_configs1);
        projects0 = projects0.subcommand(operations1);
        projects0 = projects0.subcommand(occurrences1);
        projects0 = projects0.subcommand(notes1);
        app = app.subcommand(providers0);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_containeranalysis1_alpha1 as api;

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
