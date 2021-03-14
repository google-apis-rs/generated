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
        let mut app = App::new("content2d1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210303")
            .about("Manage your product listings and accounts for Google Shopping")
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
        let mut accounts0 = SubCommand::with_name("accounts")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: authinfo, claimwebsite, custombatch, delete, get, insert, link, list, listlinks, update and updatelabels");
        {
            let mcmd = SubCommand::with_name("authinfo")
                .about("Returns information about the authenticated user.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("claimwebsite")
                .about("Claims the website of a Merchant Center sub-account.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("custombatch").about("Retrieves, inserts, updates, and deletes multiple Merchant Center (sub-)accounts in a single request.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes a Merchant Center sub-account.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a Merchant Center account.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("insert").about("Creates a Merchant Center sub-account.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("link").about("Performs an action on a link between two Merchant Center accounts, namely accountId and linkedAccountId.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the sub-accounts in your Merchant Center account.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("listlinks")
                .about("Returns the list of accounts linked to your Merchant Center account.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a Merchant Center account. Any fields that are not provided are deleted from the resource.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("updatelabels").about(
                "Updates labels that are assigned to the Merchant Center account by CSS user.",
            );
            accounts0 = accounts0.subcommand(mcmd);
        }
        let mut accountstatuses0 = SubCommand::with_name("accountstatuses")
            .setting(AppSettings::ColoredHelp)
            .about("methods: custombatch, get and list");
        {
            let mcmd = SubCommand::with_name("custombatch")
                .about("Retrieves multiple Merchant Center account statuses in a single request.");
            accountstatuses0 = accountstatuses0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the status of a Merchant Center account. No itemLevelIssues are returned for multi-client accounts.");
            accountstatuses0 = accountstatuses0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the statuses of the sub-accounts in your Merchant Center account.");
            accountstatuses0 = accountstatuses0.subcommand(mcmd);
        }
        let mut accounttax0 = SubCommand::with_name("accounttax")
            .setting(AppSettings::ColoredHelp)
            .about("methods: custombatch, get, list and update");
        {
            let mcmd = SubCommand::with_name("custombatch").about(
                "Retrieves and updates tax settings of multiple accounts in a single request.",
            );
            accounttax0 = accounttax0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Retrieves the tax settings of the account.");
            accounttax0 = accounttax0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Lists the tax settings of the sub-accounts in your Merchant Center account.",
            );
            accounttax0 = accounttax0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the tax settings of the account. Any fields that are not provided are deleted from the resource.");
            accounttax0 = accounttax0.subcommand(mcmd);
        }
        let mut buyongoogleprograms0 = SubCommand::with_name("buyongoogleprograms")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and onboard");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves a status of BoG program for your Merchant Center account.");
            buyongoogleprograms0 = buyongoogleprograms0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("onboard").about("Onboards BoG in your Merchant Center account. By using this method, you agree to the [Terms of Service](https://merchants.google.com/mc/termsofservice/transactions/US/latest). Calling this method is only possible if the authenticated account is the same as the merchant id in the request. Calling this method multiple times will only accept Terms of Service if the latest version is not currently signed.");
            buyongoogleprograms0 = buyongoogleprograms0.subcommand(mcmd);
        }
        let mut collections0 = SubCommand::with_name("collections")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Uploads a collection to your Merchant Center account. If a collection with the same collectionId already exists, this method updates that entry. In each update, the collection is completely replaced by the fields in the body of the update request.");
            collections0 = collections0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes a collection from your Merchant Center account.");
            collections0 = collections0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves a collection from your Merchant Center account.");
            collections0 = collections0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the collections in your Merchant Center account. The response might contain fewer items than specified by page_size. Rely on next_page_token to determine if there are more items to be requested.");
            collections0 = collections0.subcommand(mcmd);
        }
        let mut collectionstatuses0 = SubCommand::with_name("collectionstatuses")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the status of a collection from your Merchant Center account.");
            collectionstatuses0 = collectionstatuses0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the statuses of the collections in your Merchant Center account.");
            collectionstatuses0 = collectionstatuses0.subcommand(mcmd);
        }
        let mut csses0 = SubCommand::with_name("csses")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list and updatelabels");
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a single CSS domain by ID.");
            csses0 = csses0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists CSS domains affiliated with a CSS group.");
            csses0 = csses0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("updatelabels")
                .about("Updates labels that are assigned to a CSS domain by its CSS group.");
            csses0 = csses0.subcommand(mcmd);
        }
        let mut datafeeds0 = SubCommand::with_name("datafeeds")
            .setting(AppSettings::ColoredHelp)
            .about("methods: custombatch, delete, fetchnow, get, insert, list and update");
        {
            let mcmd = SubCommand::with_name("custombatch").about("Deletes, fetches, gets, inserts and updates multiple datafeeds in a single request.");
            datafeeds0 = datafeeds0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes a datafeed configuration from your Merchant Center account.");
            datafeeds0 = datafeeds0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("fetchnow").about("Invokes a fetch for the datafeed in your Merchant Center account. If you need to call this method more than once per day, we recommend you use the Products service to update your product data.");
            datafeeds0 = datafeeds0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves a datafeed configuration from your Merchant Center account.");
            datafeeds0 = datafeeds0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Registers a datafeed configuration with your Merchant Center account.");
            datafeeds0 = datafeeds0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the configurations for datafeeds in your Merchant Center account.");
            datafeeds0 = datafeeds0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a datafeed configuration of your Merchant Center account. Any fields that are not provided are deleted from the resource.");
            datafeeds0 = datafeeds0.subcommand(mcmd);
        }
        let mut datafeedstatuses0 = SubCommand::with_name("datafeedstatuses")
            .setting(AppSettings::ColoredHelp)
            .about("methods: custombatch, get and list");
        {
            let mcmd = SubCommand::with_name("custombatch")
                .about("Gets multiple Merchant Center datafeed statuses in a single request.");
            datafeedstatuses0 = datafeedstatuses0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves the status of a datafeed from your Merchant Center account.");
            datafeedstatuses0 = datafeedstatuses0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the statuses of the datafeeds in your Merchant Center account.");
            datafeedstatuses0 = datafeedstatuses0.subcommand(mcmd);
        }
        let mut liasettings0 = SubCommand::with_name("liasettings")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: custombatch, get, getaccessiblegmbaccounts, list, listposdataproviders, requestgmbaccess, requestinventoryverification, setinventoryverificationcontact, setposdataprovider and update");
        {
            let mcmd = SubCommand::with_name("custombatch").about("Retrieves and/or updates the LIA settings of multiple accounts in a single request.");
            liasettings0 = liasettings0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Retrieves the LIA settings of the account.");
            liasettings0 = liasettings0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("getaccessiblegmbaccounts")
                .about("Retrieves the list of accessible Google My Business accounts.");
            liasettings0 = liasettings0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Lists the LIA settings of the sub-accounts in your Merchant Center account.",
            );
            liasettings0 = liasettings0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("listposdataproviders").about("Retrieves the list of POS data providers that have active settings for the all eiligible countries.");
            liasettings0 = liasettings0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("requestgmbaccess")
                .about("Requests access to a specified Google My Business account.");
            liasettings0 = liasettings0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("requestinventoryverification")
                .about("Requests inventory validation for the specified country.");
            liasettings0 = liasettings0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("setinventoryverificationcontact")
                .about("Sets the inventory verification contract for the specified country.");
            liasettings0 = liasettings0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("setposdataprovider")
                .about("Sets the POS data provider for the specified country.");
            liasettings0 = liasettings0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the LIA settings of the account. Any fields that are not provided are deleted from the resource.");
            liasettings0 = liasettings0.subcommand(mcmd);
        }
        let mut localinventory0 = SubCommand::with_name("localinventory")
            .setting(AppSettings::ColoredHelp)
            .about("methods: custombatch and insert");
        {
            let mcmd = SubCommand::with_name("custombatch").about(
                "Updates local inventory for multiple products or stores in a single request.",
            );
            localinventory0 = localinventory0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Updates the local inventory of a product in your Merchant Center account.");
            localinventory0 = localinventory0.subcommand(mcmd);
        }
        let mut orderinvoices0 = SubCommand::with_name("orderinvoices")
            .setting(AppSettings::ColoredHelp)
            .about("methods: createchargeinvoice and createrefundinvoice");
        {
            let mcmd = SubCommand::with_name("createchargeinvoice").about("Creates a charge invoice for a shipment group, and triggers a charge capture for orderinvoice enabled orders.");
            orderinvoices0 = orderinvoices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("createrefundinvoice").about("Creates a refund invoice for one or more shipment groups, and triggers a refund for orderinvoice enabled orders. This can only be used for line items that have previously been charged using `createChargeInvoice`. All amounts (except for the summary) are incremental with respect to the previous invoice.");
            orderinvoices0 = orderinvoices0.subcommand(mcmd);
        }
        let mut orderreports0 = SubCommand::with_name("orderreports")
            .setting(AppSettings::ColoredHelp)
            .about("methods: listdisbursements and listtransactions");
        {
            let mcmd = SubCommand::with_name("listdisbursements")
                .about("Retrieves a report for disbursements from your Merchant Center account.");
            orderreports0 = orderreports0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("listtransactions").about("Retrieves a list of transactions for a disbursement from your Merchant Center account.");
            orderreports0 = orderreports0.subcommand(mcmd);
        }
        let mut orderreturns0 = SubCommand::with_name("orderreturns")
            .setting(AppSettings::ColoredHelp)
            .about("methods: acknowledge, createorderreturn, get, list and process");
        {
            let mcmd = SubCommand::with_name("acknowledge")
                .about("Acks an order return in your Merchant Center account.");
            orderreturns0 = orderreturns0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("createorderreturn")
                .about("Create return in your Merchant Center account.");
            orderreturns0 = orderreturns0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves an order return from your Merchant Center account.");
            orderreturns0 = orderreturns0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists order returns in your Merchant Center account.");
            orderreturns0 = orderreturns0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("process")
                .about("Processes return in your Merchant Center account.");
            orderreturns0 = orderreturns0.subcommand(mcmd);
        }
        let mut orders0 = SubCommand::with_name("orders")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: acknowledge, advancetestorder, cancel, cancellineitem, canceltestorderbycustomer, createtestorder, createtestreturn, get, getbymerchantorderid, gettestordertemplate, instorerefundlineitem, list, refunditem, refundorder, rejectreturnlineitem, returnrefundlineitem, setlineitemmetadata, shiplineitems, updatelineitemshippingdetails, updatemerchantorderid and updateshipment");
        {
            let mcmd =
                SubCommand::with_name("acknowledge").about("Marks an order as acknowledged.");
            orders0 = orders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("advancetestorder").about("Sandbox only. Moves a test order from state \"`inProgress`\" to state \"`pendingShipment`\".");
            orders0 = orders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("cancel")
                .about("Cancels all line items in an order, making a full refund.");
            orders0 = orders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("cancellineitem")
                .about("Cancels a line item, making a full refund.");
            orders0 = orders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("canceltestorderbycustomer")
                .about("Sandbox only. Cancels a test order for customer-initiated cancellation.");
            orders0 = orders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("createtestorder")
                .about("Sandbox only. Creates a test order.");
            orders0 = orders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("createtestreturn")
                .about("Sandbox only. Creates a test return.");
            orders0 = orders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves an order from your Merchant Center account.");
            orders0 = orders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("getbymerchantorderid")
                .about("Retrieves an order using merchant order ID.");
            orders0 = orders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("gettestordertemplate").about("Sandbox only. Retrieves an order template that can be used to quickly create a new order in sandbox.");
            orders0 = orders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("instorerefundlineitem").about("Deprecated. Notifies that item return and refund was handled directly by merchant outside of Google payments processing (e.g. cash refund done in store). Note: We recommend calling the returnrefundlineitem method to refund in-store returns. We will issue the refund directly to the customer. This helps to prevent possible differences arising between merchant and Google transaction records. We also recommend having the point of sale system communicate with Google to ensure that customers do not receive a double refund by first refunding via Google then via an in-store return.");
            orders0 = orders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the orders in your Merchant Center account.");
            orders0 = orders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("refunditem")
                .about("Issues a partial or total refund for items and shipment.");
            orders0 = orders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("refundorder")
                .about("Issues a partial or total refund for an order.");
            orders0 = orders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("rejectreturnlineitem")
                .about("Rejects return on an line item.");
            orders0 = orders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("returnrefundlineitem").about("Returns and refunds a line item. Note that this method can only be called on fully shipped orders. Please also note that the Orderreturns API is the preferred way to handle returns after you receive a return from a customer. You can use Orderreturns.list or Orderreturns.get to search for the return, and then use Orderreturns.processreturn to issue the refund. If the return cannot be found, then we recommend using this API to issue a refund.");
            orders0 = orders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("setlineitemmetadata").about("Sets (or overrides if it already exists) merchant provided annotations in the form of key-value pairs. A common use case would be to supply us with additional structured information about a line item that cannot be provided via other methods. Submitted key-value pairs can be retrieved as part of the orders resource.");
            orders0 = orders0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("shiplineitems").about("Marks line item(s) as shipped.");
            orders0 = orders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("updatelineitemshippingdetails")
                .about("Updates ship by and delivery by dates for a line item.");
            orders0 = orders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("updatemerchantorderid")
                .about("Updates the merchant order ID for a given order.");
            orders0 = orders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("updateshipment")
                .about("Updates a shipment\'s status, carrier, and/or tracking ID.");
            orders0 = orders0.subcommand(mcmd);
        }
        let mut ordertrackingsignals0 = SubCommand::with_name("ordertrackingsignals")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create");
        {
            let mcmd = SubCommand::with_name("create").about("Creates new order tracking signal.");
            ordertrackingsignals0 = ordertrackingsignals0.subcommand(mcmd);
        }
        let mut pos0 = SubCommand::with_name("pos")
            .setting(AppSettings::ColoredHelp)
            .about("methods: custombatch, delete, get, insert, inventory, list and sale");
        {
            let mcmd = SubCommand::with_name("custombatch")
                .about("Batches multiple POS-related calls in a single request.");
            pos0 = pos0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes a store for the given merchant.");
            pos0 = pos0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Retrieves information about the given store.");
            pos0 = pos0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("insert").about("Creates a store for the given merchant.");
            pos0 = pos0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("inventory")
                .about("Submit inventory for the given merchant.");
            pos0 = pos0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists the stores of the target merchant.");
            pos0 = pos0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("sale").about("Submit a sale event for the given merchant.");
            pos0 = pos0.subcommand(mcmd);
        }
        let mut products0 = SubCommand::with_name("products")
            .setting(AppSettings::ColoredHelp)
            .about("methods: custombatch, delete, get, insert and list");
        {
            let mcmd = SubCommand::with_name("custombatch")
                .about("Retrieves, inserts, and deletes multiple products in a single request.");
            products0 = products0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes a product from your Merchant Center account.");
            products0 = products0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves a product from your Merchant Center account.");
            products0 = products0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Uploads a product to your Merchant Center account. If an item with the same channel, contentLanguage, offerId, and targetCountry already exists, this method updates that entry.");
            products0 = products0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the products in your Merchant Center account. The response might contain fewer items than specified by maxResults. Rely on nextPageToken to determine if there are more items to be requested.");
            products0 = products0.subcommand(mcmd);
        }
        let mut productstatuses0 = SubCommand::with_name("productstatuses")
            .setting(AppSettings::ColoredHelp)
            .about("methods: custombatch, get and list");
        {
            let mcmd = SubCommand::with_name("custombatch")
                .about("Gets the statuses of multiple products in a single request.");
            productstatuses0 = productstatuses0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the status of a product from your Merchant Center account.");
            productstatuses0 = productstatuses0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the statuses of the products in your Merchant Center account.");
            productstatuses0 = productstatuses0.subcommand(mcmd);
        }
        let mut pubsubnotificationsettings0 = SubCommand::with_name("pubsubnotificationsettings")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and update");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves a Merchant Center account\'s pubsub notification settings.");
            pubsubnotificationsettings0 = pubsubnotificationsettings0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Register a Merchant Center account for pubsub notifications. Note that cloud topic name should not be provided as part of the request.");
            pubsubnotificationsettings0 = pubsubnotificationsettings0.subcommand(mcmd);
        }
        let mut regionalinventory0 = SubCommand::with_name("regionalinventory")
            .setting(AppSettings::ColoredHelp)
            .about("methods: custombatch and insert");
        {
            let mcmd = SubCommand::with_name("custombatch").about(
                "Updates regional inventory for multiple products or regions in a single request.",
            );
            regionalinventory0 = regionalinventory0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Update the regional inventory of a product in your Merchant Center account. If a regional inventory with the same region ID already exists, this method updates that entry.");
            regionalinventory0 = regionalinventory0.subcommand(mcmd);
        }
        let mut regions0 = SubCommand::with_name("regions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a region definition in your Merchant Center account.");
            regions0 = regions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes a region definition from your Merchant Center account.");
            regions0 = regions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves a region defined in your Merchant Center account.");
            regions0 = regions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the regions in your Merchant Center account.");
            regions0 = regions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates a region definition in your Merchant Center account.");
            regions0 = regions0.subcommand(mcmd);
        }
        let mut reports0 = SubCommand::with_name("reports")
            .setting(AppSettings::ColoredHelp)
            .about("methods: search");
        {
            let mcmd = SubCommand::with_name("search").about("Retrieves merchant performance mertrics matching the search query and optionally segmented by selected dimensions.");
            reports0 = reports0.subcommand(mcmd);
        }
        let mut repricingrules0 = SubCommand::with_name("repricingrules")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a repricing rule for your Merchant Center account.");
            repricingrules0 = repricingrules0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes a repricing rule in your Merchant Center account.");
            repricingrules0 = repricingrules0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves a repricing rule from your Merchant Center account.");
            repricingrules0 = repricingrules0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the repricing rules in your Merchant Center account.");
            repricingrules0 = repricingrules0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a repricing rule in your Merchant Center account. All mutable fields will be overwritten in each update request. In each update, you must provide all required mutable fields, or an error will be thrown. If you do not provide an optional field in the update request, if that field currently exists, it will be deleted from the rule.");
            repricingrules0 = repricingrules0.subcommand(mcmd);
        }
        let mut returnaddress0 = SubCommand::with_name("returnaddress")
            .setting(AppSettings::ColoredHelp)
            .about("methods: custombatch, delete, get, insert and list");
        {
            let mcmd = SubCommand::with_name("custombatch")
                .about("Batches multiple return address related calls in a single request.");
            returnaddress0 = returnaddress0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes a return address for the given Merchant Center account.");
            returnaddress0 = returnaddress0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets a return address of the Merchant Center account.");
            returnaddress0 = returnaddress0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Inserts a return address for the Merchant Center account.");
            returnaddress0 = returnaddress0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the return addresses of the Merchant Center account.");
            returnaddress0 = returnaddress0.subcommand(mcmd);
        }
        let mut returnpolicy0 = SubCommand::with_name("returnpolicy")
            .setting(AppSettings::ColoredHelp)
            .about("methods: custombatch, delete, get, insert and list");
        {
            let mcmd = SubCommand::with_name("custombatch")
                .about("Batches multiple return policy related calls in a single request.");
            returnpolicy0 = returnpolicy0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes a return policy for the given Merchant Center account.");
            returnpolicy0 = returnpolicy0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets a return policy of the Merchant Center account.");
            returnpolicy0 = returnpolicy0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Inserts a return policy for the Merchant Center account.");
            returnpolicy0 = returnpolicy0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the return policies of the Merchant Center account.");
            returnpolicy0 = returnpolicy0.subcommand(mcmd);
        }
        let mut returnpolicyonline0 = SubCommand::with_name("returnpolicyonline")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new return policy.");
            returnpolicyonline0 = returnpolicyonline0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an existing return policy.");
            returnpolicyonline0 = returnpolicyonline0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an existing return policy.");
            returnpolicyonline0 = returnpolicyonline0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all existing return policies.");
            returnpolicyonline0 = returnpolicyonline0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing return policy.");
            returnpolicyonline0 = returnpolicyonline0.subcommand(mcmd);
        }
        let mut settlementreports0 = SubCommand::with_name("settlementreports")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves a settlement report from your Merchant Center account.");
            settlementreports0 = settlementreports0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of settlement reports from your Merchant Center account.");
            settlementreports0 = settlementreports0.subcommand(mcmd);
        }
        let mut settlementtransactions0 = SubCommand::with_name("settlementtransactions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of transactions for the settlement.");
            settlementtransactions0 = settlementtransactions0.subcommand(mcmd);
        }
        let mut shippingsettings0 = SubCommand::with_name("shippingsettings")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: custombatch, get, getsupportedcarriers, getsupportedholidays, getsupportedpickupservices, list and update");
        {
            let mcmd = SubCommand::with_name("custombatch").about("Retrieves and updates the shipping settings of multiple accounts in a single request.");
            shippingsettings0 = shippingsettings0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves the shipping settings of the account.");
            shippingsettings0 = shippingsettings0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("getsupportedcarriers")
                .about("Retrieves supported carriers and carrier services for an account.");
            shippingsettings0 = shippingsettings0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("getsupportedholidays")
                .about("Retrieves supported holidays for an account.");
            shippingsettings0 = shippingsettings0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("getsupportedpickupservices")
                .about("Retrieves supported pickup services for an account.");
            shippingsettings0 = shippingsettings0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Lists the shipping settings of the sub-accounts in your Merchant Center account.",
            );
            shippingsettings0 = shippingsettings0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the shipping settings of the account. Any fields that are not provided are deleted from the resource.");
            shippingsettings0 = shippingsettings0.subcommand(mcmd);
        }
        let mut credentials1 = SubCommand::with_name("credentials")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create");
        {
            let mcmd = SubCommand::with_name("create").about("Uploads credentials for the Merchant Center account. If credentials already exist for this Merchant Center account and purpose, this method updates them.");
            credentials1 = credentials1.subcommand(mcmd);
        }
        let mut labels1 = SubCommand::with_name("labels")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, list and patch");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new label, not assigned to any account.");
            labels1 = labels1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about(
                "Deletes a label and removes it from all accounts to which it was assigned.",
            );
            labels1 = labels1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists the labels assigned to an account.");
            labels1 = labels1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a label.");
            labels1 = labels1.subcommand(mcmd);
        }
        let mut returncarrier1 = SubCommand::with_name("returncarrier")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, list and patch");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Links return carrier to a merchant account.");
            returncarrier1 = returncarrier1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Delete a return carrier in the merchant account.");
            returncarrier1 = returncarrier1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists available return carriers in the merchant account.");
            returncarrier1 = returncarrier1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates a return carrier in the merchant account.");
            returncarrier1 = returncarrier1.subcommand(mcmd);
        }
        let mut repricingreports1 = SubCommand::with_name("repricingreports")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the metrics report for a given Repricing product.");
            repricingreports1 = repricingreports1.subcommand(mcmd);
        }
        let mut repricingreports1 = SubCommand::with_name("repricingreports")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the metrics report for a given Repricing rule.");
            repricingreports1 = repricingreports1.subcommand(mcmd);
        }
        repricingrules0 = repricingrules0.subcommand(repricingreports1);
        productstatuses0 = productstatuses0.subcommand(repricingreports1);
        accounts0 = accounts0.subcommand(returncarrier1);
        accounts0 = accounts0.subcommand(labels1);
        accounts0 = accounts0.subcommand(credentials1);
        app = app.subcommand(shippingsettings0);
        app = app.subcommand(settlementtransactions0);
        app = app.subcommand(settlementreports0);
        app = app.subcommand(returnpolicyonline0);
        app = app.subcommand(returnpolicy0);
        app = app.subcommand(returnaddress0);
        app = app.subcommand(repricingrules0);
        app = app.subcommand(reports0);
        app = app.subcommand(regions0);
        app = app.subcommand(regionalinventory0);
        app = app.subcommand(pubsubnotificationsettings0);
        app = app.subcommand(productstatuses0);
        app = app.subcommand(products0);
        app = app.subcommand(pos0);
        app = app.subcommand(ordertrackingsignals0);
        app = app.subcommand(orders0);
        app = app.subcommand(orderreturns0);
        app = app.subcommand(orderreports0);
        app = app.subcommand(orderinvoices0);
        app = app.subcommand(localinventory0);
        app = app.subcommand(liasettings0);
        app = app.subcommand(datafeedstatuses0);
        app = app.subcommand(datafeeds0);
        app = app.subcommand(csses0);
        app = app.subcommand(collectionstatuses0);
        app = app.subcommand(collections0);
        app = app.subcommand(buyongoogleprograms0);
        app = app.subcommand(accounttax0);
        app = app.subcommand(accountstatuses0);
        app = app.subcommand(accounts0);

        Self { app }
    }
}
use google_content2d1 as api;

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
