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
        let mut app = App::new("displayvideo1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200623")
            .about("Display & Video 360 API allows users to manage and create campaigns and reports.")
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
        let mut advertisers0 = SubCommand::with_name("advertisers")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: bulk_edit_advertiser_assigned_targeting_options, bulk_list_advertiser_assigned_targeting_options, create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("bulk_edit_advertiser_assigned_targeting_options").about("Bulk edits targeting options under a single advertiser.\nThe operation will delete the assigned targeting options provided in\nBulkEditAdvertiserAssignedTargetingOptionsRequest.delete_requests and\nthen create the assigned targeting options provided in\nBulkEditAdvertiserAssignedTargetingOptionsRequest.create_requests .");
            advertisers0 = advertisers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("bulk_list_advertiser_assigned_targeting_options")
                .about("Lists assigned targeting options of an advertiser across targeting types.");
            advertisers0 = advertisers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new advertiser.\nReturns the newly created advertiser if successful.\nThis method can take up to 180 seconds to complete.");
            advertisers0 = advertisers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an advertiser.\nDeleting an advertiser will delete all of its child resources, for example,\ncampaigns, insertion orders and line items.\nA deleted advertiser cannot be recovered.");
            advertisers0 = advertisers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an advertiser.");
            advertisers0 = advertisers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists advertisers that are accessible to the current user.\n\nThe order is defined by the order_by\nparameter.\n\nA single partner_id is required.\nCross-partner listing is not supported.");
            advertisers0 = advertisers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Updates an existing advertiser.\nReturns the updated advertiser if successful.",
            );
            advertisers0 = advertisers0.subcommand(mcmd);
        }
        let mut combined_audiences0 = SubCommand::with_name("combined_audiences")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a combined audience.");
            combined_audiences0 = combined_audiences0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Lists combined audiences.\n\nThe order is defined by the\norder_by parameter.",
            );
            combined_audiences0 = combined_audiences0.subcommand(mcmd);
        }
        let mut custom_lists0 = SubCommand::with_name("custom_lists")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a custom list.");
            custom_lists0 = custom_lists0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists custom lists.\n\nThe order is defined by the order_by\nparameter.");
            custom_lists0 = custom_lists0.subcommand(mcmd);
        }
        let mut first_and_third_party_audiences0 =
            SubCommand::with_name("first_and_third_party_audiences")
                .setting(AppSettings::ColoredHelp)
                .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a first and third party audience.");
            first_and_third_party_audiences0 = first_and_third_party_audiences0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists first and third party audiences.\n\nThe order is defined by the\norder_by parameter.");
            first_and_third_party_audiences0 = first_and_third_party_audiences0.subcommand(mcmd);
        }
        let mut floodlight_groups0 = SubCommand::with_name("floodlight_groups")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and patch");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a Floodlight group.");
            floodlight_groups0 = floodlight_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing Floodlight group.\nReturns the updated Floodlight group if successful.");
            floodlight_groups0 = floodlight_groups0.subcommand(mcmd);
        }
        let mut google_audiences0 = SubCommand::with_name("google_audiences")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a Google audience.");
            google_audiences0 = google_audiences0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Lists Google audiences.\n\nThe order is defined by the order_by\nparameter.",
            );
            google_audiences0 = google_audiences0.subcommand(mcmd);
        }
        let mut inventory_source_groups0 = SubCommand::with_name("inventory_source_groups")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new inventory source group. Returns the newly created inventory\nsource group if successful.");
            inventory_source_groups0 = inventory_source_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an inventory source group.");
            inventory_source_groups0 = inventory_source_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an inventory source group.");
            inventory_source_groups0 = inventory_source_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists inventory source groups that are accessible to the current user.\n\nThe order is defined by the\norder_by parameter.");
            inventory_source_groups0 = inventory_source_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an inventory source group. Returns the updated inventory source\ngroup if successful.");
            inventory_source_groups0 = inventory_source_groups0.subcommand(mcmd);
        }
        let mut inventory_sources0 = SubCommand::with_name("inventory_sources")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets an inventory source.");
            inventory_sources0 = inventory_sources0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists inventory sources that are accessible to the current user.\n\nThe order is defined by the\norder_by parameter.\nIf a filter by\nentity_status is not\nspecified, inventory sources with entity status `ENTITY_STATUS_ARCHIVED`\nwill not be included in the results.");
            inventory_sources0 = inventory_sources0.subcommand(mcmd);
        }
        let mut media0 = SubCommand::with_name("media")
            .setting(AppSettings::ColoredHelp)
            .about("methods: download");
        {
            let mcmd = SubCommand::with_name("download").about("Downloads media. Download is supported on the URI `/download/{resource_name=**}?alt=media.`\n\n**Note**: Download requests will not be successful without including `alt=media` query string.");
            media0 = media0.subcommand(mcmd);
        }
        let mut partners0 = SubCommand::with_name("partners")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: channels");
        let mut sdfdownloadtasks0 = SubCommand::with_name("sdfdownloadtasks")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an SDF Download Task. Returns an\nOperation.\n\nAn SDF Download Task is a long-running, asynchronous operation. The\nmetadata type of this operation is\nSdfDownloadTaskMetadata. If the request is successful, the\nresponse type of the operation is\nSdfDownloadTask. The response will not include the download files,\nwhich must be retrieved with\nmedia.download. The state of\noperation can be retrieved with\nsdfdownloadtask.operations.get.\n\nAny errors can be found in the\nerror.message. Note\nthat error.details is expected to be\nempty.");
            sdfdownloadtasks0 = sdfdownloadtasks0.subcommand(mcmd);
        }
        let mut targeting_types0 = SubCommand::with_name("targeting_types")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: targeting_options");
        let mut assets1 = SubCommand::with_name("assets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: upload");
        {
            let mcmd = SubCommand::with_name("upload").about("Uploads an asset.\nReturns the ID of the newly uploaded asset if successful.\nThe asset file size should be no more than 10 MB for images, 200 MB for\nZIP files, and 1 GB for videos.");
            assets1 = assets1.subcommand(mcmd);
        }
        let mut campaigns1 = SubCommand::with_name("campaigns")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about(
                "Creates a new campaign.\nReturns the newly created campaign if successful.",
            );
            campaigns1 = campaigns1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Permanently deletes a campaign. A deleted campaign cannot be recovered.\nThe campaign should be archived first, i.e. set\nentity_status to `ENTITY_STATUS_ARCHIVED`, to be\nable to delete it.");
            campaigns1 = campaigns1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a campaign.");
            campaigns1 = campaigns1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists campaigns in an advertiser.\n\nThe order is defined by the order_by\nparameter.\nIf a filter by\nentity_status is not specified, campaigns with\n`ENTITY_STATUS_ARCHIVED` will not be included in the results.");
            campaigns1 = campaigns1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Updates an existing campaign.\nReturns the updated campaign if successful.",
            );
            campaigns1 = campaigns1.subcommand(mcmd);
        }
        let mut channels1 = SubCommand::with_name("channels")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new channel. Returns the newly created channel if successful.");
            channels1 = channels1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Updates an existing inventory source.\nReturns the updated inventory source if successful.\nGets a channel for a partner or advertiser.");
            channels1 = channels1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists channels for a partner or advertiser.");
            channels1 = channels1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates a channel. Returns the updated channel if successful.");
            channels1 = channels1.subcommand(mcmd);
        }
        let mut creatives1 = SubCommand::with_name("creatives")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about(
                "Creates a new creative.\nReturns the newly created creative if successful.",
            );
            creatives1 = creatives1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a creative.\nReturns error code `NOT_FOUND` if the creative does not exist.\nThe creative should be archived first, i.e. set\nentity_status to `ENTITY_STATUS_ARCHIVED`, before\nit can be deleted.");
            creatives1 = creatives1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a creative.");
            creatives1 = creatives1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists creatives in an advertiser.\n\nThe order is defined by the order_by\nparameter.\nIf a filter by\nentity_status is not specified, creatives with\n`ENTITY_STATUS_ARCHIVED` will not be included in the results.");
            creatives1 = creatives1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Updates an existing creative.\nReturns the updated creative if successful.",
            );
            creatives1 = creatives1.subcommand(mcmd);
        }
        let mut insertion_orders1 = SubCommand::with_name("insertion_orders")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new insertion order.\nReturns the newly created insertion order if successful.");
            insertion_orders1 = insertion_orders1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an insertion order.\nReturns error code `NOT_FOUND` if the insertion order does not exist.\nThe insertion order should be archived first, i.e. set\nentity_status to `ENTITY_STATUS_ARCHIVED`,\nto be able to delete it.");
            insertion_orders1 = insertion_orders1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an insertion order.\nReturns error code `NOT_FOUND` if the insertion order does not exist.");
            insertion_orders1 = insertion_orders1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists insertion orders in an advertiser.\n\nThe order is defined by the order_by\nparameter.\nIf a filter by\nentity_status is not specified, insertion\norders with `ENTITY_STATUS_ARCHIVED` will not be included in the results.");
            insertion_orders1 = insertion_orders1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing insertion order.\nReturns the updated insertion order if successful.");
            insertion_orders1 = insertion_orders1.subcommand(mcmd);
        }
        let mut line_items1 = SubCommand::with_name("line_items")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: bulk_edit_line_item_assigned_targeting_options, bulk_list_line_item_assigned_targeting_options, create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("bulk_edit_line_item_assigned_targeting_options").about("Bulk edits targeting options under a single line item.\nThe operation will delete the assigned targeting options provided in\nBulkEditLineItemAssignedTargetingOptionsRequest.delete_requests and\nthen create the assigned targeting options provided in\nBulkEditLineItemAssignedTargetingOptionsRequest.create_requests .");
            line_items1 = line_items1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("bulk_list_line_item_assigned_targeting_options")
                .about("Lists assigned targeting options of a line item across targeting types.");
            line_items1 = line_items1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about(
                "Creates a new line item.\nReturns the newly created line item if successful.",
            );
            line_items1 = line_items1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a line item.\nReturns error code `NOT_FOUND` if the line item does not exist.\nThe line item should be archived first, i.e. set\nentity_status to `ENTITY_STATUS_ARCHIVED`, to be\nable to delete it.");
            line_items1 = line_items1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a line item.");
            line_items1 = line_items1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists line items in an advertiser.\n\nThe order is defined by the order_by\nparameter.\nIf a filter by\nentity_status is not specified, line items with\n`ENTITY_STATUS_ARCHIVED` will not be included in the results.");
            line_items1 = line_items1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Updates an existing line item.\nReturns the updated line item if successful.",
            );
            line_items1 = line_items1.subcommand(mcmd);
        }
        let mut location_lists1 = SubCommand::with_name("location_lists")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new location list. Returns the newly created location list if\nsuccessful.");
            location_lists1 = location_lists1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a location list.");
            location_lists1 = location_lists1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists location lists based on a given advertiser id.");
            location_lists1 = location_lists1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates a location list. Returns the updated location list if successful.");
            location_lists1 = location_lists1.subcommand(mcmd);
        }
        let mut negative_keyword_lists1 = SubCommand::with_name("negative_keyword_lists")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new negative keyword list. Returns the newly created negative\nkeyword list if successful.");
            negative_keyword_lists1 = negative_keyword_lists1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a negative keyword list given an advertiser ID and a negative\nkeyword list ID.");
            negative_keyword_lists1 = negative_keyword_lists1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a negative keyword list given an advertiser ID and a negative keyword\nlist ID.");
            negative_keyword_lists1 = negative_keyword_lists1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists negative keyword lists based on a given advertiser id.");
            negative_keyword_lists1 = negative_keyword_lists1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a negative keyword list. Returns the updated negative keyword list\nif successful.");
            negative_keyword_lists1 = negative_keyword_lists1.subcommand(mcmd);
        }
        let mut targeting_types1 = SubCommand::with_name("targeting_types")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: assigned_targeting_options");
        let mut assigned_inventory_sources1 = SubCommand::with_name("assigned_inventory_sources")
            .setting(AppSettings::ColoredHelp)
            .about("methods: bulk_edit, create, delete and list");
        {
            let mcmd = SubCommand::with_name("bulk_edit").about("Bulk edits multiple assignments between inventory sources and a single\ninventory source group.\n\nThe operation will delete the assigned inventory sources provided in\nBulkEditAssignedInventorySourcesRequest.deleted_assigned_inventory_sources\nand then create the assigned inventory sources provided in\nBulkEditAssignedInventorySourcesRequest.created_assigned_inventory_sources.");
            assigned_inventory_sources1 = assigned_inventory_sources1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about(
                "Creates an assignment between an inventory source and an inventory source\ngroup.",
            );
            assigned_inventory_sources1 = assigned_inventory_sources1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the assignment between an inventory source and an inventory source\ngroup.");
            assigned_inventory_sources1 = assigned_inventory_sources1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists inventory sources assigned to an inventory source group.");
            assigned_inventory_sources1 = assigned_inventory_sources1.subcommand(mcmd);
        }
        let mut channels1 = SubCommand::with_name("channels")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new channel. Returns the newly created channel if successful.");
            channels1 = channels1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Updates an existing inventory source.\nReturns the updated inventory source if successful.\nGets a channel for a partner or advertiser.");
            channels1 = channels1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists channels for a partner or advertiser.");
            channels1 = channels1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates a channel. Returns the updated channel if successful.");
            channels1 = channels1.subcommand(mcmd);
        }
        let mut operations1 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of an asynchronous SDF download task operation. Clients should poll this method at intervals of 30 seconds.");
            operations1 = operations1.subcommand(mcmd);
        }
        let mut targeting_options1 = SubCommand::with_name("targeting_options")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a single targeting option.");
            targeting_options1 = targeting_options1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists targeting options of a given type.");
            targeting_options1 = targeting_options1.subcommand(mcmd);
        }
        let mut sites2 = SubCommand::with_name("sites")
            .setting(AppSettings::ColoredHelp)
            .about("methods: bulk_edit, create, delete and list");
        {
            let mcmd = SubCommand::with_name("bulk_edit").about("Bulk edits sites under a single channel.\n\nThe operation will delete the sites provided in\nBulkEditSitesRequest.deleted_sites and then create the sites\nprovided in BulkEditSitesRequest.created_sites.");
            sites2 = sites2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a site in a channel.");
            sites2 = sites2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a site from a channel.");
            sites2 = sites2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists sites in a channel.");
            sites2 = sites2.subcommand(mcmd);
        }
        let mut targeting_types2 = SubCommand::with_name("targeting_types")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: assigned_targeting_options");
        let mut assigned_locations2 = SubCommand::with_name("assigned_locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: bulk_edit, create, delete and list");
        {
            let mcmd = SubCommand::with_name("bulk_edit").about("Bulk edits multiple assignments between locations and a single location\nlist.\n\nThe operation will delete the assigned locations provided in\nBulkEditAssignedLocationsRequest.deleted_assigned_locations and then\ncreate the assigned locations provided in\nBulkEditAssignedLocationsRequest.created_assigned_locations.");
            assigned_locations2 = assigned_locations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates an assignment between a location and a location list.");
            assigned_locations2 = assigned_locations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the assignment between a location and a location list.");
            assigned_locations2 = assigned_locations2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists locations assigned to a location list.");
            assigned_locations2 = assigned_locations2.subcommand(mcmd);
        }
        let mut negative_keywords2 = SubCommand::with_name("negative_keywords")
            .setting(AppSettings::ColoredHelp)
            .about("methods: bulk_edit, create, delete and list");
        {
            let mcmd = SubCommand::with_name("bulk_edit").about("Bulk edits negative keywords in a single negative keyword list.\n\nThe operation will delete the negative keywords provided in\nBulkEditNegativeKeywordsRequest.deleted_negative_keywords and then\ncreate the negative keywords provided in\nBulkEditNegativeKeywordsRequest.created_negative_keywords.\n\nThis operation is guaranteed to be atomic and will never result in a\npartial success or partial failure.");
            negative_keywords2 = negative_keywords2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a negative keyword in a negative keyword list.");
            negative_keywords2 = negative_keywords2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes a negative keyword from a negative keyword list.");
            negative_keywords2 = negative_keywords2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists negative keywords in a negative keyword list.");
            negative_keywords2 = negative_keywords2.subcommand(mcmd);
        }
        let mut assigned_targeting_options2 = SubCommand::with_name("assigned_targeting_options")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Assigns a targeting option to an advertiser.\nReturns the assigned targeting option if successful.");
            assigned_targeting_options2 = assigned_targeting_options2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes an assigned targeting option from an advertiser.");
            assigned_targeting_options2 = assigned_targeting_options2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets a single targeting option assigned to an advertiser.");
            assigned_targeting_options2 = assigned_targeting_options2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the targeting options assigned to an advertiser.");
            assigned_targeting_options2 = assigned_targeting_options2.subcommand(mcmd);
        }
        let mut sites2 = SubCommand::with_name("sites")
            .setting(AppSettings::ColoredHelp)
            .about("methods: bulk_edit, create, delete and list");
        {
            let mcmd = SubCommand::with_name("bulk_edit").about("Bulk edits sites under a single channel.\n\nThe operation will delete the sites provided in\nBulkEditSitesRequest.deleted_sites and then create the sites\nprovided in BulkEditSitesRequest.created_sites.");
            sites2 = sites2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a site in a channel.");
            sites2 = sites2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a site from a channel.");
            sites2 = sites2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists sites in a channel.");
            sites2 = sites2.subcommand(mcmd);
        }
        let mut assigned_targeting_options3 = SubCommand::with_name("assigned_targeting_options")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Assigns a targeting option to a line item.\nReturns the assigned targeting option if successful.");
            assigned_targeting_options3 = assigned_targeting_options3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes an assigned targeting option from a line item.");
            assigned_targeting_options3 = assigned_targeting_options3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets a single targeting option assigned to a line item.");
            assigned_targeting_options3 = assigned_targeting_options3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the targeting options assigned to a line item.");
            assigned_targeting_options3 = assigned_targeting_options3.subcommand(mcmd);
        }
        targeting_types2 = targeting_types2.subcommand(assigned_targeting_options3);
        channels1 = channels1.subcommand(sites2);
        targeting_types1 = targeting_types1.subcommand(assigned_targeting_options2);
        negative_keyword_lists1 = negative_keyword_lists1.subcommand(negative_keywords2);
        location_lists1 = location_lists1.subcommand(assigned_locations2);
        line_items1 = line_items1.subcommand(targeting_types2);
        channels1 = channels1.subcommand(sites2);
        targeting_types0 = targeting_types0.subcommand(targeting_options1);
        sdfdownloadtasks0 = sdfdownloadtasks0.subcommand(operations1);
        partners0 = partners0.subcommand(channels1);
        inventory_source_groups0 = inventory_source_groups0.subcommand(assigned_inventory_sources1);
        advertisers0 = advertisers0.subcommand(targeting_types1);
        advertisers0 = advertisers0.subcommand(negative_keyword_lists1);
        advertisers0 = advertisers0.subcommand(location_lists1);
        advertisers0 = advertisers0.subcommand(line_items1);
        advertisers0 = advertisers0.subcommand(insertion_orders1);
        advertisers0 = advertisers0.subcommand(creatives1);
        advertisers0 = advertisers0.subcommand(channels1);
        advertisers0 = advertisers0.subcommand(campaigns1);
        advertisers0 = advertisers0.subcommand(assets1);
        app = app.subcommand(targeting_types0);
        app = app.subcommand(sdfdownloadtasks0);
        app = app.subcommand(partners0);
        app = app.subcommand(media0);
        app = app.subcommand(inventory_sources0);
        app = app.subcommand(inventory_source_groups0);
        app = app.subcommand(google_audiences0);
        app = app.subcommand(floodlight_groups0);
        app = app.subcommand(first_and_third_party_audiences0);
        app = app.subcommand(custom_lists0);
        app = app.subcommand(combined_audiences0);
        app = app.subcommand(advertisers0);

        Self { app }
    }
}
use google_displayvideo1 as api;

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
