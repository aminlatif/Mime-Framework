# Mime Security Package

Goals:
* **Authentication**
* **Authorization**
* **Protection against common exploits**
    * CSRF (Cross Site Request Forgery)
    * HTTP Header
    * HTTP Request
* Access control
* Role-based security
* Express-based security
* OAuth2
* JWT

## Key Tables and Their Uses
1. `users`
    * Basic user credentials
    * Used to load `UserDetails` object during authentication.
2. `authorities`
    * Maps usernames to their roles/authorities (e.g. `ROLE_USER`, `ROLE_ADMIN`)
3. `groups`, `group_members`, and `group_authorities`
    * Adds **group support**
    * User roles can be derived from group membership, not just directly from the `authorities` table.
    * `groups` store group identities (e.g. "admin_group", "user_group")
    * `group_members` maps a username to a group
    * `group_authorities` defines which authorities are assigned to each group.
4. `persistent_logins`
    * "remember-me" (persistent token) implementation
    * When a user chooses "remember me", a token is saved in this table. On subsequent visits, it validates the token and logs the user in based on that rather than requiring credentials again.
5. `acl_sid`, `acl_class`, `acl_object_identity`, and `acl_entry`
    * These are used when you want fine-grained, object-level permissions (e.g. "user A can edit this specific document but not that one").
    * `acl_sid`: Security identities (SID), representing either users or authorities.
    * `acl_class`: Stores "class" of domain objects that ACLs apply to (e.g. the Java class name).
    * `acl_object_identity`: Maps a domain object instance (by class and identifier) to an ACL record.
    * `acl_entry`: The actual entries of the ACL (permissions), linking object identities (`acl_object_identity`) to SIDs, with permission masks, grant/deny, order, etc.
6. `oauth2_authorized_client`
    * This table allows Spring Security to store access tokens and refresh tokens for OAuth2 clients (useful when you're building an OAuth2 client that needs persistent tokens).