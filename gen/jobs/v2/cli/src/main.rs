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
        let mut app = App::new("jobs2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200604")
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
        let mut companies0 = SubCommand::with_name("companies")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new company entity.");
            companies0 = companies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified company.");
            companies0 = companies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified company.");
            companies0 = companies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all companies associated with a Cloud Talent Solution account.");
            companies0 = companies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified company. Company names can\'t be updated. To update a\ncompany name, delete the company and all jobs associated with it, and only\nthen re-create them.");
            companies0 = companies0.subcommand(mcmd);
        }
        let mut jobs0 = SubCommand::with_name("jobs")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: batch_delete, create, delete, delete_by_filter, get, histogram, list, patch, search and search_for_alert");
        {
            let mcmd = SubCommand::with_name("batch_delete")
                .about("Deletes a list of Job postings by filter.");
            jobs0 = jobs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new job.\n\nTypically, the job becomes searchable within 10 seconds, but it may take\nup to 5 minutes.");
            jobs0 = jobs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified job.\n\nTypically, the job becomes unsearchable within 10 seconds, but it may take\nup to 5 minutes.");
            jobs0 = jobs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete_by_filter").about("Deprecated. Use BatchDeleteJobs instead.\n\nDeletes the specified job by filter. You can specify whether to\nsynchronously wait for validation, indexing, and general processing to be\ncompleted before the response is returned.");
            jobs0 = jobs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified job, whose status is OPEN or recently EXPIRED\nwithin the last 90 days.");
            jobs0 = jobs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("histogram").about("Deprecated. Use SearchJobsRequest.histogram_facets instead to make\na single call with both search and histogram.\n\nRetrieves a histogram for the given\nGetHistogramRequest. This call provides a structured\ncount of jobs that match against the search query, grouped by specified\nfacets.\n\nThis call constrains the visibility of jobs\npresent in the database, and only counts jobs the caller has\npermission to search against.\n\nFor example, use this call to generate the\nnumber of jobs in the U.S. by state.");
            jobs0 = jobs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists jobs by filter.");
            jobs0 = jobs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates specified job.\n\nTypically, updated contents become visible in search results within 10\nseconds, but it may take up to 5 minutes.");
            jobs0 = jobs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search").about("Searches for jobs using the provided SearchJobsRequest.\n\nThis call constrains the visibility of jobs\npresent in the database, and only returns jobs that the caller has\npermission to search against.");
            jobs0 = jobs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search_for_alert").about("Searches for jobs using the provided SearchJobsRequest.\n\nThis API call is intended for the use case of targeting passive job\nseekers (for example, job seekers who have signed up to receive email\nalerts about potential job opportunities), and has different algorithmic\nadjustments that are targeted to passive job seekers.\n\nThis call constrains the visibility of jobs\npresent in the database, and only returns jobs the caller has\npermission to search against.");
            jobs0 = jobs0.subcommand(mcmd);
        }
        let mut v_20 = SubCommand::with_name("v_2")
            .setting(AppSettings::ColoredHelp)
            .about("methods: complete");
        {
            let mcmd = SubCommand::with_name("complete").about("Completes the specified prefix with job keyword suggestions.\nIntended for use by a job search auto-complete search box.");
            v_20 = v_20.subcommand(mcmd);
        }
        let mut jobs1 = SubCommand::with_name("jobs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about(
                "Deprecated. Use ListJobs instead.\n\nLists all jobs associated with a company.",
            );
            jobs1 = jobs1.subcommand(mcmd);
        }
        companies0 = companies0.subcommand(jobs1);
        app = app.subcommand(v_20);
        app = app.subcommand(jobs0);
        app = app.subcommand(companies0);

        Self { app }
    }
}
use google_jobs2 as api;

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
