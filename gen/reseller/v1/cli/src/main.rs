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
        let mut app = App::new("reseller1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210315")
            .about("Perform common functions that are available on the Channel Services console at scale, like placing orders and viewing customer information")
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
        let mut customers0 = SubCommand::with_name("customers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, insert, patch and update");
        {
            let mcmd = SubCommand::with_name("get").about("Get a customer account. Use this operation to see a customer account already in your reseller management, or to see the minimal account information for an existing customer that you do not manage. For more information about the API response for existing customers, see [retrieving a customer account](/admin-sdk/reseller/v1/how-tos/manage_customers#get_customer).");
            customers0 = customers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Order a new customer\'s account. Before ordering a new customer account, establish whether the customer account already exists using the [`customers.get`](/admin-sdk/reseller/v1/reference/customers/get) If the customer account exists as a direct Google account or as a resold customer account from another reseller, use the `customerAuthToken\\` as described in [order a resold account for an existing customer](/admin-sdk/reseller/v1/how-tos/manage_customers#create_existing_customer). For more information about ordering a new customer account, see [order a new customer account](/admin-sdk/reseller/v1/how-tos/manage_customers#create_customer). After creating a new customer account, you must provision a user as an administrator. The customer\'s administrator is required to sign in to the Admin console and sign the G Suite via Reseller agreement to activate the account. Resellers are prohibited from signing the G Suite via Reseller agreement on the customer\'s behalf. For more information, see [order a new customer account](/admin-sdk/reseller/v1/how-tos/manage_customers#tos).");
            customers0 = customers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Update a customer account\'s settings. This method supports patch semantics.",
            );
            customers0 = customers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Update a customer account\'s settings. For more information, see [update a customer\'s settings](/admin-sdk/reseller/v1/how-tos/manage_customers#update_customer).");
            customers0 = customers0.subcommand(mcmd);
        }
        let mut resellernotify0 = SubCommand::with_name("resellernotify")
            .setting(AppSettings::ColoredHelp)
            .about("methods: getwatchdetails, register and unregister");
        {
            let mcmd = SubCommand::with_name("getwatchdetails")
                .about("Returns all the details of the watch corresponding to the reseller.");
            resellernotify0 = resellernotify0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("register")
                .about("Registers a Reseller for receiving notifications.");
            resellernotify0 = resellernotify0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("unregister")
                .about("Unregisters a Reseller for receiving notifications.");
            resellernotify0 = resellernotify0.subcommand(mcmd);
        }
        let mut subscriptions0 = SubCommand::with_name("subscriptions")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: activate, change_plan, change_renewal_settings, change_seats, delete, get, insert, list, start_paid_service and suspend");
        {
            let mcmd = SubCommand::with_name("activate").about("Activates a subscription previously suspended by the reseller. If you did not suspend the customer subscription and it is suspended for any other reason, such as for abuse or a pending ToS acceptance, this call will not reactivate the customer subscription.");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("change_plan").about("Update a subscription plan. Use this method to update a plan for a 30-day trial or a flexible plan subscription to an annual commitment plan with monthly or yearly payments. How a plan is updated differs depending on the plan and the products. For more information, see the description in [manage subscriptions](/admin-sdk/reseller/v1/how-tos/manage_subscriptions#update_subscription_plan).");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("change_renewal_settings").about("Update a user license\'s renewal settings. This is applicable for accounts with annual commitment plans only. For more information, see the description in [manage subscriptions](/admin-sdk/reseller/v1/how-tos/manage_subscriptions#update_renewal).");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("change_seats").about("Update a subscription\'s user license settings. For more information about updating an annual commitment plan or a flexible plan subscription’s licenses, see [Manage Subscriptions](/admin-sdk/reseller/v1/how-tos/manage_subscriptions#update_subscription_seat).");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Cancel, suspend, or transfer a subscription to direct.");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get a specific subscription. The `subscriptionId` can be found using the [Retrieve all reseller subscriptions](/admin-sdk/reseller/v1/how-tos/manage_subscriptions#get_all_subscriptions) method. For more information about retrieving a specific subscription, see the information descrived in [manage subscriptions](/admin-sdk/reseller/v1/how-tos/manage_subscriptions#get_subscription).");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Create or transfer a subscription. Create a subscription for a customer\'s account that you ordered using the [Order a new customer account](/admin-sdk/reseller/v1/reference/customers/insert.html) method. For more information about creating a subscription for different payment plans, see [manage subscriptions](/admin-sdk/reseller/v1/how-tos/manage_subscriptions#create_subscription).\\ If you did not order the customer\'s account using the customer insert method, use the customer\'s `customerAuthToken` when creating a subscription for that customer. If transferring a G Suite subscription with an associated Google Drive or Google Vault subscription, use the [batch operation](/admin-sdk/reseller/v1/how-tos/batch.html) to transfer all of these subscriptions. For more information, see how to [transfer subscriptions](/admin-sdk/reseller/v1/how-tos/manage_subscriptions#transfer_a_subscription).");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List of subscriptions managed by the reseller. The list can be all subscriptions, all of a customer\'s subscriptions, or all of a customer\'s transferable subscriptions. Optionally, this method can filter the response by a `customerNamePrefix`. For more information, see [manage subscriptions](/admin-sdk/reseller/v1/how-tos/manage_subscriptions).");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("start_paid_service").about("Immediately move a 30-day free trial subscription to a paid service subscription. This method is only applicable if a payment plan has already been set up for the 30-day trial subscription. For more information, see [manage subscriptions](/admin-sdk/reseller/v1/how-tos/manage_subscriptions#paid_service).");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("suspend").about("Suspends an active subscription. You can use this method to suspend a paid subscription that is currently in the `ACTIVE` state. * For `FLEXIBLE` subscriptions, billing is paused. * For `ANNUAL_MONTHLY_PAY` or `ANNUAL_YEARLY_PAY` subscriptions: * Suspending the subscription does not change the renewal date that was originally committed to. * A suspended subscription does not renew. If you activate the subscription after the original renewal date, a new annual subscription will be created, starting on the day of activation. We strongly encourage you to suspend subscriptions only for short periods of time as suspensions over 60 days may result in the subscription being cancelled.");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        app = app.subcommand(subscriptions0);
        app = app.subcommand(resellernotify0);
        app = app.subcommand(customers0);

        Self { app }
    }
}
use google_reseller1 as api;

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
