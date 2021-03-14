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
        let mut app = App::new("blogger3")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210311")
            .about("The Blogger API provides access to posts, comments and pages of a Blogger blog.")
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
        let mut blog_user_infos0 = SubCommand::with_name("blog_user_infos")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets one blog and user info pair by blog id and user id.");
            blog_user_infos0 = blog_user_infos0.subcommand(mcmd);
        }
        let mut blogs0 = SubCommand::with_name("blogs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, get_by_url and list_by_user");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a blog by id.");
            blogs0 = blogs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_by_url").about("Gets a blog by url.");
            blogs0 = blogs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_by_user").about("Lists blogs by user.");
            blogs0 = blogs0.subcommand(mcmd);
        }
        let mut comments0 = SubCommand::with_name("comments")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: approve, delete, get, list, list_by_blog, mark_as_spam and remove_content");
        {
            let mcmd = SubCommand::with_name("approve")
                .about("Marks a comment as not spam by blog id, post id and comment id.");
            comments0 = comments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes a comment by blog id, post id and comment id.");
            comments0 = comments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a comment by id.");
            comments0 = comments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists comments.");
            comments0 = comments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_by_blog").about("Lists comments by blog.");
            comments0 = comments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("mark_as_spam")
                .about("Marks a comment as spam by blog id, post id and comment id.");
            comments0 = comments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove_content")
                .about("Removes the content of a comment by blog id, post id and comment id.");
            comments0 = comments0.subcommand(mcmd);
        }
        let mut page_views0 = SubCommand::with_name("page_views")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets page views by blog id.");
            page_views0 = page_views0.subcommand(mcmd);
        }
        let mut pages0 = SubCommand::with_name("pages")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch, publish, revert and update");
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes a page by blog id and page id.");
            pages0 = pages0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a page by blog id and page id.");
            pages0 = pages0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a page.");
            pages0 = pages0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists pages.");
            pages0 = pages0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Patches a page.");
            pages0 = pages0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("publish").about("Publishes a page.");
            pages0 = pages0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("revert")
                .about("Reverts a published or scheduled page to draft state.");
            pages0 = pages0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update").about("Updates a page by blog id and page id.");
            pages0 = pages0.subcommand(mcmd);
        }
        let mut post_user_infos0 = SubCommand::with_name("post_user_infos")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets one post and user info pair, by post_id and user_id.");
            post_user_infos0 = post_user_infos0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists post and user info pairs.");
            post_user_infos0 = post_user_infos0.subcommand(mcmd);
        }
        let mut posts0 = SubCommand::with_name("posts")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: delete, get, get_by_path, insert, list, patch, publish, revert, search and update");
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes a post by blog id and post id.");
            posts0 = posts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a post by blog id and post id");
            posts0 = posts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_by_path").about("Gets a post by path.");
            posts0 = posts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a post.");
            posts0 = posts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists posts.");
            posts0 = posts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Patches a post.");
            posts0 = posts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("publish").about("Publishes a post.");
            posts0 = posts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("revert")
                .about("Reverts a published or scheduled post to draft state.");
            posts0 = posts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search")
                .about("Searches for posts matching given query terms in the specified blog.");
            posts0 = posts0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update").about("Updates a post by blog id and post id.");
            posts0 = posts0.subcommand(mcmd);
        }
        let mut users0 = SubCommand::with_name("users")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one user by user_id.");
            users0 = users0.subcommand(mcmd);
        }
        app = app.subcommand(users0);
        app = app.subcommand(posts0);
        app = app.subcommand(post_user_infos0);
        app = app.subcommand(pages0);
        app = app.subcommand(page_views0);
        app = app.subcommand(comments0);
        app = app.subcommand(blogs0);
        app = app.subcommand(blog_user_infos0);

        Self { app }
    }
}
use google_blogger3 as api;

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
