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
        let mut app = App::new("fitness1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200429")
            .about("Stores and accesses user data in the fitness store from apps on any platform.")
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
        let mut users0 = SubCommand::with_name("users")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: data_sources, dataset and sessions");
        let mut data_sources1 = SubCommand::with_name("data_sources")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new data source that is unique across all data sources belonging to this user.\n\nA data source is a unique source of sensor data. Data sources can expose raw data coming from hardware sensors on local or companion devices. They can also expose derived data, created by transforming or merging other data sources. Multiple data sources can exist for the same data type. Every data point in every dataset inserted into or read from the Fitness API has an associated data source.\n\nEach data source produces a unique stream of dataset updates, with a unique data source identifier. Not all changes to data source affect the data stream ID, so that data collected by updated versions of the same application/device can still be considered to belong to the same data source.\n\nData sources are identified using a string generated by the server, based on the contents of the source being created. The dataStreamId field should not be set when invoking this method. It will be automatically generated by the server with the correct format. If a dataStreamId is set, it must match the format that the server would generate. This format is a combination of some fields from the data source, and has a specific order. If it doesn\'t match, the request will fail with an error.\n\nSpecifying a DataType which is not a known type (beginning with \"com.google.\") will create a DataSource with a custom data type. Custom data types are only readable by the application that created them. Custom data types are deprecated; use standard data types instead.\n\nIn addition to the data source fields included in the data source ID, the developer project number that is authenticated when creating the data source is included. This developer project number is obfuscated when read by any other developer reading public data types.");
            data_sources1 = data_sources1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified data source. The request will fail if the data source contains any data points.");
            data_sources1 = data_sources1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified data source.");
            data_sources1 = data_sources1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all data sources that are visible to the developer, using the OAuth scopes provided. The list is not exhaustive; the user may have private data sources that are only visible to other developers, or calls using other scopes.");
            data_sources1 = data_sources1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the specified data source. The dataStreamId, dataType, type, dataStreamName, and device properties with the exception of version, cannot be modified.\n\nData sources are identified by their dataStreamId.");
            data_sources1 = data_sources1.subcommand(mcmd);
        }
        let mut dataset1 = SubCommand::with_name("dataset")
            .setting(AppSettings::ColoredHelp)
            .about("methods: aggregate");
        {
            let mcmd = SubCommand::with_name("aggregate").about("Aggregates data of a certain type or stream into buckets divided by a given type of boundary. Multiple data sets of multiple types and from multiple sources can be aggregated into exactly one bucket type per request.");
            dataset1 = dataset1.subcommand(mcmd);
        }
        let mut sessions1 = SubCommand::with_name("sessions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, list and update");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes a session specified by the given session ID.");
            sessions1 = sessions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists sessions previously created.");
            sessions1 = sessions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates or insert a given session.");
            sessions1 = sessions1.subcommand(mcmd);
        }
        let mut data_point_changes2 = SubCommand::with_name("data_point_changes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Queries for user\'s data point changes for a particular data source.");
            data_point_changes2 = data_point_changes2.subcommand(mcmd);
        }
        let mut datasets2 = SubCommand::with_name("datasets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get and patch");
        {
            let mcmd = SubCommand::with_name("delete").about("Performs an inclusive delete of all data points whose start and end times have any overlap with the time range specified by the dataset ID. For most data types, the entire data point will be deleted. For data types where the time span represents a consistent value (such as com.google.activity.segment), and a data point straddles either end point of the dataset, only the overlapping portion of the data point will be deleted.");
            datasets2 = datasets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns a dataset containing all data points whose start and end times overlap with the specified range of the dataset minimum start time and maximum end time. Specifically, any data point whose start time is less than or equal to the dataset end time and whose end time is greater than or equal to the dataset start time.");
            datasets2 = datasets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Adds data points to a dataset. The dataset need not be previously created. All points within the given dataset will be returned with subsquent calls to retrieve this dataset. Data points can belong to more than one dataset. This method does not use patch semantics.");
            datasets2 = datasets2.subcommand(mcmd);
        }
        data_sources1 = data_sources1.subcommand(datasets2);
        data_sources1 = data_sources1.subcommand(data_point_changes2);
        users0 = users0.subcommand(sessions1);
        users0 = users0.subcommand(dataset1);
        users0 = users0.subcommand(data_sources1);
        app = app.subcommand(users0);

        Self { app }
    }
}
use google_fitness1 as api;

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
