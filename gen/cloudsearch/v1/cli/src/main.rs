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
        let mut app = App::new("cloudsearch1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200609")
            .about("Cloud Search provides cloud-based search capabilities over G Suite data.  The Cloud Search API allows indexing of non-G Suite data into Cloud Search.")
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
        let mut debug0 = SubCommand::with_name("debug")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: datasources and identitysources");
        let mut indexing0 = SubCommand::with_name("indexing")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: datasources");
        let mut media0 = SubCommand::with_name("media")
            .setting(AppSettings::ColoredHelp)
            .about("methods: upload");
        {
            let mcmd = SubCommand::with_name("upload").about("Uploads media for indexing.\n\nThe upload endpoint supports direct and resumable upload protocols and\nis intended for large items that can not be\n[inlined during index requests](https://developers.google.com/cloud-search/docs/reference/rest/v1/indexing.datasources.items#itemcontent).\nTo index large content:\n\n1. Call\n   indexing.datasources.items.upload\n   with the resource name to begin an upload session and retrieve the\n   UploadItemRef.\n1. Call media.upload to upload the content using the same resource name from step 1.\n1. Call indexing.datasources.items.index\n   to index the item. Populate the\n   [ItemContent](/cloud-search/docs/reference/rest/v1/indexing.datasources.items#ItemContent)\n   with the UploadItemRef from step 1.\n\n\nFor additional information, see\n[Create a content connector using the REST API](https://developers.google.com/cloud-search/docs/guides/content-connector#rest).\n\n  **Note:** This API requires a service account to execute.");
            media0 = media0.subcommand(mcmd);
        }
        let mut operations0 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice.");
            operations0 = operations0.subcommand(mcmd);
        }
        let mut query0 = SubCommand::with_name("query")
            .setting(AppSettings::ColoredHelp)
            .about("methods: search and suggest");
        {
            let mcmd = SubCommand::with_name("search").about("The Cloud Search Query API provides the search method, which returns\nthe most relevant results from a user query.  The results can come from\nG Suite Apps, such as Gmail or Google Drive, or they can come from data\nthat you have indexed from a third party.\n\n**Note:** This API requires a standard end user account to execute.\nA service account can\'t perform Query API requests directly; to use a\nservice account to perform queries, set up [G Suite domain-wide delegation\nof\nauthority](https://developers.google.com/cloud-search/docs/guides/delegation/).");
            query0 = query0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("suggest").about("Provides suggestions for autocompleting the query.\n\n**Note:** This API requires a standard end user account to execute.\nA service account can\'t perform Query API requests directly; to use a\nservice account to perform queries, set up [G Suite domain-wide delegation\nof\nauthority](https://developers.google.com/cloud-search/docs/guides/delegation/).");
            query0 = query0.subcommand(mcmd);
        }
        let mut settings0 = SubCommand::with_name("settings")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: datasources and searchapplications");
        let mut stats0 = SubCommand::with_name("stats")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_index, get_query, get_session and get_user");
        {
            let mcmd = SubCommand::with_name("get_index").about("Gets indexed item statistics aggreggated across all data sources. This\nAPI only returns statistics for previous dates; it doesn\'t return\nstatistics for the current day.\n\n**Note:** This API requires a standard end user account to execute.");
            stats0 = stats0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_query").about("Get the query statistics for customer.\n\n**Note:** This API requires a standard end user account to execute.");
            stats0 = stats0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_session").about("Get the # of search sessions, % of successful sessions with a click query\nstatistics for customer.\n\n**Note:** This API requires a standard end user account to execute.");
            stats0 = stats0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_user").about("Get the users statistics for customer.\n\n**Note:** This API requires a standard end user account to execute.");
            stats0 = stats0.subcommand(mcmd);
        }
        let mut datasources1 = SubCommand::with_name("datasources")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: items");
        let mut identitysources1 = SubCommand::with_name("identitysources")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: items and unmappedids");
        let mut datasources1 = SubCommand::with_name("datasources")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete_schema, get_schema and update_schema");
        {
            let mcmd = SubCommand::with_name("delete_schema").about("Deletes the schema of a data source.\n\n**Note:** This API requires an admin or service account to execute.");
            datasources1 = datasources1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_schema").about("Gets the schema of a data source.\n\n**Note:** This API requires an admin or service account to execute.");
            datasources1 = datasources1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_schema").about("Updates the schema of a data source. This method does not perform\nincremental updates to the schema. Instead, this method updates the schema\nby overwriting the entire schema.\n\n**Note:** This API requires an admin or service account to execute.");
            datasources1 = datasources1.subcommand(mcmd);
        }
        let mut sources1 = SubCommand::with_name("sources")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Returns list of sources that user can use for Search and Suggest APIs.\n\n**Note:** This API requires a standard end user account to execute.\nA service account can\'t perform Query API requests directly; to use a\nservice account to perform queries, set up [G Suite domain-wide delegation\nof\nauthority](https://developers.google.com/cloud-search/docs/guides/delegation/).");
            sources1 = sources1.subcommand(mcmd);
        }
        let mut datasources1 = SubCommand::with_name("datasources")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("create").about(
                "Creates a datasource.\n\n**Note:** This API requires an admin account to execute.",
            );
            datasources1 = datasources1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about(
                "Deletes a datasource.\n\n**Note:** This API requires an admin account to execute.",
            );
            datasources1 = datasources1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about(
                "Gets a datasource.\n\n**Note:** This API requires an admin account to execute.",
            );
            datasources1 = datasources1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Lists datasources.\n\n**Note:** This API requires an admin account to execute.",
            );
            datasources1 = datasources1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about(
                "Updates a datasource.\n\n**Note:** This API requires an admin account to execute.",
            );
            datasources1 = datasources1.subcommand(mcmd);
        }
        let mut searchapplications1 = SubCommand::with_name("searchapplications")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, reset and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a search application.\n\n**Note:** This API requires an admin account to execute.");
            searchapplications1 = searchapplications1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a search application.\n\n**Note:** This API requires an admin account to execute.");
            searchapplications1 = searchapplications1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified search application.\n\n**Note:** This API requires an admin account to execute.");
            searchapplications1 = searchapplications1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all search applications.\n\n**Note:** This API requires an admin account to execute.");
            searchapplications1 = searchapplications1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reset").about("Resets a search application to default settings. This will return an empty\nresponse.\n\n**Note:** This API requires an admin account to execute.");
            searchapplications1 = searchapplications1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a search application.\n\n**Note:** This API requires an admin account to execute.");
            searchapplications1 = searchapplications1.subcommand(mcmd);
        }
        let mut index1 = SubCommand::with_name("index")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: datasources");
        let mut query1 = SubCommand::with_name("query")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: searchapplications");
        let mut session1 = SubCommand::with_name("session")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: searchapplications");
        let mut user1 = SubCommand::with_name("user")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: searchapplications");
        let mut items2 = SubCommand::with_name("items")
            .setting(AppSettings::ColoredHelp)
            .about("methods: check_access and search_by_view_url");
        {
            let mcmd = SubCommand::with_name("check_access").about("Checks whether an item is accessible by specified principal.\n\n**Note:** This API requires an admin account to execute.");
            items2 = items2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search_by_view_url").about("Fetches the item whose viewUrl exactly matches that of the URL provided\nin the request.\n\n**Note:** This API requires an admin account to execute.");
            items2 = items2.subcommand(mcmd);
        }
        let mut items2 = SubCommand::with_name("items")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list_forunmappedidentity");
        {
            let mcmd = SubCommand::with_name("list_forunmappedidentity").about("Lists names of items associated with an unmapped identity.\n\n**Note:** This API requires an admin account to execute.");
            items2 = items2.subcommand(mcmd);
        }
        let mut unmappedids2 = SubCommand::with_name("unmappedids")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists unmapped user identities for an identity source.\n\n**Note:** This API requires an admin account to execute.");
            unmappedids2 = unmappedids2.subcommand(mcmd);
        }
        let mut items2 = SubCommand::with_name("items")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: delete, delete_queue_items, get, index, list, poll, push, unreserve and upload");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes Item resource for the\nspecified resource name. This API requires an admin or service account\nto execute. The service account used is the one whitelisted in the\ncorresponding data source.");
            items2 = items2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete_queue_items").about("Deletes all items in a queue. This method is useful for deleting stale\nitems.\n\nThis API requires an admin or service account to execute. The service\naccount used is the one whitelisted in the corresponding data source.");
            items2 = items2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets Item resource by item name.\n\nThis API requires an admin or service account to execute.  The service\naccount used is the one whitelisted in the corresponding data source.");
            items2 = items2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("index").about("Updates Item ACL, metadata, and\ncontent. It will insert the Item if it\ndoes not exist.\nThis method does not support partial updates.  Fields with no provided\nvalues are cleared out in the Cloud Search index.\n\nThis API requires an admin or service account to execute. The service\naccount used is the one whitelisted in the corresponding data source.");
            items2 = items2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all or a subset of Item resources.\n\nThis API requires an admin or service account to execute. The service\naccount used is the one whitelisted in the corresponding data source.");
            items2 = items2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("poll").about("Polls for unreserved items from the indexing queue and marks a\nset as reserved, starting with items that have\nthe oldest timestamp from the highest priority\nItemStatus.\nThe priority order is as follows: <br />\nERROR\n<br />\nMODIFIED\n<br />\nNEW_ITEM\n<br />\nACCEPTED\n<br />\nReserving items ensures that polling from other threads\ncannot create overlapping sets.\n\nAfter handling the reserved items, the client should put items back\ninto the unreserved state, either by calling\nindex,\nor by calling\npush with\nthe type REQUEUE.\n\nItems automatically become available (unreserved) after 4 hours even if no\nupdate or push method is called.\n\nThis API requires an admin or service account to execute. The service\naccount used is the one whitelisted in the corresponding data source.");
            items2 = items2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("push").about("Pushes an item onto a queue for later polling and updating.\n\nThis API requires an admin or service account to execute. The service\naccount used is the one whitelisted in the corresponding data source.");
            items2 = items2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("unreserve").about("Unreserves all items from a queue, making them all eligible to be\npolled.  This method is useful for resetting the indexing queue\nafter a connector has been restarted.\n\nThis API requires an admin or service account to execute. The service\naccount used is the one whitelisted in the corresponding data source.");
            items2 = items2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("upload").about("Creates an upload session for uploading item content. For items smaller\nthan 100 KB, it\'s easier to embed the content\ninline within\nan index request.\n\nThis API requires an admin or service account to execute. The service\naccount used is the one whitelisted in the corresponding data source.");
            items2 = items2.subcommand(mcmd);
        }
        let mut datasources2 = SubCommand::with_name("datasources")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets indexed item statistics for a single data source.\n\n**Note:** This API requires a standard end user account to execute.");
            datasources2 = datasources2.subcommand(mcmd);
        }
        let mut searchapplications2 = SubCommand::with_name("searchapplications")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Get the query statistics for search application.\n\n**Note:** This API requires a standard end user account to execute.");
            searchapplications2 = searchapplications2.subcommand(mcmd);
        }
        let mut searchapplications2 = SubCommand::with_name("searchapplications")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Get the # of search sessions, % of successful sessions with a click query\nstatistics for search application.\n\n**Note:** This API requires a standard end user account to execute.");
            searchapplications2 = searchapplications2.subcommand(mcmd);
        }
        let mut searchapplications2 = SubCommand::with_name("searchapplications")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Get the users statistics for search application.\n\n**Note:** This API requires a standard end user account to execute.");
            searchapplications2 = searchapplications2.subcommand(mcmd);
        }
        let mut unmappedids3 = SubCommand::with_name("unmappedids")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("List all unmapped identities for a specific item.\n\n**Note:** This API requires an admin account to execute.");
            unmappedids3 = unmappedids3.subcommand(mcmd);
        }
        items2 = items2.subcommand(unmappedids3);
        user1 = user1.subcommand(searchapplications2);
        session1 = session1.subcommand(searchapplications2);
        query1 = query1.subcommand(searchapplications2);
        index1 = index1.subcommand(datasources2);
        datasources1 = datasources1.subcommand(items2);
        identitysources1 = identitysources1.subcommand(unmappedids2);
        identitysources1 = identitysources1.subcommand(items2);
        datasources1 = datasources1.subcommand(items2);
        stats0 = stats0.subcommand(user1);
        stats0 = stats0.subcommand(session1);
        stats0 = stats0.subcommand(query1);
        stats0 = stats0.subcommand(index1);
        settings0 = settings0.subcommand(searchapplications1);
        settings0 = settings0.subcommand(datasources1);
        query0 = query0.subcommand(sources1);
        indexing0 = indexing0.subcommand(datasources1);
        debug0 = debug0.subcommand(identitysources1);
        debug0 = debug0.subcommand(datasources1);
        app = app.subcommand(stats0);
        app = app.subcommand(settings0);
        app = app.subcommand(query0);
        app = app.subcommand(operations0);
        app = app.subcommand(media0);
        app = app.subcommand(indexing0);
        app = app.subcommand(debug0);

        Self { app }
    }
}
use google_cloudsearch1 as api;

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
