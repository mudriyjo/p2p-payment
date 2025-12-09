# TODO List
0. Fix all TODOs
1. Finalize User API (CRUD + listing)
2. Refactor Roles to same structure as User not sub domain + finalize API and routes to work with Roles
3. Prepare Swagger documentation for API
4. Prepare JWT login/logout flow for backoffice user with 2 step
- 1st step - login/password
- 2nd step - email code confirmation (for Dev use logging)
- Then generate JWT for user to login
5. Prepare Middleware to check protected and unprotected API by JWT and roles (first implement Admin)
6. Add Unit tests for existing domains and uncovered parts
7. Prepare base Usecases for backoffice
8. Research what would be the best option to save all action in activity log table for tracking and listing by user