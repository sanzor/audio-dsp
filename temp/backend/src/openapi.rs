use utoipa::OpenApi;

use crate::controllers::api_keys_controller::{
    ApiKeyResult, ApiKeysResult, CreateApiKeyInput, CreateApiKeyResult, GetApiKeyResult,
    UpdateApiKeyInput, UpdateApiKeyResult,
};
use crate::controllers::auth_controller::{
    AcceptInviteInput, AcceptInviteResult, LoginInput, LoginResult, RegisterUserOnlyInput,
    RegisterUserOnlyResult,
};
use crate::controllers::me_controller::{
    BootstrapInput, BootstrapOutput, SelectOrgInput, SelectOrgResult,
};
use crate::controllers::membership_roles_controller::{
    CreateMembershipRoleInput, CreateMembershipRoleResult,
};
use crate::controllers::memberships_controller::{
    MembershipResult, MembershipsResult,
};
use crate::controllers::organization_controller::{
    CreateOrganizationInput, CreateOrganizationResult, GetOrganizationResult,
    OrganizationMembershipResult, OrganizationResult, OrganizationsResult, UpdateOrganizationInput,
    UpdateOrganizationResult,
};
use crate::controllers::permissions_controller::{
    CreatePermissionInput, CreatePermissionResult, GetPermissionResult, PermissionResult,
    PermissionsResult, UpdatePermissionInput, UpdatePermissionResult,
};
use crate::controllers::role_permissions_controller::{
    CreateRolePermissionInput, CreateRolePermissionResult, GetRolePermissionResult,
    RolePermissionResult, RolePermissionsResult,
};
use crate::controllers::roles_controller::{
    CreateRoleInput, CreateRoleResult, GetRoleResult, RoleResult, RolesResult, UpdateRoleInput,
    UpdateRoleResult,
};
use crate::controllers::subscriptions_controller::{
    GetSubscriptionResult, SubscriptionResult, SubscriptionsResult, UpdateSubscriptionInput,
    UpdateSubscriptionResult,
};
use crate::controllers::user_controller::{
    CreateUserInput, CreateUserResult, GetUserResult, UpdateUserInput, UpdateUserResult,
    UsersResult,
};
use crate::controllers::{
    api_keys_controller, auth_controller, me_controller, membership_roles_controller,
    memberships_controller, organization_controller, permissions_controller,
    rate_limits_config_controller, role_permissions_controller, roles_controller,
    subscriptions_controller, usage_controller, user_controller,
};
use crate::controllers::usage_controller::{GetUsageResponse, UsageSnapshotResult};
use crate::domain::db::db_token_bucket_usage::DbTokenBucketUsage;
use crate::domain::db::db_token_window_usage::DbTokenWindowUsage;
use crate::rate_limits_config::{
    ApplyRateLimitRefillInput, RateLimitConfigInput, RateLimitConfigResponse, RateLimitRefillInput,
    RateLimitRefillResponse, UpsertRateLimitConfigInput,
};
use crate::users::user_result::UserResult;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "User Portal API",
        version = "0.1.0",
        description = "User and API key management portal.",
    ),
    paths(
        organization_controller::create_organization,
        organization_controller::update_organization,
        organization_controller::delete_organization,
        organization_controller::get_organization,
        organization_controller::get_all_organizations,
        user_controller::create_user,
        user_controller::update_user,
        user_controller::delete_user,
        user_controller::get_user,
        user_controller::get_all_users,
        memberships_controller::delete_membership,
        memberships_controller::list_memberships,
        roles_controller::create_role,
        roles_controller::update_role,
        roles_controller::delete_role,
        roles_controller::get_role,
        roles_controller::list_roles,
        permissions_controller::create_permission,
        permissions_controller::update_permission,
        permissions_controller::delete_permission,
        permissions_controller::get_permission,
        permissions_controller::list_permissions,
        role_permissions_controller::create_role_permission,
        role_permissions_controller::delete_role_permission,
        role_permissions_controller::get_role_permission,
        role_permissions_controller::list_role_permissions,
        membership_roles_controller::create_membership_role,
        membership_roles_controller::delete_membership_role,
        api_keys_controller::create_api_key,
        api_keys_controller::update_api_key,
        api_keys_controller::delete_api_key,
        api_keys_controller::get_api_key,
        api_keys_controller::get_all_api_keys,
        subscriptions_controller::update_subscription,
        subscriptions_controller::delete_subscription,
        subscriptions_controller::get_subscription,
        subscriptions_controller::list_subscriptions,
        rate_limits_config_controller::get_rate_limit_config,
        rate_limits_config_controller::update_rate_limit_config,
        usage_controller::get_usage,
        auth_controller::register_user,
        auth_controller::accept_invite,
        auth_controller::login,
        me_controller::select_org,
        me_controller::bootstrap
    ),
    components(
        schemas(
            CreateOrganizationInput,
            CreateOrganizationResult,
            UpdateOrganizationInput,
            UpdateOrganizationResult,
            OrganizationResult,
            OrganizationsResult,
            GetOrganizationResult,
            OrganizationMembershipResult,
            CreateUserInput,
            CreateUserResult,
            UpdateUserInput,
            UpdateUserResult,
            UsersResult,
            GetUserResult,
            MembershipResult,
            MembershipsResult,
            CreateRoleInput,
            CreateRoleResult,
            UpdateRoleInput,
            UpdateRoleResult,
            RoleResult,
            RolesResult,
            GetRoleResult,
            CreatePermissionInput,
            CreatePermissionResult,
            UpdatePermissionInput,
            UpdatePermissionResult,
            PermissionResult,
            PermissionsResult,
            GetPermissionResult,
            CreateRolePermissionInput,
            CreateRolePermissionResult,
            RolePermissionResult,
            RolePermissionsResult,
            GetRolePermissionResult,
            CreateMembershipRoleInput,
            CreateMembershipRoleResult,
            CreateApiKeyInput,
            CreateApiKeyResult,
            UpdateApiKeyInput,
            UpdateApiKeyResult,
            ApiKeyResult,
            ApiKeysResult,
            GetApiKeyResult,
            UpdateSubscriptionInput,
            UpdateSubscriptionResult,
            SubscriptionResult,
            SubscriptionsResult,
            GetSubscriptionResult,
            RateLimitConfigInput,
            RateLimitConfigResponse,
            UpsertRateLimitConfigInput,
            RateLimitRefillInput,
            ApplyRateLimitRefillInput,
            RateLimitRefillResponse,
            RegisterUserOnlyInput,
            RegisterUserOnlyResult,
            AcceptInviteInput,
            AcceptInviteResult,
            LoginInput,
            LoginResult,
            UserResult,
            SelectOrgInput,
            SelectOrgResult,
            BootstrapInput,
            BootstrapOutput,
            UsageSnapshotResult,
            GetUsageResponse,
            DbTokenBucketUsage,
            DbTokenWindowUsage
        )
    ),
    tags(
        (name = "Organizations", description = "Organization management endpoints"),
        (name = "Users", description = "User management endpoints"),
        (name = "Memberships", description = "Organization memberships"),
        (name = "MembershipRoles", description = "Role assignments for memberships"),
        (name = "ApiKeys", description = "API key management endpoints"),
        (name = "Roles", description = "Organization roles"),
        (name = "Permissions", description = "Permission catalog"),
        (name = "RolePermissions", description = "Role to permission mappings"),
        (name = "Subscriptions", description = "Subscription management endpoints"),
        (name = "RateLimits", description = "Rate limit config endpoints"),
        (name = "Auth", description = "Authentication endpoints"),
        (name = "Me", description = "Current user and session endpoints"),
        (name = "Usage", description = "Token usage snapshot endpoints"),
    )
)]
pub struct ApiDoc;
