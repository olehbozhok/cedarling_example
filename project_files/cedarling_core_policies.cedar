//0. Allows staff to create/reply any resource
permit (
    principal in Jans::Role::"Staff",
    action == Jans::Action::"Write",
    resource
);

//5. Allow Staff to  close, read Tickets.
permit (
    principal in Jans::Role::"Staff",
    action in [Jans::Action::"Close", Jans::Action::"Read"],
    resource
)
when { resource has ticketName };

//1. Allow Customer to create Ticket
permit (
    principal in Jans::Role::"Customer",
    action == Jans::Action::"Write",
    resource == Jans::Ticket::"unknown"
)
when { (resource has ticketName) };

//8. Allow Customer to manage Tickets when Ticket are created by this Customer
permit (
    principal in Jans::Role::"Customer",
    action == Jans::Action::"Write",
    resource
)
when
{
    (resource has author &&
     resource has ticketName &&
     resource.author in principal)
};

//6. Allow anyone to  reply, close, edit, view tickets associated with their organization
permit (
    principal,
    action in
        [Jans::Action::"Close", Jans::Action::"Read", Jans::Action::"Write"],
    resource
)
when
{
    resource has ticketName &&
    (principal has organization &&
     resource has organization &&
     resource.organization in principal.organization)
};

//2. Allow Partner to manage products
permit (
    principal in Jans::Role::"Partner",
    action == Jans::Action::"Write",
    resource
)
when { resource has productname };

//2. Allow Customer to manage products
permit (
    principal in Jans::Role::"Customer",
    action == Jans::Action::"Write",
    resource
)
when { resource has productname };

permit (
    principal in Jans::Role::"Staff",
    action == Jans::Action::"Write",
    resource
)
when { resource has productname };

//4. Allow Partner to manage tickets of his clients. tickets of his clients means that ticket’s organizationId is among Partner’s  supportsOrganizations list
permit (
    principal in Jans::Role::"Partner",
    action in
        [Jans::Action::"Close", Jans::Action::"Read", Jans::Action::"Write"],
    resource
)
when
{
    resource has userId &&
    resource has organization &&
    principal has supportsOrganizations &&
    resource.organization in principal.supportsOrganizations
};

//7. Allow Partner to  reply, close, view Tickets tickets of client who allowed access on Manage Partners Page means Parter Organization is among   ticket’s attribute supportedByOrganizations
permit (
    principal in Jans::Role::"Partner",
    action in
        [Jans::Action::"Reply",
         Jans::Action::"Close",
         Jans::Action::"Read",
         Jans::Action::"Write"],
    resource
)
when
{
    resource has ticketName &&
    resource has supportedByOrganizations &&
    (principal has organization &&
     principal.organization in resource.supportedByOrganizations)
};