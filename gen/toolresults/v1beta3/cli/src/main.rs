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
        let mut app = App::new("toolresults1_beta3")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200513")
            .about("API to publish and access results from developer tools.")
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
        let mut clusters0 = SubCommand::with_name("clusters")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves a single screenshot cluster by its ID");
            clusters0 = clusters0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists Screenshot Clusters\n\nReturns the list of screenshot clusters corresponding to an execution.\nScreenshot clusters are created after the execution is finished.\nClusters are created from a set of screenshots. Between any two\nscreenshots, a matching score is calculated based off their metadata\nthat determines how similar they are. Screenshots are placed\nin the cluster that has screens which have the highest matching\nscores.");
            clusters0 = clusters0.subcommand(mcmd);
        }
        let mut environments0 = SubCommand::with_name("environments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets an Environment.\n\nMay return any of the following canonical error codes:\n\n- PERMISSION_DENIED - if the user is not authorized to read project\n- INVALID_ARGUMENT - if the request is malformed\n- NOT_FOUND - if the Environment does not exist");
            environments0 = environments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists Environments for a given Execution.\n\nThe Environments are sorted by display name.\n\nMay return any of the following canonical error codes:\n\n- PERMISSION_DENIED - if the user is not authorized to read project\n- INVALID_ARGUMENT - if the request is malformed\n- NOT_FOUND - if the containing Execution does not exist");
            environments0 = environments0.subcommand(mcmd);
        }
        let mut executions0 = SubCommand::with_name("executions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an Execution.\n\nThe returned Execution will have the id set.\n\nMay return any of the following canonical error codes:\n\n- PERMISSION_DENIED - if the user is not authorized to write to project\n- INVALID_ARGUMENT - if the request is malformed\n- NOT_FOUND - if the containing History does not exist");
            executions0 = executions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an Execution.\n\nMay return any of the following canonical error codes:\n\n- PERMISSION_DENIED - if the user is not authorized to write to project\n- INVALID_ARGUMENT - if the request is malformed\n- NOT_FOUND - if the Execution does not exist");
            executions0 = executions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists Executions for a given History.\n\nThe executions are sorted by creation_time in descending order. The\nexecution_id key will be used to order the executions with the same\ncreation_time.\n\nMay return any of the following canonical error codes:\n\n- PERMISSION_DENIED - if the user is not authorized to read project\n- INVALID_ARGUMENT - if the request is malformed\n- NOT_FOUND - if the containing History does not exist");
            executions0 = executions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing Execution with the supplied partial entity.\n\nMay return any of the following canonical error codes:\n\n- PERMISSION_DENIED - if the user is not authorized to write to project\n- INVALID_ARGUMENT - if the request is malformed\n- FAILED_PRECONDITION - if the requested state transition is illegal\n- NOT_FOUND - if the containing History does not exist");
            executions0 = executions0.subcommand(mcmd);
        }
        let mut histories0 = SubCommand::with_name("histories")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a History.\n\nThe returned History will have the id set.\n\nMay return any of the following canonical error codes:\n\n- PERMISSION_DENIED - if the user is not authorized to write to project\n- INVALID_ARGUMENT - if the request is malformed\n- NOT_FOUND - if the containing project does not exist");
            histories0 = histories0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a History.\n\nMay return any of the following canonical error codes:\n\n- PERMISSION_DENIED - if the user is not authorized to read project\n- INVALID_ARGUMENT - if the request is malformed\n- NOT_FOUND - if the History does not exist");
            histories0 = histories0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists Histories for a given Project.\n\nThe histories are sorted by modification time in descending order. The\nhistory_id key will be used to order the history with the same\nmodification time.\n\nMay return any of the following canonical error codes:\n\n- PERMISSION_DENIED - if the user is not authorized to read project\n- INVALID_ARGUMENT - if the request is malformed\n- NOT_FOUND - if the History does not exist");
            histories0 = histories0.subcommand(mcmd);
        }
        let mut perf_metrics_summary0 = SubCommand::with_name("perf_metrics_summary")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a PerfMetricsSummary resource. Returns the existing one if it has\nalready been created.\n\nMay return any of the following error code(s):\n- NOT_FOUND - The containing Step does not exist");
            perf_metrics_summary0 = perf_metrics_summary0.subcommand(mcmd);
        }
        let mut perf_sample_series0 = SubCommand::with_name("perf_sample_series")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a PerfSampleSeries.\n\nMay return any of the following error code(s):\n- ALREADY_EXISTS - PerfMetricSummary already exists for the given Step\n- NOT_FOUND - The containing Step does not exist");
            perf_sample_series0 = perf_sample_series0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a PerfSampleSeries.\n\nMay return any of the following error code(s):\n- NOT_FOUND - The specified PerfSampleSeries does not exist");
            perf_sample_series0 = perf_sample_series0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists PerfSampleSeries for a given Step.\n\nThe request provides an optional filter which specifies one or more\nPerfMetricsType to include in the result; if none returns all.\nThe resulting PerfSampleSeries are sorted by ids.\n\nMay return any of the following canonical error codes:\n- NOT_FOUND - The containing Step does not exist");
            perf_sample_series0 = perf_sample_series0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_settings and initialize_settings");
        {
            let mcmd = SubCommand::with_name("get_settings").about("Gets the Tool Results settings for a project.\n\nMay return any of the following canonical error codes:\n\n- PERMISSION_DENIED - if the user is not authorized to read from project");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("initialize_settings").about("Creates resources for settings which have not yet been set.\n\nCurrently, this creates a single resource: a Google Cloud Storage bucket,\nto be used as the default bucket for this project. The bucket is created\nin an FTL-own storage project. Except for in rare cases, calling this\nmethod in parallel from multiple clients will only create a single bucket.\nIn order to avoid unnecessary storage charges, the bucket is configured to\nautomatically delete objects older than 90 days.\n\nThe bucket is created with the following permissions:\n- Owner access for owners of central storage project (FTL-owned)\n- Writer access for owners/editors of customer project\n- Reader access for viewers of customer project\nThe default ACL on objects created in the bucket is:\n- Owner access for owners of central storage project\n- Reader access for owners/editors/viewers of customer project\nSee Google Cloud Storage documentation for more details.\n\nIf there is already a default bucket set and the project can access the\nbucket, this call does nothing. However, if the project doesn\'t have the\npermission to access the bucket or the bucket is deleted, a new bucket\nwill be created.\n\nMay return any canonical error codes, including the following:\n\n- PERMISSION_DENIED - if the user is not authorized to write to project\n- Any error code raised by Google Cloud Storage");
            projects0 = projects0.subcommand(mcmd);
        }
        let mut samples0 = SubCommand::with_name("samples")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_create and list");
        {
            let mcmd = SubCommand::with_name("batch_create").about("Creates a batch of PerfSamples\n- a client can submit multiple batches of Perf Samples through repeated\ncalls to this method in order to split up a large request payload\n- duplicates and existing timestamp entries will be ignored.\n- the batch operation may partially succeed\n- the set of elements successfully inserted is returned in the response\n(omits items which already existed in the database).\n\nMay return any of the following canonical error codes:\n- NOT_FOUND - The containing PerfSampleSeries does not exist");
            samples0 = samples0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the Performance Samples of a given Sample Series\n- The list results are sorted by timestamps ascending\n- The default page size is 500 samples; and maximum size allowed 5000\n- The response token indicates the last returned PerfSample timestamp\n- When the results size exceeds the page size, submit a subsequent request\nincluding the page token to return the rest of the samples up to the\npage limit\n\nMay return any of the following canonical error codes:\n- OUT_OF_RANGE - The specified request page_token is out of valid range\n- NOT_FOUND - The containing PerfSampleSeries does not exist");
            samples0 = samples0.subcommand(mcmd);
        }
        let mut steps0 = SubCommand::with_name("steps")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: accessibility_clusters, create, get, get_perf_metrics_summary, list, patch and publish_xunit_xml_files");
        {
            let mcmd = SubCommand::with_name("accessibility_clusters").about("Lists accessibility clusters for a given Step\n\nMay return any of the following canonical error codes:\n\n- PERMISSION_DENIED - if the user is not authorized to read project\n- INVALID_ARGUMENT - if the request is malformed\n- FAILED_PRECONDITION - if an argument in the request happens to be\n                        invalid; e.g. if the locale format is incorrect\n- NOT_FOUND - if the containing Step does not exist");
            steps0 = steps0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a Step.\n\nThe returned Step will have the id set.\n\nMay return any of the following canonical error codes:\n\n- PERMISSION_DENIED - if the user is not authorized to write to project\n- INVALID_ARGUMENT - if the request is malformed\n- FAILED_PRECONDITION - if the step is too large (more than 10Mib)\n- NOT_FOUND - if the containing Execution does not exist");
            steps0 = steps0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a Step.\n\nMay return any of the following canonical error codes:\n\n- PERMISSION_DENIED - if the user is not authorized to read project\n- INVALID_ARGUMENT - if the request is malformed\n- NOT_FOUND - if the Step does not exist");
            steps0 = steps0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_perf_metrics_summary").about("Retrieves a PerfMetricsSummary.\n\nMay return any of the following error code(s):\n- NOT_FOUND - The specified PerfMetricsSummary does not exist");
            steps0 = steps0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists Steps for a given Execution.\n\nThe steps are sorted by creation_time in descending order. The\nstep_id key will be used to order the steps with the same\ncreation_time.\n\nMay return any of the following canonical error codes:\n\n- PERMISSION_DENIED - if the user is not authorized to read project\n- INVALID_ARGUMENT - if the request is malformed\n- FAILED_PRECONDITION - if an argument in the request happens to be\n                        invalid; e.g. if an attempt is made to list the\n                        children of a nonexistent Step\n- NOT_FOUND - if the containing Execution does not exist");
            steps0 = steps0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing Step with the supplied partial entity.\n\nMay return any of the following canonical error codes:\n\n- PERMISSION_DENIED - if the user is not authorized to write project\n- INVALID_ARGUMENT - if the request is malformed\n- FAILED_PRECONDITION - if the requested state transition is illegal\n                        (e.g try to upload a duplicate xml file), if the\n                        updated step is too large (more than 10Mib)\n- NOT_FOUND - if the containing Execution does not exist");
            steps0 = steps0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("publish_xunit_xml_files").about("Publish xml files to an existing Step.\n\nMay return any of the following canonical error codes:\n\n- PERMISSION_DENIED - if the user is not authorized to write project\n- INVALID_ARGUMENT - if the request is malformed\n- FAILED_PRECONDITION - if the requested state transition is illegal,\ne.g try to upload a duplicate xml file or a file too large.\n- NOT_FOUND - if the containing Execution does not exist");
            steps0 = steps0.subcommand(mcmd);
        }
        let mut test_cases0 = SubCommand::with_name("test_cases")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets details of a Test Case for a Step.\nExperimental test cases API. Still in active development.\n\nMay return any of the following canonical error codes:\n\n- PERMISSION_DENIED - if the user is not authorized to write to project\n- INVALID_ARGUMENT - if the request is malformed\n- NOT_FOUND - if the containing Test Case does not exist");
            test_cases0 = test_cases0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists Test Cases attached to a Step.\nExperimental test cases API. Still in active development.\n\nMay return any of the following canonical error codes:\n\n- PERMISSION_DENIED - if the user is not authorized to write to project\n- INVALID_ARGUMENT - if the request is malformed\n- NOT_FOUND - if the containing Step does not exist");
            test_cases0 = test_cases0.subcommand(mcmd);
        }
        let mut thumbnails0 = SubCommand::with_name("thumbnails")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists thumbnails of images attached to a step.\n\nMay return any of the following canonical error codes:\n- PERMISSION_DENIED - if the user is not authorized to read from the\n                      project, or from any of the images\n- INVALID_ARGUMENT - if the request is malformed\n- NOT_FOUND - if the step does not exist, or if any of the images\n              do not exist");
            thumbnails0 = thumbnails0.subcommand(mcmd);
        }
        app = app.subcommand(thumbnails0);
        app = app.subcommand(test_cases0);
        app = app.subcommand(steps0);
        app = app.subcommand(samples0);
        app = app.subcommand(projects0);
        app = app.subcommand(perf_sample_series0);
        app = app.subcommand(perf_metrics_summary0);
        app = app.subcommand(histories0);
        app = app.subcommand(executions0);
        app = app.subcommand(environments0);
        app = app.subcommand(clusters0);

        Self { app }
    }
}
use google_toolresults1_beta3 as api;

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
