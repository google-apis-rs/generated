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
            .version("0.1.0-20210312")
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
                        .about("methods: audit, bulk_edit_advertiser_assigned_targeting_options, bulk_list_advertiser_assigned_targeting_options, create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("audit").about("Audits an advertiser. Returns the counts of used entities per resource type under the advertiser provided. Used entities count towards their respective resource limit. See https://support.google.com/displayvideo/answer/6071450.");
            advertisers0 = advertisers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("bulk_edit_advertiser_assigned_targeting_options").about("Bulk edits targeting options under a single advertiser. The operation will delete the assigned targeting options provided in BulkEditAdvertiserAssignedTargetingOptionsRequest.delete_requests and then create the assigned targeting options provided in BulkEditAdvertiserAssignedTargetingOptionsRequest.create_requests .");
            advertisers0 = advertisers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("bulk_list_advertiser_assigned_targeting_options")
                .about("Lists assigned targeting options of an advertiser across targeting types.");
            advertisers0 = advertisers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new advertiser. Returns the newly created advertiser if successful. This method can take up to 180 seconds to complete.");
            advertisers0 = advertisers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an advertiser. Deleting an advertiser will delete all of its child resources, for example, campaigns, insertion orders and line items. A deleted advertiser cannot be recovered.");
            advertisers0 = advertisers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an advertiser.");
            advertisers0 = advertisers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists advertisers that are accessible to the current user. The order is defined by the order_by parameter. A single partner_id is required. Cross-partner listing is not supported.");
            advertisers0 = advertisers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Updates an existing advertiser. Returns the updated advertiser if successful.",
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
            let mcmd = SubCommand::with_name("list")
                .about("Lists combined audiences. The order is defined by the order_by parameter.");
            combined_audiences0 = combined_audiences0.subcommand(mcmd);
        }
        let mut custom_bidding_algorithms0 = SubCommand::with_name("custom_bidding_algorithms")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a custom bidding algorithm.");
            custom_bidding_algorithms0 = custom_bidding_algorithms0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists custom bidding algorithms that are accessible to the current user and can be used in bidding stratgies. The order is defined by the order_by parameter.");
            custom_bidding_algorithms0 = custom_bidding_algorithms0.subcommand(mcmd);
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
                .about("Lists custom lists. The order is defined by the order_by parameter.");
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
            let mcmd = SubCommand::with_name("list").about("Lists first and third party audiences. The order is defined by the order_by parameter.");
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
            let mcmd = SubCommand::with_name("patch").about("Updates an existing Floodlight group. Returns the updated Floodlight group if successful.");
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
            let mcmd = SubCommand::with_name("list")
                .about("Lists Google audiences. The order is defined by the order_by parameter.");
            google_audiences0 = google_audiences0.subcommand(mcmd);
        }
        let mut inventory_source_groups0 = SubCommand::with_name("inventory_source_groups")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new inventory source group. Returns the newly created inventory source group if successful.");
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
            let mcmd = SubCommand::with_name("list").about("Lists inventory source groups that are accessible to the current user. The order is defined by the order_by parameter.");
            inventory_source_groups0 = inventory_source_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an inventory source group. Returns the updated inventory source group if successful.");
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
            let mcmd = SubCommand::with_name("list").about("Lists inventory sources that are accessible to the current user. The order is defined by the order_by parameter. If a filter by entity_status is not specified, inventory sources with entity status `ENTITY_STATUS_ARCHIVED` will not be included in the results.");
            inventory_sources0 = inventory_sources0.subcommand(mcmd);
        }
        let mut media0 = SubCommand::with_name("media")
            .setting(AppSettings::ColoredHelp)
            .about("methods: download");
        {
            let mcmd = SubCommand::with_name("download").about("Downloads media. Download is supported on the URI `/download/{resource_name=**}?alt=media.` **Note**: Download requests will not be successful without including `alt=media` query string.");
            media0 = media0.subcommand(mcmd);
        }
        let mut partners0 = SubCommand::with_name("partners")
            .setting(AppSettings::ColoredHelp)
            .about("methods: bulk_edit_partner_assigned_targeting_options, get and list");
        {
            let mcmd = SubCommand::with_name("bulk_edit_partner_assigned_targeting_options").about("Bulk edits targeting options under a single partner. The operation will delete the assigned targeting options provided in BulkEditPartnerAssignedTargetingOptionsRequest.deleteRequests and then create the assigned targeting options provided in BulkEditPartnerAssignedTargetingOptionsRequest.createRequests .");
            partners0 = partners0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a partner.");
            partners0 = partners0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists partners that are accessible to the current user. The order is defined by the order_by parameter.");
            partners0 = partners0.subcommand(mcmd);
        }
        let mut sdfdownloadtasks0 = SubCommand::with_name("sdfdownloadtasks")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an SDF Download Task. Returns an Operation. An SDF Download Task is a long-running, asynchronous operation. The metadata type of this operation is SdfDownloadTaskMetadata. If the request is successful, the response type of the operation is SdfDownloadTask. The response will not include the download files, which must be retrieved with media.download. The state of operation can be retrieved with sdfdownloadtask.operations.get. Any errors can be found in the error.message. Note that error.details is expected to be empty.");
            sdfdownloadtasks0 = sdfdownloadtasks0.subcommand(mcmd);
        }
        let mut targeting_types0 = SubCommand::with_name("targeting_types")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: targeting_options");
        let mut users0 = SubCommand::with_name("users")
            .setting(AppSettings::ColoredHelp)
            .about("methods: bulk_edit_assigned_user_roles, create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("bulk_edit_assigned_user_roles").about("Bulk edits user roles for a user. The operation will delete the assigned user roles provided in BulkEditAssignedUserRolesRequest.deletedAssignedUserRoles and then assign the user roles provided in BulkEditAssignedUserRolesRequest.createdAssignedUserRoles.");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new user. Returns the newly created user if successful.");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a user.");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a user.");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists users that are accessible to the current user. If two users have user roles on the same partner or advertiser, they can access each other.");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates an existing user. Returns the updated user if successful.");
            users0 = users0.subcommand(mcmd);
        }
        let mut assets1 = SubCommand::with_name("assets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: upload");
        {
            let mcmd = SubCommand::with_name("upload").about("Uploads an asset. Returns the ID of the newly uploaded asset if successful. The asset file size should be no more than 10 MB for images, 200 MB for ZIP files, and 1 GB for videos.");
            assets1 = assets1.subcommand(mcmd);
        }
        let mut campaigns1 = SubCommand::with_name("campaigns")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new campaign. Returns the newly created campaign if successful.");
            campaigns1 = campaigns1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Permanently deletes a campaign. A deleted campaign cannot be recovered. The campaign should be archived first, i.e. set entity_status to `ENTITY_STATUS_ARCHIVED`, to be able to delete it.");
            campaigns1 = campaigns1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a campaign.");
            campaigns1 = campaigns1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists campaigns in an advertiser. The order is defined by the order_by parameter. If a filter by entity_status is not specified, campaigns with `ENTITY_STATUS_ARCHIVED` will not be included in the results.");
            campaigns1 = campaigns1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates an existing campaign. Returns the updated campaign if successful.");
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
            let mcmd =
                SubCommand::with_name("get").about("Gets a channel for a partner or advertiser.");
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
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new creative. Returns the newly created creative if successful.");
            creatives1 = creatives1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a creative. Returns error code `NOT_FOUND` if the creative does not exist. The creative should be archived first, i.e. set entity_status to `ENTITY_STATUS_ARCHIVED`, before it can be deleted.");
            creatives1 = creatives1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a creative.");
            creatives1 = creatives1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists creatives in an advertiser. The order is defined by the order_by parameter. If a filter by entity_status is not specified, creatives with `ENTITY_STATUS_ARCHIVED` will not be included in the results.");
            creatives1 = creatives1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates an existing creative. Returns the updated creative if successful.");
            creatives1 = creatives1.subcommand(mcmd);
        }
        let mut insertion_orders1 = SubCommand::with_name("insertion_orders")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: bulk_list_insertion_order_assigned_targeting_options, create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name(
                "bulk_list_insertion_order_assigned_targeting_options",
            )
            .about(
                "Lists assigned targeting options of an insertion order across targeting types.",
            );
            insertion_orders1 = insertion_orders1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new insertion order. Returns the newly created insertion order if successful.");
            insertion_orders1 = insertion_orders1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an insertion order. Returns error code `NOT_FOUND` if the insertion order does not exist. The insertion order should be archived first, i.e. set entity_status to `ENTITY_STATUS_ARCHIVED`, to be able to delete it.");
            insertion_orders1 = insertion_orders1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an insertion order. Returns error code `NOT_FOUND` if the insertion order does not exist.");
            insertion_orders1 = insertion_orders1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists insertion orders in an advertiser. The order is defined by the order_by parameter. If a filter by entity_status is not specified, insertion orders with `ENTITY_STATUS_ARCHIVED` will not be included in the results.");
            insertion_orders1 = insertion_orders1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing insertion order. Returns the updated insertion order if successful.");
            insertion_orders1 = insertion_orders1.subcommand(mcmd);
        }
        let mut line_items1 = SubCommand::with_name("line_items")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: bulk_edit_line_item_assigned_targeting_options, bulk_list_line_item_assigned_targeting_options, create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("bulk_edit_line_item_assigned_targeting_options").about("Bulk edits targeting options under a single line item. The operation will delete the assigned targeting options provided in BulkEditLineItemAssignedTargetingOptionsRequest.delete_requests and then create the assigned targeting options provided in BulkEditLineItemAssignedTargetingOptionsRequest.create_requests .");
            line_items1 = line_items1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("bulk_list_line_item_assigned_targeting_options")
                .about("Lists assigned targeting options of a line item across targeting types.");
            line_items1 = line_items1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about(
                "Creates a new line item. Returns the newly created line item if successful.",
            );
            line_items1 = line_items1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a line item. Returns error code `NOT_FOUND` if the line item does not exist. The line item should be archived first, i.e. set entity_status to `ENTITY_STATUS_ARCHIVED`, to be able to delete it.");
            line_items1 = line_items1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a line item.");
            line_items1 = line_items1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists line items in an advertiser. The order is defined by the order_by parameter. If a filter by entity_status is not specified, line items with `ENTITY_STATUS_ARCHIVED` will not be included in the results.");
            line_items1 = line_items1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Updates an existing line item. Returns the updated line item if successful.",
            );
            line_items1 = line_items1.subcommand(mcmd);
        }
        let mut location_lists1 = SubCommand::with_name("location_lists")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new location list. Returns the newly created location list if successful.");
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
        let mut manual_triggers1 = SubCommand::with_name("manual_triggers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: activate, create, deactivate, get, list and patch");
        {
            let mcmd = SubCommand::with_name("activate").about("Activates a manual trigger. Each activation of the manual trigger must be at least 5 minutes apart, otherwise an error will be returned.");
            manual_triggers1 = manual_triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new manual trigger. Returns the newly created manual trigger if successful.");
            manual_triggers1 = manual_triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("deactivate").about("Deactivates a manual trigger.");
            manual_triggers1 = manual_triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a manual trigger.");
            manual_triggers1 = manual_triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists manual triggers that are accessible to the current user for a given advertiser ID. The order is defined by the order_by parameter. A single advertiser_id is required.");
            manual_triggers1 = manual_triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Updates a manual trigger. Returns the updated manual trigger if successful.",
            );
            manual_triggers1 = manual_triggers1.subcommand(mcmd);
        }
        let mut negative_keyword_lists1 = SubCommand::with_name("negative_keyword_lists")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new negative keyword list. Returns the newly created negative keyword list if successful.");
            negative_keyword_lists1 = negative_keyword_lists1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a negative keyword list given an advertiser ID and a negative keyword list ID.");
            negative_keyword_lists1 = negative_keyword_lists1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a negative keyword list given an advertiser ID and a negative keyword list ID.");
            negative_keyword_lists1 = negative_keyword_lists1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists negative keyword lists based on a given advertiser id.");
            negative_keyword_lists1 = negative_keyword_lists1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a negative keyword list. Returns the updated negative keyword list if successful.");
            negative_keyword_lists1 = negative_keyword_lists1.subcommand(mcmd);
        }
        let mut targeting_types1 = SubCommand::with_name("targeting_types")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: assigned_targeting_options");
        let mut assigned_inventory_sources1 = SubCommand::with_name("assigned_inventory_sources")
            .setting(AppSettings::ColoredHelp)
            .about("methods: bulk_edit, create, delete and list");
        {
            let mcmd = SubCommand::with_name("bulk_edit").about("Bulk edits multiple assignments between inventory sources and a single inventory source group. The operation will delete the assigned inventory sources provided in BulkEditAssignedInventorySourcesRequest.deleted_assigned_inventory_sources and then create the assigned inventory sources provided in BulkEditAssignedInventorySourcesRequest.created_assigned_inventory_sources.");
            assigned_inventory_sources1 = assigned_inventory_sources1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about(
                "Creates an assignment between an inventory source and an inventory source group.",
            );
            assigned_inventory_sources1 = assigned_inventory_sources1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about(
                "Deletes the assignment between an inventory source and an inventory source group.",
            );
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
            let mcmd =
                SubCommand::with_name("get").about("Gets a channel for a partner or advertiser.");
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
        let mut targeting_types1 = SubCommand::with_name("targeting_types")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: assigned_targeting_options");
        let mut operations1 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of an asynchronous SDF download task operation. Clients should poll this method at intervals of 30 seconds.");
            operations1 = operations1.subcommand(mcmd);
        }
        let mut targeting_options1 = SubCommand::with_name("targeting_options")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list and search");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a single targeting option.");
            targeting_options1 = targeting_options1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists targeting options of a given type.");
            targeting_options1 = targeting_options1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search").about(
                "Searches for targeting options of a given type based on the given search terms.",
            );
            targeting_options1 = targeting_options1.subcommand(mcmd);
        }
        let mut sites2 = SubCommand::with_name("sites")
            .setting(AppSettings::ColoredHelp)
            .about("methods: bulk_edit, create, delete and list");
        {
            let mcmd = SubCommand::with_name("bulk_edit").about("Bulk edits sites under a single channel. The operation will delete the sites provided in BulkEditSitesRequest.deleted_sites and then create the sites provided in BulkEditSitesRequest.created_sites.");
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
        let mut targeting_types2 = SubCommand::with_name("targeting_types")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: assigned_targeting_options");
        let mut assigned_locations2 = SubCommand::with_name("assigned_locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: bulk_edit, create, delete and list");
        {
            let mcmd = SubCommand::with_name("bulk_edit").about("Bulk edits multiple assignments between locations and a single location list. The operation will delete the assigned locations provided in BulkEditAssignedLocationsRequest.deleted_assigned_locations and then create the assigned locations provided in BulkEditAssignedLocationsRequest.created_assigned_locations.");
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
            let mcmd = SubCommand::with_name("bulk_edit").about("Bulk edits negative keywords in a single negative keyword list. The operation will delete the negative keywords provided in BulkEditNegativeKeywordsRequest.deleted_negative_keywords and then create the negative keywords provided in BulkEditNegativeKeywordsRequest.created_negative_keywords. This operation is guaranteed to be atomic and will never result in a partial success or partial failure.");
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
            let mcmd = SubCommand::with_name("create").about("Assigns a targeting option to an advertiser. Returns the assigned targeting option if successful.");
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
            let mcmd = SubCommand::with_name("bulk_edit").about("Bulk edits sites under a single channel. The operation will delete the sites provided in BulkEditSitesRequest.deleted_sites and then create the sites provided in BulkEditSitesRequest.created_sites.");
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
        let mut assigned_targeting_options2 = SubCommand::with_name("assigned_targeting_options")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Assigns a targeting option to a partner. Returns the assigned targeting option if successful.");
            assigned_targeting_options2 = assigned_targeting_options2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes an assigned targeting option from a partner.");
            assigned_targeting_options2 = assigned_targeting_options2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets a single targeting option assigned to a partner.");
            assigned_targeting_options2 = assigned_targeting_options2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the targeting options assigned to a partner.");
            assigned_targeting_options2 = assigned_targeting_options2.subcommand(mcmd);
        }
        let mut assigned_targeting_options3 = SubCommand::with_name("assigned_targeting_options")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets a single targeting option assigned to an insertion order.");
            assigned_targeting_options3 = assigned_targeting_options3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the targeting options assigned to an insertion order.");
            assigned_targeting_options3 = assigned_targeting_options3.subcommand(mcmd);
        }
        let mut assigned_targeting_options3 = SubCommand::with_name("assigned_targeting_options")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Assigns a targeting option to a line item. Returns the assigned targeting option if successful.");
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
        targeting_types2 = targeting_types2.subcommand(assigned_targeting_options3);
        targeting_types1 = targeting_types1.subcommand(assigned_targeting_options2);
        channels1 = channels1.subcommand(sites2);
        targeting_types1 = targeting_types1.subcommand(assigned_targeting_options2);
        negative_keyword_lists1 = negative_keyword_lists1.subcommand(negative_keywords2);
        location_lists1 = location_lists1.subcommand(assigned_locations2);
        line_items1 = line_items1.subcommand(targeting_types2);
        insertion_orders1 = insertion_orders1.subcommand(targeting_types2);
        channels1 = channels1.subcommand(sites2);
        targeting_types0 = targeting_types0.subcommand(targeting_options1);
        sdfdownloadtasks0 = sdfdownloadtasks0.subcommand(operations1);
        partners0 = partners0.subcommand(targeting_types1);
        partners0 = partners0.subcommand(channels1);
        inventory_source_groups0 = inventory_source_groups0.subcommand(assigned_inventory_sources1);
        advertisers0 = advertisers0.subcommand(targeting_types1);
        advertisers0 = advertisers0.subcommand(negative_keyword_lists1);
        advertisers0 = advertisers0.subcommand(manual_triggers1);
        advertisers0 = advertisers0.subcommand(location_lists1);
        advertisers0 = advertisers0.subcommand(line_items1);
        advertisers0 = advertisers0.subcommand(insertion_orders1);
        advertisers0 = advertisers0.subcommand(creatives1);
        advertisers0 = advertisers0.subcommand(channels1);
        advertisers0 = advertisers0.subcommand(campaigns1);
        advertisers0 = advertisers0.subcommand(assets1);
        app = app.subcommand(users0);
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
        app = app.subcommand(custom_bidding_algorithms0);
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
