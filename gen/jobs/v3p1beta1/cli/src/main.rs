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
        let mut app = App::new("jobs3_p1beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200506")
            .about("Cloud Talent Solution provides the capability to create, read, update, and delete job postings, as well as search jobs based on keywords and filters.\n")
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
            .about("methods: complete");
        {
            let mcmd = SubCommand::with_name("complete").about("Completes the specified prefix with keyword suggestions.\nIntended for use by a job search auto-complete search box.");
            projects0 = projects0.subcommand(mcmd);
        }
        let mut client_events1 = SubCommand::with_name("client_events")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create");
        {
            let mcmd = SubCommand::with_name("create").about("Report events issued when end user interacts with customer\'s application\nthat uses Cloud Talent Solution. You may inspect the created events in\n[self service\ntools](https://console.cloud.google.com/talent-solution/overview).\n[Learn\nmore](https://cloud.google.com/talent-solution/docs/management-tools)\nabout self service tools.");
            client_events1 = client_events1.subcommand(mcmd);
        }
        let mut companies1 = SubCommand::with_name("companies")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new company entity.");
            companies1 = companies1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes specified company.\nPrerequisite: The company has no jobs associated with it.");
            companies1 = companies1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves specified company.");
            companies1 = companies1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all companies associated with the service account.");
            companies1 = companies1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates specified company. Company names can\'t be updated. To update a\ncompany name, delete the company and all jobs associated with it, and only\nthen re-create them.");
            companies1 = companies1.subcommand(mcmd);
        }
        let mut jobs1 = SubCommand::with_name("jobs")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: batch_delete, create, delete, get, list, patch, search and search_for_alert");
        {
            let mcmd =
                SubCommand::with_name("batch_delete").about("Deletes a list of Jobs by filter.");
            jobs1 = jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new job.\n\nTypically, the job becomes searchable within 10 seconds, but it may take\nup to 5 minutes.");
            jobs1 = jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified job.\n\nTypically, the job becomes unsearchable within 10 seconds, but it may take\nup to 5 minutes.");
            jobs1 = jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified job, whose status is OPEN or recently EXPIRED\nwithin the last 90 days.");
            jobs1 = jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists jobs by filter.");
            jobs1 = jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates specified job.\n\nTypically, updated contents become visible in search results within 10\nseconds, but it may take up to 5 minutes.");
            jobs1 = jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search").about("Searches for jobs using the provided SearchJobsRequest.\n\nThis call constrains the visibility of jobs\npresent in the database, and only returns jobs that the caller has\npermission to search against.");
            jobs1 = jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search_for_alert").about("Searches for jobs using the provided SearchJobsRequest.\n\nThis API call is intended for the use case of targeting passive job\nseekers (for example, job seekers who have signed up to receive email\nalerts about potential job opportunities), and has different algorithmic\nadjustments that are targeted to passive job seekers.\n\nThis call constrains the visibility of jobs\npresent in the database, and only returns jobs the caller has\npermission to search against.");
            jobs1 = jobs1.subcommand(mcmd);
        }
        let mut operations1 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice.");
            operations1 = operations1.subcommand(mcmd);
        }
        projects0 = projects0.subcommand(operations1);
        projects0 = projects0.subcommand(jobs1);
        projects0 = projects0.subcommand(companies1);
        projects0 = projects0.subcommand(client_events1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_jobs3_p1beta1 as api;

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
