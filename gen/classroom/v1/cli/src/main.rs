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
        let mut app = App::new("classroom1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200511")
            .about("Manages classes, rosters, and invitations in Google Classroom.")
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
        let mut courses0 = SubCommand::with_name("courses")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a course.\n\nThe user specified in `ownerId` is the owner of the created course\nand added as a teacher.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to create\ncourses or for access errors.\n* `NOT_FOUND` if the primary teacher is not a valid user.\n* `FAILED_PRECONDITION` if the course owner\'s account is disabled or for\nthe following request errors:\n    * UserGroupsMembershipLimitReached\n* `ALREADY_EXISTS` if an alias was specified in the `id` and\nalready exists.");
            courses0 = courses0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a course.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to delete the\nrequested course or for access errors.\n* `NOT_FOUND` if no course exists with the requested ID.");
            courses0 = courses0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns a course.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to access the\nrequested course or for access errors.\n* `NOT_FOUND` if no course exists with the requested ID.");
            courses0 = courses0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns a list of courses that the requesting user is permitted to view,\nrestricted to those that match the request. Returned courses are ordered by\ncreation time, with the most recently created coming first.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` for access errors.\n* `INVALID_ARGUMENT` if the query argument is malformed.\n* `NOT_FOUND` if any users specified in the query arguments do not exist.");
            courses0 = courses0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates one or more fields in a course.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to modify the\nrequested course or for access errors.\n* `NOT_FOUND` if no course exists with the requested ID.\n* `INVALID_ARGUMENT` if invalid fields are specified in the update mask or\nif no update mask is supplied.\n* `FAILED_PRECONDITION` for the following request errors:\n    * CourseNotModifiable");
            courses0 = courses0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a course.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to modify the\nrequested course or for access errors.\n* `NOT_FOUND` if no course exists with the requested ID.\n* `FAILED_PRECONDITION` for the following request errors:\n    * CourseNotModifiable");
            courses0 = courses0.subcommand(mcmd);
        }
        let mut invitations0 = SubCommand::with_name("invitations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: accept, create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("accept").about("Accepts an invitation, removing it and adding the invited user to the\nteachers or students (as appropriate) of the specified course. Only the\ninvited user may accept an invitation.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to accept the\nrequested invitation or for access errors.\n* `FAILED_PRECONDITION` for the following request errors:\n    * CourseMemberLimitReached\n    * CourseNotModifiable\n    * CourseTeacherLimitReached\n    * UserGroupsMembershipLimitReached\n* `NOT_FOUND` if no invitation exists with the requested ID.");
            invitations0 = invitations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates an invitation. Only one invitation for a user and course may exist\nat a time. Delete and re-create an invitation to make changes.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to create\ninvitations for this course or for access errors.\n* `NOT_FOUND` if the course or the user does not exist.\n* `FAILED_PRECONDITION` if the requested user\'s account is disabled or if\nthe user already has this role or a role with greater permissions.\n* `ALREADY_EXISTS` if an invitation for the specified user and course\nalready exists.");
            invitations0 = invitations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an invitation.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to delete the\nrequested invitation or for access errors.\n* `NOT_FOUND` if no invitation exists with the requested ID.");
            invitations0 = invitations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns an invitation.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to view the\nrequested invitation or for access errors.\n* `NOT_FOUND` if no invitation exists with the requested ID.");
            invitations0 = invitations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns a list of invitations that the requesting user is permitted to\nview, restricted to those that match the list request.\n\n*Note:* At least one of `user_id` or `course_id` must be supplied. Both\nfields can be supplied.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` for access errors.");
            invitations0 = invitations0.subcommand(mcmd);
        }
        let mut registrations0 = SubCommand::with_name("registrations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create and delete");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a `Registration`, causing Classroom to start sending notifications\nfrom the provided `feed` to the destination provided in `cloudPubSubTopic`.\n\nReturns the created `Registration`. Currently, this will be the same as\nthe argument, but with server-assigned fields such as `expiry_time` and\n`id` filled in.\n\nNote that any value specified for the `expiry_time` or `id` fields will be\nignored.\n\nWhile Classroom may validate the `cloudPubSubTopic` and return errors on a\nbest effort basis, it is the caller\'s responsibility to ensure that it\nexists and that Classroom has permission to publish to it.\n\nThis method may return the following error codes:\n\n* `PERMISSION_DENIED` if:\n    * the authenticated user does not have permission to receive\n      notifications from the requested field; or\n    * the credential provided does not include the appropriate scope for\n      the requested feed.\n    * another access error is encountered.\n* `INVALID_ARGUMENT` if:\n    * no `cloudPubsubTopic` is specified, or the specified\n      `cloudPubsubTopic` is not valid; or\n    * no `feed` is specified, or the specified `feed` is not valid.\n* `NOT_FOUND` if:\n    * the specified `feed` cannot be located, or the requesting user does\n      not have permission to determine whether or not it exists; or\n    * the specified `cloudPubsubTopic` cannot be located, or Classroom has\n      not been granted permission to publish to it.");
            registrations0 = registrations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a `Registration`, causing Classroom to stop sending notifications\nfor that `Registration`.");
            registrations0 = registrations0.subcommand(mcmd);
        }
        let mut user_profiles0 = SubCommand::with_name("user_profiles")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Returns a user profile.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to access\nthis user profile, if no profile exists with the requested ID, or for\naccess errors.");
            user_profiles0 = user_profiles0.subcommand(mcmd);
        }
        let mut aliases1 = SubCommand::with_name("aliases")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an alias for a course.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to create the\nalias or for access errors.\n* `NOT_FOUND` if the course does not exist.\n* `ALREADY_EXISTS` if the alias already exists.\n* `FAILED_PRECONDITION` if the alias requested does not make sense for the\n  requesting user or course (for example, if a user not in a domain\n  attempts to access a domain-scoped alias).");
            aliases1 = aliases1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an alias of a course.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to remove the\nalias or for access errors.\n* `NOT_FOUND` if the alias does not exist.\n* `FAILED_PRECONDITION` if the alias requested does not make sense for the\n  requesting user or course (for example, if a user not in a domain\n  attempts to delete a domain-scoped alias).");
            aliases1 = aliases1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns a list of aliases for a course.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to access the\ncourse or for access errors.\n* `NOT_FOUND` if the course does not exist.");
            aliases1 = aliases1.subcommand(mcmd);
        }
        let mut announcements1 = SubCommand::with_name("announcements")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, modify_assignees and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an announcement.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to access the\nrequested course, create announcements in the requested course, share a\nDrive attachment, or for access errors.\n* `INVALID_ARGUMENT` if the request is malformed.\n* `NOT_FOUND` if the requested course does not exist.\n* `FAILED_PRECONDITION` for the following request error:\n    * AttachmentNotVisible");
            announcements1 = announcements1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an announcement.\n\nThis request must be made by the Developer Console project of the\n[OAuth client ID](https://support.google.com/cloud/answer/6158849) used to\ncreate the corresponding announcement item.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting developer project did not create\nthe corresponding announcement, if the requesting user is not permitted\nto delete the requested course or for access errors.\n* `FAILED_PRECONDITION` if the requested announcement has already been\ndeleted.\n* `NOT_FOUND` if no course exists with the requested ID.");
            announcements1 = announcements1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns an announcement.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to access the\nrequested course or announcement, or for access errors.\n* `INVALID_ARGUMENT` if the request is malformed.\n* `NOT_FOUND` if the requested course or announcement does not exist.");
            announcements1 = announcements1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns a list of announcements that the requester is permitted to view.\n\nCourse students may only view `PUBLISHED` announcements. Course teachers\nand domain administrators may view all announcements.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to access\nthe requested course or for access errors.\n* `INVALID_ARGUMENT` if the request is malformed.\n* `NOT_FOUND` if the requested course does not exist.");
            announcements1 = announcements1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("modify_assignees").about("Modifies assignee mode and options of an announcement.\n\nOnly a teacher of the course that contains the announcement may\ncall this method.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to access the\nrequested course or course work or for access errors.\n* `INVALID_ARGUMENT` if the request is malformed.\n* `NOT_FOUND` if the requested course or course work does not exist.");
            announcements1 = announcements1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates one or more fields of an announcement.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting developer project did not create\nthe corresponding announcement or for access errors.\n* `INVALID_ARGUMENT` if the request is malformed.\n* `FAILED_PRECONDITION` if the requested announcement has already been\ndeleted.\n* `NOT_FOUND` if the requested course or announcement does not exist");
            announcements1 = announcements1.subcommand(mcmd);
        }
        let mut course_work1 = SubCommand::with_name("course_work")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, modify_assignees and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates course work.\n\nThe resulting course work (and corresponding student submissions) are\nassociated with the Developer Console project of the\n[OAuth client ID](https://support.google.com/cloud/answer/6158849) used to\nmake the request. Classroom API requests to modify course work and student\nsubmissions must be made with an OAuth client ID from the associated\nDeveloper Console project.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to access the\nrequested course, create course work in the requested course, share a\nDrive attachment, or for access errors.\n* `INVALID_ARGUMENT` if the request is malformed.\n* `NOT_FOUND` if the requested course does not exist.\n* `FAILED_PRECONDITION` for the following request error:\n    * AttachmentNotVisible");
            course_work1 = course_work1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a course work.\n\nThis request must be made by the Developer Console project of the\n[OAuth client ID](https://support.google.com/cloud/answer/6158849) used to\ncreate the corresponding course work item.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting developer project did not create\nthe corresponding course work, if the requesting user is not permitted\nto delete the requested course or for access errors.\n* `FAILED_PRECONDITION` if the requested course work has already been\ndeleted.\n* `NOT_FOUND` if no course exists with the requested ID.");
            course_work1 = course_work1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns course work.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to access the\nrequested course or course work, or for access errors.\n* `INVALID_ARGUMENT` if the request is malformed.\n* `NOT_FOUND` if the requested course or course work does not exist.");
            course_work1 = course_work1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns a list of course work that the requester is permitted to view.\n\nCourse students may only view `PUBLISHED` course work. Course teachers\nand domain administrators may view all course work.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to access\nthe requested course or for access errors.\n* `INVALID_ARGUMENT` if the request is malformed.\n* `NOT_FOUND` if the requested course does not exist.");
            course_work1 = course_work1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("modify_assignees").about("Modifies assignee mode and options of a coursework.\n\nOnly a teacher of the course that contains the coursework may\ncall this method.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to access the\nrequested course or course work or for access errors.\n* `INVALID_ARGUMENT` if the request is malformed.\n* `NOT_FOUND` if the requested course or course work does not exist.");
            course_work1 = course_work1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates one or more fields of a course work.\n\nSee google.classroom.v1.CourseWork for details\nof which fields may be updated and who may change them.\n\nThis request must be made by the Developer Console project of the\n[OAuth client ID](https://support.google.com/cloud/answer/6158849) used to\ncreate the corresponding course work item.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting developer project did not create\nthe corresponding course work, if the user is not permitted to make the\nrequested modification to the student submission, or for\naccess errors.\n* `INVALID_ARGUMENT` if the request is malformed.\n* `FAILED_PRECONDITION` if the requested course work has already been\ndeleted.\n* `NOT_FOUND` if the requested course, course work, or student submission\ndoes not exist.");
            course_work1 = course_work1.subcommand(mcmd);
        }
        let mut students1 = SubCommand::with_name("students")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Adds a user as a student of a course.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to create\nstudents in this course or for access errors.\n* `NOT_FOUND` if the requested course ID does not exist.\n* `FAILED_PRECONDITION` if the requested user\'s account is disabled,\nfor the following request errors:\n    * CourseMemberLimitReached\n    * CourseNotModifiable\n    * UserGroupsMembershipLimitReached\n* `ALREADY_EXISTS` if the user is already a student or teacher in the\ncourse.");
            students1 = students1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a student of a course.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to delete\nstudents of this course or for access errors.\n* `NOT_FOUND` if no student of this course has the requested ID or if the\ncourse does not exist.");
            students1 = students1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns a student of a course.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to view\nstudents of this course or for access errors.\n* `NOT_FOUND` if no student of this course has the requested ID or if the\ncourse does not exist.");
            students1 = students1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns a list of students of this course that the requester\nis permitted to view.\n\nThis method returns the following error codes:\n\n* `NOT_FOUND` if the course does not exist.\n* `PERMISSION_DENIED` for access errors.");
            students1 = students1.subcommand(mcmd);
        }
        let mut teachers1 = SubCommand::with_name("teachers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a teacher of a course.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not  permitted to create\nteachers in this course or for access errors.\n* `NOT_FOUND` if the requested course ID does not exist.\n* `FAILED_PRECONDITION` if the requested user\'s account is disabled,\nfor the following request errors:\n    * CourseMemberLimitReached\n    * CourseNotModifiable\n    * CourseTeacherLimitReached\n    * UserGroupsMembershipLimitReached\n* `ALREADY_EXISTS` if the user is already a teacher or student in the\ncourse.");
            teachers1 = teachers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a teacher of a course.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to delete\nteachers of this course or for access errors.\n* `NOT_FOUND` if no teacher of this course has the requested ID or if the\ncourse does not exist.\n* `FAILED_PRECONDITION` if the requested ID belongs to the primary teacher\nof this course.");
            teachers1 = teachers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns a teacher of a course.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to view\nteachers of this course or for access errors.\n* `NOT_FOUND` if no teacher of this course has the requested ID or if the\ncourse does not exist.");
            teachers1 = teachers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns a list of teachers of this course that the requester\nis permitted to view.\n\nThis method returns the following error codes:\n\n* `NOT_FOUND` if the course does not exist.\n* `PERMISSION_DENIED` for access errors.");
            teachers1 = teachers1.subcommand(mcmd);
        }
        let mut topics1 = SubCommand::with_name("topics")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a topic.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to access the\nrequested course, create a topic in the requested course,\nor for access errors.\n* `INVALID_ARGUMENT` if the request is malformed.\n* `NOT_FOUND` if the requested course does not exist.");
            topics1 = topics1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a topic.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not allowed to delete the\nrequested topic or for access errors.\n* `FAILED_PRECONDITION` if the requested topic has already been\ndeleted.\n* `NOT_FOUND` if no course or topic exists with the requested ID.");
            topics1 = topics1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns a topic.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to access the\nrequested course or topic, or for access errors.\n* `INVALID_ARGUMENT` if the request is malformed.\n* `NOT_FOUND` if the requested course or topic does not exist.");
            topics1 = topics1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns the list of topics that the requester is permitted to view.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to access\nthe requested course or for access errors.\n* `INVALID_ARGUMENT` if the request is malformed.\n* `NOT_FOUND` if the requested course does not exist.");
            topics1 = topics1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates one or more fields of a topic.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting developer project did not create\nthe corresponding topic or for access errors.\n* `INVALID_ARGUMENT` if the request is malformed.\n* `NOT_FOUND` if the requested course or topic does not exist");
            topics1 = topics1.subcommand(mcmd);
        }
        let mut guardian_invitations1 = SubCommand::with_name("guardian_invitations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a guardian invitation, and sends an email to the guardian asking\nthem to confirm that they are the student\'s guardian.\n\nOnce the guardian accepts the invitation, their `state` will change to\n`COMPLETED` and they will start receiving guardian notifications. A\n`Guardian` resource will also be created to represent the active guardian.\n\nThe request object must have the `student_id` and\n`invited_email_address` fields set. Failing to set these fields, or\nsetting any other fields in the request, will result in an error.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the current user does not have permission to\n  manage guardians, if the guardian in question has already rejected\n  too many requests for that student, if guardians are not enabled for the\n  domain in question, or for other access errors.\n* `RESOURCE_EXHAUSTED` if the student or guardian has exceeded the guardian\n  link limit.\n* `INVALID_ARGUMENT` if the guardian email address is not valid (for\n  example, if it is too long), or if the format of the student ID provided\n  cannot be recognized (it is not an email address, nor a `user_id` from\n  this API). This error will also be returned if read-only fields are set,\n  or if the `state` field is set to to a value other than `PENDING`.\n* `NOT_FOUND` if the student ID provided is a valid student ID, but\n  Classroom has no record of that student.\n* `ALREADY_EXISTS` if there is already a pending guardian invitation for\n  the student and `invited_email_address` provided, or if the provided\n  `invited_email_address` matches the Google account of an existing\n  `Guardian` for this user.");
            guardian_invitations1 = guardian_invitations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns a specific guardian invitation.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to view\n  guardian invitations for the student identified by the `student_id`, if\n  guardians are not enabled for the domain in question, or for other\n  access errors.\n* `INVALID_ARGUMENT` if a `student_id` is specified, but its format cannot\n  be recognized (it is not an email address, nor a `student_id` from the\n  API, nor the literal string `me`).\n* `NOT_FOUND` if Classroom cannot find any record of the given student or\n  `invitation_id`. May also be returned if the student exists, but the\n  requesting user does not have access to see that student.");
            guardian_invitations1 = guardian_invitations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns a list of guardian invitations that the requesting user is\npermitted to view, filtered by the parameters provided.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if a `student_id` is specified, and the requesting\n  user is not permitted to view guardian invitations for that student, if\n  `\"-\"` is specified as the `student_id` and the user is not a domain\n  administrator, if guardians are not enabled for the domain in question,\n  or for other access errors.\n* `INVALID_ARGUMENT` if a `student_id` is specified, but its format cannot\n  be recognized (it is not an email address, nor a `student_id` from the\n  API, nor the literal string `me`). May also be returned if an invalid\n  `page_token` or `state` is provided.\n* `NOT_FOUND` if a `student_id` is specified, and its format can be\n  recognized, but Classroom has no record of that student.");
            guardian_invitations1 = guardian_invitations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Modifies a guardian invitation.\n\nCurrently, the only valid modification is to change the `state` from\n`PENDING` to `COMPLETE`. This has the effect of withdrawing the invitation.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the current user does not have permission to\n  manage guardians, if guardians are not enabled for the domain in question\n  or for other access errors.\n* `FAILED_PRECONDITION` if the guardian link is not in the `PENDING` state.\n* `INVALID_ARGUMENT` if the format of the student ID provided\n  cannot be recognized (it is not an email address, nor a `user_id` from\n  this API), or if the passed `GuardianInvitation` has a `state` other than\n  `COMPLETE`, or if it modifies fields other than `state`.\n* `NOT_FOUND` if the student ID provided is a valid student ID, but\n  Classroom has no record of that student, or if the `id` field does not\n  refer to a guardian invitation known to Classroom.");
            guardian_invitations1 = guardian_invitations1.subcommand(mcmd);
        }
        let mut guardians1 = SubCommand::with_name("guardians")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a guardian.\n\nThe guardian will no longer receive guardian notifications and the guardian\nwill no longer be accessible via the API.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if no user that matches the provided `student_id`\n  is visible to the requesting user, if the requesting user is not\n  permitted to manage guardians for the student identified by the\n  `student_id`, if guardians are not enabled for the domain in question,\n  or for other access errors.\n* `INVALID_ARGUMENT` if a `student_id` is specified, but its format cannot\n  be recognized (it is not an email address, nor a `student_id` from the\n  API).\n* `NOT_FOUND` if the requesting user is permitted to modify guardians for\n  the requested `student_id`, but no `Guardian` record exists for that\n  student with the provided `guardian_id`.");
            guardians1 = guardians1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns a specific guardian.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if no user that matches the provided `student_id`\n  is visible to the requesting user, if the requesting user is not\n  permitted to view guardian information for the student identified by the\n  `student_id`, if guardians are not enabled for the domain in question,\n  or for other access errors.\n* `INVALID_ARGUMENT` if a `student_id` is specified, but its format cannot\n  be recognized (it is not an email address, nor a `student_id` from the\n  API, nor the literal string `me`).\n* `NOT_FOUND` if the requesting user is permitted to view guardians for\n  the requested `student_id`, but no `Guardian` record exists for that\n  student that matches the provided `guardian_id`.");
            guardians1 = guardians1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns a list of guardians that the requesting user is permitted to\nview, restricted to those that match the request.\n\nTo list guardians for any student that the requesting user may view\nguardians for, use the literal character `-` for the student ID.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if a `student_id` is specified, and the requesting\n  user is not permitted to view guardian information for that student, if\n  `\"-\"` is specified as the `student_id` and the user is not a domain\n  administrator, if guardians are not enabled for the domain in question,\n  if the `invited_email_address` filter is set by a user who is not a\n  domain administrator, or for other access errors.\n* `INVALID_ARGUMENT` if a `student_id` is specified, but its format cannot\n  be recognized (it is not an email address, nor a `student_id` from the\n  API, nor the literal string `me`). May also be returned if an invalid\n  `page_token` is provided.\n* `NOT_FOUND` if a `student_id` is specified, and its format can be\n  recognized, but Classroom has no record of that student.");
            guardians1 = guardians1.subcommand(mcmd);
        }
        let mut student_submissions2 = SubCommand::with_name("student_submissions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list, modify_attachments, patch, reclaim, r#return and turn_in");
        {
            let mcmd = SubCommand::with_name("get").about("Returns a student submission.\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to access the\nrequested course, course work, or student submission or for\naccess errors.\n* `INVALID_ARGUMENT` if the request is malformed.\n* `NOT_FOUND` if the requested course, course work, or student submission\ndoes not exist.");
            student_submissions2 = student_submissions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns a list of student submissions that the requester is permitted to\nview, factoring in the OAuth scopes of the request.\n`-` may be specified as the `course_work_id` to include student\nsubmissions for multiple course work items.\n\nCourse students may only view their own work. Course teachers\nand domain administrators may view all student submissions.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to access the\nrequested course or course work, or for access errors.\n* `INVALID_ARGUMENT` if the request is malformed.\n* `NOT_FOUND` if the requested course does not exist.");
            student_submissions2 = student_submissions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("modify_attachments").about("Modifies attachments of student submission.\n\nAttachments may only be added to student submissions belonging to course\nwork objects with a `workType` of `ASSIGNMENT`.\n\nThis request must be made by the Developer Console project of the\n[OAuth client ID](https://support.google.com/cloud/answer/6158849) used to\ncreate the corresponding course work item.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to access the\nrequested course or course work, if the user is not permitted to modify\nattachments on the requested student submission, or for\naccess errors.\n* `INVALID_ARGUMENT` if the request is malformed.\n* `NOT_FOUND` if the requested course, course work, or student submission\ndoes not exist.");
            student_submissions2 = student_submissions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates one or more fields of a student submission.\n\nSee google.classroom.v1.StudentSubmission for details\nof which fields may be updated and who may change them.\n\nThis request must be made by the Developer Console project of the\n[OAuth client ID](https://support.google.com/cloud/answer/6158849) used to\ncreate the corresponding course work item.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting developer project did not create\nthe corresponding course work, if the user is not permitted to make the\nrequested modification to the student submission, or for\naccess errors.\n* `INVALID_ARGUMENT` if the request is malformed.\n* `NOT_FOUND` if the requested course, course work, or student submission\ndoes not exist.");
            student_submissions2 = student_submissions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reclaim").about("Reclaims a student submission on behalf of the student that owns it.\n\nReclaiming a student submission transfers ownership of attached Drive\nfiles to the student and updates the submission state.\n\nOnly the student that owns the requested student submission may call this\nmethod, and only for a student submission that has been turned in.\n\nThis request must be made by the Developer Console project of the\n[OAuth client ID](https://support.google.com/cloud/answer/6158849) used to\ncreate the corresponding course work item.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to access the\nrequested course or course work, unsubmit the requested student submission,\nor for access errors.\n* `FAILED_PRECONDITION` if the student submission has not been turned in.\n* `INVALID_ARGUMENT` if the request is malformed.\n* `NOT_FOUND` if the requested course, course work, or student submission\ndoes not exist.");
            student_submissions2 = student_submissions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("r#return").about("Returns a student submission.\n\nReturning a student submission transfers ownership of attached Drive\nfiles to the student and may also update the submission state.\nUnlike the Classroom application, returning a student submission does not\nset assignedGrade to the draftGrade value.\n\nOnly a teacher of the course that contains the requested student submission\nmay call this method.\n\nThis request must be made by the Developer Console project of the\n[OAuth client ID](https://support.google.com/cloud/answer/6158849) used to\ncreate the corresponding course work item.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to access the\nrequested course or course work, return the requested student submission,\nor for access errors.\n* `INVALID_ARGUMENT` if the request is malformed.\n* `NOT_FOUND` if the requested course, course work, or student submission\ndoes not exist.");
            student_submissions2 = student_submissions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("turn_in").about("Turns in a student submission.\n\nTurning in a student submission transfers ownership of attached Drive\nfiles to the teacher and may also update the submission state.\n\nThis may only be called by the student that owns the specified student\nsubmission.\n\nThis request must be made by the Developer Console project of the\n[OAuth client ID](https://support.google.com/cloud/answer/6158849) used to\ncreate the corresponding course work item.\n\nThis method returns the following error codes:\n\n* `PERMISSION_DENIED` if the requesting user is not permitted to access the\nrequested course or course work, turn in the requested student submission,\nor for access errors.\n* `INVALID_ARGUMENT` if the request is malformed.\n* `NOT_FOUND` if the requested course, course work, or student submission\ndoes not exist.");
            student_submissions2 = student_submissions2.subcommand(mcmd);
        }
        course_work1 = course_work1.subcommand(student_submissions2);
        user_profiles0 = user_profiles0.subcommand(guardians1);
        user_profiles0 = user_profiles0.subcommand(guardian_invitations1);
        courses0 = courses0.subcommand(topics1);
        courses0 = courses0.subcommand(teachers1);
        courses0 = courses0.subcommand(students1);
        courses0 = courses0.subcommand(course_work1);
        courses0 = courses0.subcommand(announcements1);
        courses0 = courses0.subcommand(aliases1);
        app = app.subcommand(user_profiles0);
        app = app.subcommand(registrations0);
        app = app.subcommand(invitations0);
        app = app.subcommand(courses0);

        Self { app }
    }
}
use google_classroom1 as api;

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
