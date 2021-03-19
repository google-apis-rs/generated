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
        let mut app = App::new("jobs4")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210309")
            .about("Cloud Talent Solution provides the capability to create, read, update, and delete job postings, as well as search jobs based on keywords and filters. ")
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
            .about("sub-resources: operations and tenants");
        let mut operations1 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations1 = operations1.subcommand(mcmd);
        }
        let mut tenants1 = SubCommand::with_name("tenants")
            .setting(AppSettings::ColoredHelp)
            .about("methods: complete_query, create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("complete_query").about("Completes the specified prefix with keyword suggestions. Intended for use by a job search auto-complete search box.");
            tenants1 = tenants1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new tenant entity.");
            tenants1 = tenants1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes specified tenant.");
            tenants1 = tenants1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves specified tenant.");
            tenants1 = tenants1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all tenants associated with the project.");
            tenants1 = tenants1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates specified tenant.");
            tenants1 = tenants1.subcommand(mcmd);
        }
        let mut client_events2 = SubCommand::with_name("client_events")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create");
        {
            let mcmd = SubCommand::with_name("create").about("Report events issued when end user interacts with customer\'s application that uses Cloud Talent Solution. You may inspect the created events in [self service tools](https://console.cloud.google.com/talent-solution/overview). [Learn more](https://cloud.google.com/talent-solution/docs/management-tools) about self service tools.");
            client_events2 = client_events2.subcommand(mcmd);
        }
        let mut companies2 = SubCommand::with_name("companies")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new company entity.");
            companies2 = companies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes specified company. Prerequisite: The company has no jobs associated with it.");
            companies2 = companies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves specified company.");
            companies2 = companies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all companies associated with the project.");
            companies2 = companies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates specified company.");
            companies2 = companies2.subcommand(mcmd);
        }
        let mut jobs2 = SubCommand::with_name("jobs")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: batch_create, batch_delete, batch_update, create, delete, get, list, patch, search and search_for_alert");
        {
            let mcmd = SubCommand::with_name("batch_create")
                .about("Begins executing a batch create jobs operation.");
            jobs2 = jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_delete")
                .about("Begins executing a batch delete jobs operation.");
            jobs2 = jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_update")
                .about("Begins executing a batch update jobs operation.");
            jobs2 = jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new job. Typically, the job becomes searchable within 10 seconds, but it may take up to 5 minutes.");
            jobs2 = jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified job. Typically, the job becomes unsearchable within 10 seconds, but it may take up to 5 minutes.");
            jobs2 = jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified job, whose status is OPEN or recently EXPIRED within the last 90 days.");
            jobs2 = jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists jobs by filter.");
            jobs2 = jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates specified job. Typically, updated contents become visible in search results within 10 seconds, but it may take up to 5 minutes.");
            jobs2 = jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search").about("Searches for jobs using the provided SearchJobsRequest. This call constrains the visibility of jobs present in the database, and only returns jobs that the caller has permission to search against.");
            jobs2 = jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search_for_alert").about("Searches for jobs using the provided SearchJobsRequest. This API call is intended for the use case of targeting passive job seekers (for example, job seekers who have signed up to receive email alerts about potential job opportunities), it has different algorithmic adjustments that are designed to specifically target passive job seekers. This call constrains the visibility of jobs present in the database, and only returns jobs the caller has permission to search against.");
            jobs2 = jobs2.subcommand(mcmd);
        }
        tenants1 = tenants1.subcommand(jobs2);
        tenants1 = tenants1.subcommand(companies2);
        tenants1 = tenants1.subcommand(client_events2);
        projects0 = projects0.subcommand(tenants1);
        projects0 = projects0.subcommand(operations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_jobs4 as api;

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
