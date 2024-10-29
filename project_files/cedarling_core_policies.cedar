// 1 Write ticket if user is author
permit (
    principal is Jans::User,
    action == Jans::Action::"Write",
    resource is Jans::Ticket
)
when { principal.sub == resource.author_user_id };

// 1 Write ticket if user in same organization
permit (
    principal is Jans::User,
    action == Jans::Action::"Write",
    resource is Jans::Ticket
)
when { principal.organization_id == resource.organization_id };

// 2 allow  Write ticket for any workload
permit (
    principal is Jans::Workload,
    action == Jans::Action::"Write",
    resource is Jans::Ticket
);