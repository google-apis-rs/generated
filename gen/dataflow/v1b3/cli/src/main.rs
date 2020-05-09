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
        let mut app = App::new("dataflow1_b3")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200319")
            .about("Manages Google Cloud Dataflow projects on Google Cloud Platform.")
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
            .about("methods: delete_snapshots and worker_messages");
        {
            let mcmd = SubCommand::with_name("delete_snapshots").about("Deletes a snapshot.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("worker_messages")
                .about("Send a worker_message to the service.");
            projects0 = projects0.subcommand(mcmd);
        }
        let mut jobs1 = SubCommand::with_name("jobs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: aggregated, create, get, get_metrics, list, snapshot and update");
        {
            let mcmd = SubCommand::with_name("aggregated")
                .about("List the jobs of a project across all regions.");
            jobs1 = jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a Cloud Dataflow job.\n\nTo create a job, we recommend using `projects.locations.jobs.create` with a\n[regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using\n`projects.jobs.create` is not recommended, as your job will always start\nin `us-central1`.");
            jobs1 = jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the state of the specified Cloud Dataflow job.\n\nTo get the state of a job, we recommend using `projects.locations.jobs.get`\nwith a [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using\n`projects.jobs.get` is not recommended, as you can only get the state of\njobs that are running in `us-central1`.");
            jobs1 = jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_metrics").about("Request the job status.\n\nTo request the status of a job, we recommend using\n`projects.locations.jobs.getMetrics` with a [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using\n`projects.jobs.getMetrics` is not recommended, as you can only request the\nstatus of jobs that are running in `us-central1`.");
            jobs1 = jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List the jobs of a project.\n\nTo list the jobs of a project in a region, we recommend using\n`projects.locations.jobs.get` with a [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). To\nlist the all jobs across all regions, use `projects.jobs.aggregated`. Using\n`projects.jobs.list` is not recommended, as you can only get the list of\njobs that are running in `us-central1`.");
            jobs1 = jobs1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("snapshot").about("Snapshot the state of a streaming job.");
            jobs1 = jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the state of an existing Cloud Dataflow job.\n\nTo update the state of an existing job, we recommend using\n`projects.locations.jobs.update` with a [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using\n`projects.jobs.update` is not recommended, as you can only update the state\nof jobs that are running in `us-central1`.");
            jobs1 = jobs1.subcommand(mcmd);
        }
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: worker_messages");
        {
            let mcmd = SubCommand::with_name("worker_messages")
                .about("Send a worker_message to the service.");
            locations1 = locations1.subcommand(mcmd);
        }
        let mut snapshots1 = SubCommand::with_name("snapshots")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets information about a snapshot.");
            snapshots1 = snapshots1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists snapshots.");
            snapshots1 = snapshots1.subcommand(mcmd);
        }
        let mut templates1 = SubCommand::with_name("templates")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get and launch");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a Cloud Dataflow job from a template.");
            templates1 = templates1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Get the template associated with a template.");
            templates1 = templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("launch").about("Launch a template.");
            templates1 = templates1.subcommand(mcmd);
        }
        let mut debug2 = SubCommand::with_name("debug")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_config and send_capture");
        {
            let mcmd = SubCommand::with_name("get_config")
                .about("Get encoded debug configuration for component. Not cacheable.");
            debug2 = debug2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("send_capture")
                .about("Send encoded debug capture data for component.");
            debug2 = debug2.subcommand(mcmd);
        }
        let mut messages2 = SubCommand::with_name("messages")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Request the job status.\n\nTo request the status of a job, we recommend using\n`projects.locations.jobs.messages.list` with a [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using\n`projects.jobs.messages.list` is not recommended, as you can only request\nthe status of jobs that are running in `us-central1`.");
            messages2 = messages2.subcommand(mcmd);
        }
        let mut work_items2 = SubCommand::with_name("work_items")
            .setting(AppSettings::ColoredHelp)
            .about("methods: lease and report_status");
        {
            let mcmd = SubCommand::with_name("lease").about("Leases a dataflow WorkItem to run.");
            work_items2 = work_items2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("report_status")
                .about("Reports the status of dataflow WorkItems leased by a worker.");
            work_items2 = work_items2.subcommand(mcmd);
        }
        let mut flex_templates2 = SubCommand::with_name("flex_templates")
            .setting(AppSettings::ColoredHelp)
            .about("methods: launch");
        {
            let mcmd = SubCommand::with_name("launch").about("Launch a job with a FlexTemplate.");
            flex_templates2 = flex_templates2.subcommand(mcmd);
        }
        let mut jobs2 = SubCommand::with_name("jobs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get, get_metrics, list, snapshot and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a Cloud Dataflow job.\n\nTo create a job, we recommend using `projects.locations.jobs.create` with a\n[regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using\n`projects.jobs.create` is not recommended, as your job will always start\nin `us-central1`.");
            jobs2 = jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the state of the specified Cloud Dataflow job.\n\nTo get the state of a job, we recommend using `projects.locations.jobs.get`\nwith a [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using\n`projects.jobs.get` is not recommended, as you can only get the state of\njobs that are running in `us-central1`.");
            jobs2 = jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_metrics").about("Request the job status.\n\nTo request the status of a job, we recommend using\n`projects.locations.jobs.getMetrics` with a [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using\n`projects.jobs.getMetrics` is not recommended, as you can only request the\nstatus of jobs that are running in `us-central1`.");
            jobs2 = jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List the jobs of a project.\n\nTo list the jobs of a project in a region, we recommend using\n`projects.locations.jobs.get` with a [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). To\nlist the all jobs across all regions, use `projects.jobs.aggregated`. Using\n`projects.jobs.list` is not recommended, as you can only get the list of\njobs that are running in `us-central1`.");
            jobs2 = jobs2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("snapshot").about("Snapshot the state of a streaming job.");
            jobs2 = jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the state of an existing Cloud Dataflow job.\n\nTo update the state of an existing job, we recommend using\n`projects.locations.jobs.update` with a [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using\n`projects.jobs.update` is not recommended, as you can only update the state\nof jobs that are running in `us-central1`.");
            jobs2 = jobs2.subcommand(mcmd);
        }
        let mut snapshots2 = SubCommand::with_name("snapshots")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a snapshot.");
            snapshots2 = snapshots2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets information about a snapshot.");
            snapshots2 = snapshots2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists snapshots.");
            snapshots2 = snapshots2.subcommand(mcmd);
        }
        let mut sql2 = SubCommand::with_name("sql")
            .setting(AppSettings::ColoredHelp)
            .about("methods: validate");
        {
            let mcmd = SubCommand::with_name("validate").about("Validates a GoogleSQL query for Cloud Dataflow syntax. Will always\nconfirm the given query parses correctly, and if able to look up\nschema information from DataCatalog, will validate that the query\nanalyzes properly as well.");
            sql2 = sql2.subcommand(mcmd);
        }
        let mut templates2 = SubCommand::with_name("templates")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get and launch");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a Cloud Dataflow job from a template.");
            templates2 = templates2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Get the template associated with a template.");
            templates2 = templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("launch").about("Launch a template.");
            templates2 = templates2.subcommand(mcmd);
        }
        let mut debug3 = SubCommand::with_name("debug")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_config and send_capture");
        {
            let mcmd = SubCommand::with_name("get_config")
                .about("Get encoded debug configuration for component. Not cacheable.");
            debug3 = debug3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("send_capture")
                .about("Send encoded debug capture data for component.");
            debug3 = debug3.subcommand(mcmd);
        }
        let mut messages3 = SubCommand::with_name("messages")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Request the job status.\n\nTo request the status of a job, we recommend using\n`projects.locations.jobs.messages.list` with a [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using\n`projects.jobs.messages.list` is not recommended, as you can only request\nthe status of jobs that are running in `us-central1`.");
            messages3 = messages3.subcommand(mcmd);
        }
        let mut snapshots3 = SubCommand::with_name("snapshots")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists snapshots.");
            snapshots3 = snapshots3.subcommand(mcmd);
        }
        let mut work_items3 = SubCommand::with_name("work_items")
            .setting(AppSettings::ColoredHelp)
            .about("methods: lease and report_status");
        {
            let mcmd = SubCommand::with_name("lease").about("Leases a dataflow WorkItem to run.");
            work_items3 = work_items3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("report_status")
                .about("Reports the status of dataflow WorkItems leased by a worker.");
            work_items3 = work_items3.subcommand(mcmd);
        }
        jobs2 = jobs2.subcommand(work_items3);
        jobs2 = jobs2.subcommand(snapshots3);
        jobs2 = jobs2.subcommand(messages3);
        jobs2 = jobs2.subcommand(debug3);
        locations1 = locations1.subcommand(templates2);
        locations1 = locations1.subcommand(sql2);
        locations1 = locations1.subcommand(snapshots2);
        locations1 = locations1.subcommand(jobs2);
        locations1 = locations1.subcommand(flex_templates2);
        jobs1 = jobs1.subcommand(work_items2);
        jobs1 = jobs1.subcommand(messages2);
        jobs1 = jobs1.subcommand(debug2);
        projects0 = projects0.subcommand(templates1);
        projects0 = projects0.subcommand(snapshots1);
        projects0 = projects0.subcommand(locations1);
        projects0 = projects0.subcommand(jobs1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_dataflow1_b3 as api;

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
