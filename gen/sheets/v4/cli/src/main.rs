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
        let mut app = App::new("sheets4")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210309")
            .about("Reads and writes Google Sheets.")
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
        let mut spreadsheets0 = SubCommand::with_name("spreadsheets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_update, create, get and get_by_data_filter");
        {
            let mcmd = SubCommand::with_name("batch_update").about("Applies one or more updates to the spreadsheet. Each request is validated before being applied. If any request is not valid then the entire request will fail and nothing will be applied. Some requests have replies to give you some information about how they are applied. The replies will mirror the requests. For example, if you applied 4 updates and the 3rd one had a reply, then the response will have 2 empty replies, the actual reply, and another empty reply, in that order. Due to the collaborative nature of spreadsheets, it is not guaranteed that the spreadsheet will reflect exactly your changes after this completes, however it is guaranteed that the updates in the request will be applied together atomically. Your changes may be altered with respect to collaborator changes. If there are no collaborators, the spreadsheet should reflect your changes.");
            spreadsheets0 = spreadsheets0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a spreadsheet, returning the newly created spreadsheet.");
            spreadsheets0 = spreadsheets0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the spreadsheet at the given ID. The caller must specify the spreadsheet ID. By default, data within grids will not be returned. You can include grid data one of two ways: * Specify a field mask listing your desired fields using the `fields` URL parameter in HTTP * Set the includeGridData URL parameter to true. If a field mask is set, the `includeGridData` parameter is ignored For large spreadsheets, it is recommended to retrieve only the specific fields of the spreadsheet that you want. To retrieve only subsets of the spreadsheet, use the ranges URL parameter. Multiple ranges can be specified. Limiting the range will return only the portions of the spreadsheet that intersect the requested ranges. Ranges are specified using A1 notation.");
            spreadsheets0 = spreadsheets0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_by_data_filter").about("Returns the spreadsheet at the given ID. The caller must specify the spreadsheet ID. This method differs from GetSpreadsheet in that it allows selecting which subsets of spreadsheet data to return by specifying a dataFilters parameter. Multiple DataFilters can be specified. Specifying one or more data filters will return the portions of the spreadsheet that intersect ranges matched by any of the filters. By default, data within grids will not be returned. You can include grid data one of two ways: * Specify a field mask listing your desired fields using the `fields` URL parameter in HTTP * Set the includeGridData parameter to true. If a field mask is set, the `includeGridData` parameter is ignored For large spreadsheets, it is recommended to retrieve only the specific fields of the spreadsheet that you want.");
            spreadsheets0 = spreadsheets0.subcommand(mcmd);
        }
        let mut developer_metadata1 = SubCommand::with_name("developer_metadata")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and search");
        {
            let mcmd = SubCommand::with_name("get").about("Returns the developer metadata with the specified ID. The caller must specify the spreadsheet ID and the developer metadata\'s unique metadataId.");
            developer_metadata1 = developer_metadata1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search").about("Returns all developer metadata matching the specified DataFilter. If the provided DataFilter represents a DeveloperMetadataLookup object, this will return all DeveloperMetadata entries selected by it. If the DataFilter represents a location in a spreadsheet, this will return all developer metadata associated with locations intersecting that region.");
            developer_metadata1 = developer_metadata1.subcommand(mcmd);
        }
        let mut sheets1 = SubCommand::with_name("sheets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: copy_to");
        {
            let mcmd = SubCommand::with_name("copy_to").about("Copies a single sheet from a spreadsheet to another spreadsheet. Returns the properties of the newly created sheet.");
            sheets1 = sheets1.subcommand(mcmd);
        }
        let mut values1 = SubCommand::with_name("values")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: append, batch_clear, batch_clear_by_data_filter, batch_get, batch_get_by_data_filter, batch_update, batch_update_by_data_filter, clear, get and update");
        {
            let mcmd = SubCommand::with_name("append").about("Appends values to a spreadsheet. The input range is used to search for existing data and find a \"table\" within that range. Values will be appended to the next row of the table, starting with the first column of the table. See the [guide](/sheets/api/guides/values#appending_values) and [sample code](/sheets/api/samples/writing#append_values) for specific details of how tables are detected and data is appended. The caller must specify the spreadsheet ID, range, and a valueInputOption. The `valueInputOption` only controls how the input data will be added to the sheet (column-wise or row-wise), it does not influence what cell the data starts being written to.");
            values1 = values1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_clear").about("Clears one or more ranges of values from a spreadsheet. The caller must specify the spreadsheet ID and one or more ranges. Only values are cleared -- all other properties of the cell (such as formatting, data validation, etc..) are kept.");
            values1 = values1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_clear_by_data_filter").about("Clears one or more ranges of values from a spreadsheet. The caller must specify the spreadsheet ID and one or more DataFilters. Ranges matching any of the specified data filters will be cleared. Only values are cleared -- all other properties of the cell (such as formatting, data validation, etc..) are kept.");
            values1 = values1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_get").about("Returns one or more ranges of values from a spreadsheet. The caller must specify the spreadsheet ID and one or more ranges.");
            values1 = values1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_get_by_data_filter").about("Returns one or more ranges of values that match the specified data filters. The caller must specify the spreadsheet ID and one or more DataFilters. Ranges that match any of the data filters in the request will be returned.");
            values1 = values1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_update").about("Sets values in one or more ranges of a spreadsheet. The caller must specify the spreadsheet ID, a valueInputOption, and one or more ValueRanges.");
            values1 = values1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_update_by_data_filter").about("Sets values in one or more ranges of a spreadsheet. The caller must specify the spreadsheet ID, a valueInputOption, and one or more DataFilterValueRanges.");
            values1 = values1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("clear").about("Clears values from a spreadsheet. The caller must specify the spreadsheet ID and range. Only values are cleared -- all other properties of the cell (such as formatting, data validation, etc..) are kept.");
            values1 = values1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns a range of values from a spreadsheet. The caller must specify the spreadsheet ID and a range.");
            values1 = values1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Sets values in a range of a spreadsheet. The caller must specify the spreadsheet ID, range, and a valueInputOption.");
            values1 = values1.subcommand(mcmd);
        }
        spreadsheets0 = spreadsheets0.subcommand(values1);
        spreadsheets0 = spreadsheets0.subcommand(sheets1);
        spreadsheets0 = spreadsheets0.subcommand(developer_metadata1);
        app = app.subcommand(spreadsheets0);

        Self { app }
    }
}
use google_sheets4 as api;

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
