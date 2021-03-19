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
        let mut app = App::new("gmail1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210315")
            .about("The Gmail API lets you view and manage Gmail mailbox data like threads, messages, and labels.")
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
            .about("methods: get_profile, stop and watch");
        {
            let mcmd = SubCommand::with_name("get_profile")
                .about("Gets the current user\'s Gmail profile.");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("stop")
                .about("Stop receiving push notifications for the given user mailbox.");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("watch")
                .about("Set up or update a push notification watch on the given user mailbox.");
            users0 = users0.subcommand(mcmd);
        }
        let mut drafts1 = SubCommand::with_name("drafts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, send and update");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new draft with the `DRAFT` label.");
            drafts1 = drafts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Immediately and permanently deletes the specified draft. Does not simply trash it.");
            drafts1 = drafts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified draft.");
            drafts1 = drafts1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists the drafts in the user\'s mailbox.");
            drafts1 = drafts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("send").about("Sends the specified, existing draft to the recipients in the `To`, `Cc`, and `Bcc` headers.");
            drafts1 = drafts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Replaces a draft\'s content.");
            drafts1 = drafts1.subcommand(mcmd);
        }
        let mut history1 = SubCommand::with_name("history")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists the history of all changes to the given mailbox. History results are returned in chronological order (increasing `historyId`).");
            history1 = history1.subcommand(mcmd);
        }
        let mut labels1 = SubCommand::with_name("labels")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new label.");
            labels1 = labels1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Immediately and permanently deletes the specified label and removes it from any messages and threads that it is applied to.");
            labels1 = labels1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified label.");
            labels1 = labels1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists all labels in the user\'s mailbox.");
            labels1 = labels1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Patch the specified label.");
            labels1 = labels1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the specified label.");
            labels1 = labels1.subcommand(mcmd);
        }
        let mut messages1 = SubCommand::with_name("messages")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: batch_delete, batch_modify, delete, get, import, insert, list, modify, send, trash and untrash");
        {
            let mcmd = SubCommand::with_name("batch_delete").about("Deletes many messages by message ID. Provides no guarantees that messages were not already deleted or even existed at all.");
            messages1 = messages1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_modify")
                .about("Modifies the labels on the specified messages.");
            messages1 = messages1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Immediately and permanently deletes the specified message. This operation cannot be undone. Prefer `messages.trash` instead.");
            messages1 = messages1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified message.");
            messages1 = messages1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import").about("Imports a message into only this user\'s mailbox, with standard email delivery scanning and classification similar to receiving via SMTP. Does not send a message. Note: This function doesn\'t trigger forwarding rules or filters set up by the user.");
            messages1 = messages1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Directly inserts a message into only this user\'s mailbox similar to `IMAP APPEND`, bypassing most scanning and classification. Does not send a message.");
            messages1 = messages1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists the messages in the user\'s mailbox.");
            messages1 = messages1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("modify")
                .about("Modifies the labels on the specified message.");
            messages1 = messages1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("send").about("Sends the specified message to the recipients in the `To`, `Cc`, and `Bcc` headers.");
            messages1 = messages1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("trash").about("Moves the specified message to the trash.");
            messages1 = messages1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("untrash")
                .about("Removes the specified message from the trash.");
            messages1 = messages1.subcommand(mcmd);
        }
        let mut settings1 = SubCommand::with_name("settings")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: get_auto_forwarding, get_imap, get_language, get_pop, get_vacation, update_auto_forwarding, update_imap, update_language, update_pop and update_vacation");
        {
            let mcmd = SubCommand::with_name("get_auto_forwarding")
                .about("Gets the auto-forwarding setting for the specified account.");
            settings1 = settings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_imap").about("Gets IMAP settings.");
            settings1 = settings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_language").about("Gets language settings.");
            settings1 = settings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_pop").about("Gets POP settings.");
            settings1 = settings1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get_vacation").about("Gets vacation responder settings.");
            settings1 = settings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_auto_forwarding").about("Updates the auto-forwarding setting for the specified account. A verified forwarding address must be specified when auto-forwarding is enabled. This method is only available to service account clients that have been delegated domain-wide authority.");
            settings1 = settings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_imap").about("Updates IMAP settings.");
            settings1 = settings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_language").about("Updates language settings. If successful, the return object contains the `displayLanguage` that was saved for the user, which may differ from the value passed into the request. This is because the requested `displayLanguage` may not be directly supported by Gmail but have a close variant that is, and so the variant may be chosen and saved instead.");
            settings1 = settings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_pop").about("Updates POP settings.");
            settings1 = settings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_vacation")
                .about("Updates vacation responder settings.");
            settings1 = settings1.subcommand(mcmd);
        }
        let mut threads1 = SubCommand::with_name("threads")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, list, modify, trash and untrash");
        {
            let mcmd = SubCommand::with_name("delete").about("Immediately and permanently deletes the specified thread. This operation cannot be undone. Prefer `threads.trash` instead.");
            threads1 = threads1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified thread.");
            threads1 = threads1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists the threads in the user\'s mailbox.");
            threads1 = threads1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("modify").about("Modifies the labels applied to the thread. This applies to all messages in the thread.");
            threads1 = threads1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("trash").about("Moves the specified thread to the trash.");
            threads1 = threads1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("untrash")
                .about("Removes the specified thread from the trash.");
            threads1 = threads1.subcommand(mcmd);
        }
        let mut attachments2 = SubCommand::with_name("attachments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified message attachment.");
            attachments2 = attachments2.subcommand(mcmd);
        }
        let mut delegates2 = SubCommand::with_name("delegates")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Adds a delegate with its verification status set directly to `accepted`, without sending any verification email. The delegate user must be a member of the same G Suite organization as the delegator user. Gmail imposes limitations on the number of delegates and delegators each user in a G Suite organization can have. These limits depend on your organization, but in general each user can have up to 25 delegates and up to 10 delegators. Note that a delegate user must be referred to by their primary email address, and not an email alias. Also note that when a new delegate is created, there may be up to a one minute delay before the new delegate is available for use. This method is only available to service account clients that have been delegated domain-wide authority.");
            delegates2 = delegates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Removes the specified delegate (which can be of any verification status), and revokes any verification that may have been required for using it. Note that a delegate user must be referred to by their primary email address, and not an email alias. This method is only available to service account clients that have been delegated domain-wide authority.");
            delegates2 = delegates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified delegate. Note that a delegate user must be referred to by their primary email address, and not an email alias. This method is only available to service account clients that have been delegated domain-wide authority.");
            delegates2 = delegates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the delegates for the specified account. This method is only available to service account clients that have been delegated domain-wide authority.");
            delegates2 = delegates2.subcommand(mcmd);
        }
        let mut filters2 = SubCommand::with_name("filters")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a filter. Note: you can only create a maximum of 1,000 filters.");
            filters2 = filters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a filter.");
            filters2 = filters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a filter.");
            filters2 = filters2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists the message filters of a Gmail user.");
            filters2 = filters2.subcommand(mcmd);
        }
        let mut forwarding_addresses2 = SubCommand::with_name("forwarding_addresses")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a forwarding address. If ownership verification is required, a message will be sent to the recipient and the resource\'s verification status will be set to `pending`; otherwise, the resource will be created with verification status set to `accepted`. This method is only available to service account clients that have been delegated domain-wide authority.");
            forwarding_addresses2 = forwarding_addresses2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified forwarding address and revokes any verification that may have been required. This method is only available to service account clients that have been delegated domain-wide authority.");
            forwarding_addresses2 = forwarding_addresses2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified forwarding address.");
            forwarding_addresses2 = forwarding_addresses2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the forwarding addresses for the specified account.");
            forwarding_addresses2 = forwarding_addresses2.subcommand(mcmd);
        }
        let mut send_as2 = SubCommand::with_name("send_as")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch, update and verify");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a custom \"from\" send-as alias. If an SMTP MSA is specified, Gmail will attempt to connect to the SMTP service to validate the configuration before creating the alias. If ownership verification is required for the alias, a message will be sent to the email address and the resource\'s verification status will be set to `pending`; otherwise, the resource will be created with verification status set to `accepted`. If a signature is provided, Gmail will sanitize the HTML before saving it with the alias. This method is only available to service account clients that have been delegated domain-wide authority.");
            send_as2 = send_as2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified send-as alias. Revokes any verification that may have been required for using it. This method is only available to service account clients that have been delegated domain-wide authority.");
            send_as2 = send_as2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified send-as alias. Fails with an HTTP 404 error if the specified address is not a member of the collection.");
            send_as2 = send_as2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the send-as aliases for the specified account. The result includes the primary send-as address associated with the account as well as any custom \"from\" aliases.");
            send_as2 = send_as2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Patch the specified send-as alias.");
            send_as2 = send_as2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a send-as alias. If a signature is provided, Gmail will sanitize the HTML before saving it with the alias. Addresses other than the primary address for the account can only be updated by service account clients that have been delegated domain-wide authority.");
            send_as2 = send_as2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("verify").about("Sends a verification email to the specified send-as alias address. The verification status must be `pending`. This method is only available to service account clients that have been delegated domain-wide authority.");
            send_as2 = send_as2.subcommand(mcmd);
        }
        let mut smime_info3 = SubCommand::with_name("smime_info")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list and set_default");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified S/MIME config for the specified send-as alias.");
            smime_info3 = smime_info3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the specified S/MIME config for the specified send-as alias.");
            smime_info3 = smime_info3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Insert (upload) the given S/MIME config for the specified send-as alias. Note that pkcs12 format is required for the key.");
            smime_info3 = smime_info3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists S/MIME configs for the specified send-as alias.");
            smime_info3 = smime_info3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_default")
                .about("Sets the default S/MIME config for the specified send-as alias.");
            smime_info3 = smime_info3.subcommand(mcmd);
        }
        send_as2 = send_as2.subcommand(smime_info3);
        settings1 = settings1.subcommand(send_as2);
        settings1 = settings1.subcommand(forwarding_addresses2);
        settings1 = settings1.subcommand(filters2);
        settings1 = settings1.subcommand(delegates2);
        messages1 = messages1.subcommand(attachments2);
        users0 = users0.subcommand(threads1);
        users0 = users0.subcommand(settings1);
        users0 = users0.subcommand(messages1);
        users0 = users0.subcommand(labels1);
        users0 = users0.subcommand(history1);
        users0 = users0.subcommand(drafts1);
        app = app.subcommand(users0);

        Self { app }
    }
}
use google_gmail1 as api;

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
