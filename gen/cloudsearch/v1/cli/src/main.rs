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
            .version("0.1.0-20210301")
            .about("Cloud Search provides cloud-based search capabilities over G Suite data. The Cloud Search API allows indexing of non-G Suite data into Cloud Search.")
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
            let mcmd = SubCommand::with_name("upload").about("Uploads media for indexing. The upload endpoint supports direct and resumable upload protocols and is intended for large items that can not be [inlined during index requests](https://developers.google.com/cloud-search/docs/reference/rest/v1/indexing.datasources.items#itemcontent). To index large content: 1. Call indexing.datasources.items.upload with the item name to begin an upload session and retrieve the UploadItemRef. 1. Call media.upload to upload the content, as a streaming request, using the same resource name from the UploadItemRef from step 1. 1. Call indexing.datasources.items.index to index the item. Populate the [ItemContent](/cloud-search/docs/reference/rest/v1/indexing.datasources.items#ItemContent) with the UploadItemRef from step 1. For additional information, see [Create a content connector using the REST API](https://developers.google.com/cloud-search/docs/guides/content-connector#rest). **Note:** This API requires a service account to execute.");
            media0 = media0.subcommand(mcmd);
        }
        let mut operations0 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations0 = operations0.subcommand(mcmd);
        }
        let mut query0 = SubCommand::with_name("query")
            .setting(AppSettings::ColoredHelp)
            .about("methods: search and suggest");
        {
            let mcmd = SubCommand::with_name("search").about("The Cloud Search Query API provides the search method, which returns the most relevant results from a user query. The results can come from G Suite Apps, such as Gmail or Google Drive, or they can come from data that you have indexed from a third party. **Note:** This API requires a standard end user account to execute. A service account can\'t perform Query API requests directly; to use a service account to perform queries, set up [G Suite domain-wide delegation of authority](https://developers.google.com/cloud-search/docs/guides/delegation/).");
            query0 = query0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("suggest").about("Provides suggestions for autocompleting the query. **Note:** This API requires a standard end user account to execute. A service account can\'t perform Query API requests directly; to use a service account to perform queries, set up [G Suite domain-wide delegation of authority](https://developers.google.com/cloud-search/docs/guides/delegation/).");
            query0 = query0.subcommand(mcmd);
        }
        let mut settings0 = SubCommand::with_name("settings")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: datasources and searchapplications");
        let mut stats0 = SubCommand::with_name("stats")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_index, get_query, get_session and get_user");
        {
            let mcmd = SubCommand::with_name("get_index").about("Gets indexed item statistics aggreggated across all data sources. This API only returns statistics for previous dates; it doesn\'t return statistics for the current day. **Note:** This API requires a standard end user account to execute.");
            stats0 = stats0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_query").about("Get the query statistics for customer. **Note:** This API requires a standard end user account to execute.");
            stats0 = stats0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_session").about("Get the # of search sessions, % of successful sessions with a click query statistics for customer. **Note:** This API requires a standard end user account to execute.");
            stats0 = stats0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_user").about("Get the users statistics for customer. **Note:** This API requires a standard end user account to execute.");
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
            let mcmd = SubCommand::with_name("delete_schema").about("Deletes the schema of a data source. **Note:** This API requires an admin or service account to execute.");
            datasources1 = datasources1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_schema").about("Gets the schema of a data source. **Note:** This API requires an admin or service account to execute.");
            datasources1 = datasources1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_schema").about("Updates the schema of a data source. This method does not perform incremental updates to the schema. Instead, this method updates the schema by overwriting the entire schema. **Note:** This API requires an admin or service account to execute.");
            datasources1 = datasources1.subcommand(mcmd);
        }
        let mut lro1 = SubCommand::with_name("lro")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn\'t support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            lro1 = lro1.subcommand(mcmd);
        }
        let mut sources1 = SubCommand::with_name("sources")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Returns list of sources that user can use for Search and Suggest APIs. **Note:** This API requires a standard end user account to execute. A service account can\'t perform Query API requests directly; to use a service account to perform queries, set up [G Suite domain-wide delegation of authority](https://developers.google.com/cloud-search/docs/guides/delegation/).");
            sources1 = sources1.subcommand(mcmd);
        }
        let mut datasources1 = SubCommand::with_name("datasources")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("create").about(
                "Creates a datasource. **Note:** This API requires an admin account to execute.",
            );
            datasources1 = datasources1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about(
                "Deletes a datasource. **Note:** This API requires an admin account to execute.",
            );
            datasources1 = datasources1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about(
                "Gets a datasource. **Note:** This API requires an admin account to execute.",
            );
            datasources1 = datasources1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Lists datasources. **Note:** This API requires an admin account to execute.",
            );
            datasources1 = datasources1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about(
                "Updates a datasource. **Note:** This API requires an admin account to execute.",
            );
            datasources1 = datasources1.subcommand(mcmd);
        }
        let mut searchapplications1 = SubCommand::with_name("searchapplications")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, reset and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a search application. **Note:** This API requires an admin account to execute.");
            searchapplications1 = searchapplications1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a search application. **Note:** This API requires an admin account to execute.");
            searchapplications1 = searchapplications1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified search application. **Note:** This API requires an admin account to execute.");
            searchapplications1 = searchapplications1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all search applications. **Note:** This API requires an admin account to execute.");
            searchapplications1 = searchapplications1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reset").about("Resets a search application to default settings. This will return an empty response. **Note:** This API requires an admin account to execute.");
            searchapplications1 = searchapplications1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a search application. **Note:** This API requires an admin account to execute.");
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
            let mcmd = SubCommand::with_name("check_access").about("Checks whether an item is accessible by specified principal. **Note:** This API requires an admin account to execute.");
            items2 = items2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search_by_view_url").about("Fetches the item whose viewUrl exactly matches that of the URL provided in the request. **Note:** This API requires an admin account to execute.");
            items2 = items2.subcommand(mcmd);
        }
        let mut items2 = SubCommand::with_name("items")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list_forunmappedidentity");
        {
            let mcmd = SubCommand::with_name("list_forunmappedidentity").about("Lists names of items associated with an unmapped identity. **Note:** This API requires an admin account to execute.");
            items2 = items2.subcommand(mcmd);
        }
        let mut unmappedids2 = SubCommand::with_name("unmappedids")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists unmapped user identities for an identity source. **Note:** This API requires an admin account to execute.");
            unmappedids2 = unmappedids2.subcommand(mcmd);
        }
        let mut items2 = SubCommand::with_name("items")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: delete, delete_queue_items, get, index, list, poll, push, unreserve and upload");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes Item resource for the specified resource name. This API requires an admin or service account to execute. The service account used is the one whitelisted in the corresponding data source.");
            items2 = items2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete_queue_items").about("Deletes all items in a queue. This method is useful for deleting stale items. This API requires an admin or service account to execute. The service account used is the one whitelisted in the corresponding data source.");
            items2 = items2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets Item resource by item name. This API requires an admin or service account to execute. The service account used is the one whitelisted in the corresponding data source.");
            items2 = items2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("index").about("Updates Item ACL, metadata, and content. It will insert the Item if it does not exist. This method does not support partial updates. Fields with no provided values are cleared out in the Cloud Search index. This API requires an admin or service account to execute. The service account used is the one whitelisted in the corresponding data source.");
            items2 = items2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all or a subset of Item resources. This API requires an admin or service account to execute. The service account used is the one whitelisted in the corresponding data source.");
            items2 = items2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("poll").about("Polls for unreserved items from the indexing queue and marks a set as reserved, starting with items that have the oldest timestamp from the highest priority ItemStatus. The priority order is as follows: ERROR MODIFIED NEW_ITEM ACCEPTED Reserving items ensures that polling from other threads cannot create overlapping sets. After handling the reserved items, the client should put items back into the unreserved state, either by calling index, or by calling push with the type REQUEUE. Items automatically become available (unreserved) after 4 hours even if no update or push method is called. This API requires an admin or service account to execute. The service account used is the one whitelisted in the corresponding data source.");
            items2 = items2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("push").about("Pushes an item onto a queue for later polling and updating. This API requires an admin or service account to execute. The service account used is the one whitelisted in the corresponding data source.");
            items2 = items2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("unreserve").about("Unreserves all items from a queue, making them all eligible to be polled. This method is useful for resetting the indexing queue after a connector has been restarted. This API requires an admin or service account to execute. The service account used is the one whitelisted in the corresponding data source.");
            items2 = items2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("upload").about("Creates an upload session for uploading item content. For items smaller than 100 KB, it\'s easier to embed the content inline within an index request. This API requires an admin or service account to execute. The service account used is the one whitelisted in the corresponding data source.");
            items2 = items2.subcommand(mcmd);
        }
        let mut datasources2 = SubCommand::with_name("datasources")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets indexed item statistics for a single data source. **Note:** This API requires a standard end user account to execute.");
            datasources2 = datasources2.subcommand(mcmd);
        }
        let mut searchapplications2 = SubCommand::with_name("searchapplications")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Get the query statistics for search application. **Note:** This API requires a standard end user account to execute.");
            searchapplications2 = searchapplications2.subcommand(mcmd);
        }
        let mut searchapplications2 = SubCommand::with_name("searchapplications")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Get the # of search sessions, % of successful sessions with a click query statistics for search application. **Note:** This API requires a standard end user account to execute.");
            searchapplications2 = searchapplications2.subcommand(mcmd);
        }
        let mut searchapplications2 = SubCommand::with_name("searchapplications")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Get the users statistics for search application. **Note:** This API requires a standard end user account to execute.");
            searchapplications2 = searchapplications2.subcommand(mcmd);
        }
        let mut unmappedids3 = SubCommand::with_name("unmappedids")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("List all unmapped identities for a specific item. **Note:** This API requires an admin account to execute.");
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
        operations0 = operations0.subcommand(lro1);
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
