import { BrowserRouter, Navigate, Route, Routes } from "react-router-dom";
import AdminShell from "@/components/layout/admin/AdminShell";
import AccountShell from "@/components/layout/account/AccountShell";
import {
  DEFAULT_SUPER_ADMIN_PATH,
  superAdminNavItems,
} from "@/components/layout/app/routes";
import CatchAllRedirect from "@/components/routing/CatchAllRedirect";
import PublicOnlyRoute from "@/components/routing/PublicOnlyRoute";
import RequireAuth from "@/components/routing/RequireAuth";
import RequireSuperadmin from "@/components/routing/RequireSuperadmin";
import { Dashboard } from "@/components/dashboard/dashboard";
import AdminSectionPage from "@/pages/admin/AdminSectionPage";
import AccountProfile from "@/pages/account/AccountProfile";
import AccountSubscriptions from "@/pages/account/AccountSubscriptions";
import AccountShop from "@/pages/account/AccountShop";
import AccountPurchases from "@/pages/account/AccountPurchases";
import AccountBilling from "@/pages/account/AccountBilling";
import AccountApiKeys from "@/pages/account/AccountApiKeys";
import AccountSettings from "@/pages/account/AccountSettings";
import Onboarding from "@/pages/Onboarding";
import Verify from "@/pages/Verify";
import SignIn from "@/pages/auth/SignIn";
import SignUp from "@/pages/auth/SignUp";

function getAdminChildPath(path: string) {
  return path.replace("/admin/", "");
}

export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<CatchAllRedirect />} />

        <Route element={<PublicOnlyRoute />}>
          <Route path="/login" element={<SignIn />} />
          <Route path="/register" element={<SignUp />} />
          <Route path="/verify" element={<Verify />} />
          <Route path="/onboarding" element={<Onboarding />} />
        </Route>

        <Route element={<RequireSuperadmin />}>
          <Route path="/admin" element={<AdminShell />}>
            <Route
              index
              element={<Navigate to={getAdminChildPath(DEFAULT_SUPER_ADMIN_PATH)} replace />}
            />
            {superAdminNavItems.map((item) => (
              <Route
                key={item.path}
                path={getAdminChildPath(item.path)}
                element={
                  <AdminSectionPage title={item.label} permission={item.permission} />
                }
              />
            ))}
          </Route>
        </Route>

        <Route element={<RequireAuth />}>
          <Route path="/dashboard" element={<Dashboard />} />
          <Route path="/account" element={<AccountShell />}>
            <Route index element={<Navigate to="profile" replace />} />
            <Route path="profile" element={<AccountProfile />} />
            <Route path="subscriptions" element={<AccountSubscriptions />} />
            <Route path="shop" element={<AccountShop />} />
            <Route path="purchases" element={<AccountPurchases />} />
            <Route path="billing" element={<AccountBilling />} />
            <Route path="api-keys" element={<AccountApiKeys />} />
            <Route path="settings" element={<AccountSettings />} />
          </Route>
        </Route>

        <Route path="*" element={<CatchAllRedirect />} />
      </Routes>
    </BrowserRouter>
  );
}
