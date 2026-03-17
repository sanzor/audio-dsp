export type NavItem = {
  label: string;
  path: string;
  permission?: string;
};
export const appNavItems:NavItem[]=[
  { label: "Dashboard", path: "/dashboard" },
];
export const adminNavItems: NavItem[] = [
  { label: "Dashboard", path: "/dashboard" },
  { label: "Members", path: "/users", permission: "memberships:read" },
  { label: "API Keys", path: "/api-keys", permission: "api_keys:read" },
  { label: "Subscriptions", path: "/subscriptions", permission: "subscriptions:read" },
  { label: "Shop", path: "/shop", permission: "products:read" },
  { label: "Purchases", path: "/purchases", permission: "purchased_products:read" },
  { label: "Billing", path: "/billing", permission: "billing_mode:read" },
  { label: "Settings", path: "/settings", permission: "organizations:update" },
];

export const superAdminNavItems: NavItem[] = [
  { label: "Products", path: "/admin/products", permission: "products:read" },
  { label: "Purchases", path: "/admin/purchases", permission: "purchased_products:read" },
  { label: "Users", path: "/admin/users", permission: "users:read" },
  { label: "Subscriptions", path: "/admin/subscriptions", permission: "subscriptions:read" },
  { label: "Organizations", path: "/admin/organizations", permission: "organizations:read" },
  { label: "Tiers", path: "/admin/tier-configs", permission: "tier_configs:read" },
];
