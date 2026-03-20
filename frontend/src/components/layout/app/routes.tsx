export type NavItem = {
  label: string;
  path: string;
  permission?: string;
};

export const DEFAULT_APP_PATH = "/dashboard";
export const DEFAULT_SUPER_ADMIN_PATH = "/admin/users";

export const appNavItems: NavItem[] = [
  { label: "Dashboard", path: "/dashboard" },
];

export const accountNavItems: NavItem[] = [
  { label: "Profile", path: "/profile" },
  { label: "Subscriptions", path: "/subscriptions" },
  { label: "Shop", path: "/shop" },
  { label: "Purchases", path: "/purchases" },
  { label: "Billing", path: "/billing" },
  { label: "Settings", path: "/settings" },
];

export const superAdminNavItems: NavItem[] = [
  { label: "Products", path: "/admin/products", permission: "products:read" },
  { label: "Purchases", path: "/admin/purchases", permission: "purchased_products:read" },
  { label: "Users", path: DEFAULT_SUPER_ADMIN_PATH, permission: "users:read" },
  { label: "Subscriptions", path: "/admin/subscriptions", permission: "subscriptions:read" },
  { label: "Organizations", path: "/admin/organizations", permission: "organizations:read" },
  { label: "Tiers", path: "/admin/tier-configs", permission: "tier_configs:read" },
];
