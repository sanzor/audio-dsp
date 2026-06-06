import { BrowserRouter, Navigate, Route, Routes } from "react-router-dom";
import AdminShell from "@/components/layout/admin/AdminShell";
import AccountShell from "@/components/layout/account/AccountShell";
import { useAuthStore } from "@/Stores/authStore";
import { useProjectStore } from "@/Stores/projectStore";
import {
  DEFAULT_SUPER_ADMIN_PATH,
  superAdminNavItems,
} from "@/components/layout/app/routes";
import { Dashboard } from "@/components/dashboard/dashboard";
import { CreatorShell } from "@/components/creator";
import AdminSectionPage from "@/pages/admin/AdminSectionPage";
import AccountProfile from "@/pages/account/AccountProfile";
import AccountSubscriptions from "@/pages/account/AccountSubscriptions";
import AccountShop from "@/pages/account/AccountShop";
import AccountPurchases from "@/pages/account/AccountPurchases";
import AccountBilling from "@/pages/account/AccountBilling";
import AccountSettings from "@/pages/account/AccountSettings";
import Onboarding from "@/pages/Onboarding";
import Verify from "@/pages/Verify";
import SignIn from "@/pages/auth/SignIn";
import SignUp from "@/pages/auth/SignUp";

function getAdminChildPath(path: string) {
  return path.replace("/admin/", "");
}

function getSignedInPath(isAdmin: boolean, projectCount: number) {
  if (isAdmin) {
    return DEFAULT_SUPER_ADMIN_PATH;
  }

  return projectCount === 0 ? "/onboarding" : "/dashboard";
}

export default function App() {
  const user = useAuthStore((state) => state.user);
  const projectCount = useProjectStore((state) => state.projects.length);
  const isAuthenticated = Boolean(user);
  const isAdmin = user?.is_admin ?? false;
  const signedInPath = getSignedInPath(isAdmin, projectCount);

  return (
    <BrowserRouter>
      <Routes>
        <Route
          path="/"
          element={<Navigate to={isAuthenticated ? signedInPath : "/login"} replace />}
        />

        <Route
          path="/login"
          element={isAuthenticated ? <Navigate to={signedInPath} replace /> : <SignIn />}
        />
        <Route
          path="/register"
          element={isAuthenticated ? <Navigate to={signedInPath} replace /> : <SignUp />}
        />
        <Route
          path="/verify"
          element={isAuthenticated ? <Navigate to={signedInPath} replace /> : <Verify />}
        />
        <Route
          path="/onboarding"
          element={
            !isAuthenticated ? (
              <Navigate to="/login" replace />
            ) : isAdmin ? (
              <Navigate to={DEFAULT_SUPER_ADMIN_PATH} replace />
            ) : projectCount > 0 ? (
              <Navigate to="/dashboard" replace />
            ) : (
              <Onboarding />
            )
          }
        />

        <Route
          path="/admin"
          element={
            !isAuthenticated ? (
              <Navigate to="/login" replace />
            ) : !isAdmin ? (
              <Navigate to={signedInPath} replace />
            ) : (
              <AdminShell />
            )
          }
        >
          <Route
            index
            element={<Navigate to={getAdminChildPath(DEFAULT_SUPER_ADMIN_PATH)} replace />}
          />
          {superAdminNavItems.map((item) => (
            <Route
              key={item.path}
              path={getAdminChildPath(item.path)}
              element={
                !isAuthenticated ? (
                  <Navigate to="/login" replace />
                ) : !isAdmin ? (
                  <Navigate to={signedInPath} replace />
                ) : (
                  <AdminSectionPage title={item.label} permission={item.permission} />
                )
              }
            />
          ))}
        </Route>

        <Route
          path="/dashboard"
          element={
            !isAuthenticated ? (
              <Navigate to="/login" replace />
            ) : isAdmin ? (
              <Navigate to={DEFAULT_SUPER_ADMIN_PATH} replace />
            ) : projectCount === 0 ? (
              <Navigate to="/onboarding" replace />
            ) : (
              <Dashboard />
            )
          }
        />
        <Route
          path="/account"
          element={
            !isAuthenticated ? (
              <Navigate to="/login" replace />
            ) : isAdmin ? (
              <Navigate to={DEFAULT_SUPER_ADMIN_PATH} replace />
            ) : (
              <AccountShell />
            )
          }
        >
          <Route index element={<Navigate to="profile" replace />} />
          <Route path="profile" element={<AccountProfile />} />
          <Route path="subscriptions" element={<AccountSubscriptions />} />
          <Route path="shop" element={<AccountShop />} />
          <Route path="purchases" element={<AccountPurchases />} />
          <Route path="billing" element={<AccountBilling />} />
          <Route path="settings" element={<AccountSettings />} />
        </Route>

        <Route
          path="/creator"
          element={
            !isAuthenticated ? (
              <Navigate to="/login" replace />
            ) : isAdmin ? (
              <Navigate to={DEFAULT_SUPER_ADMIN_PATH} replace />
            ) : projectCount === 0 ? (
              <Navigate to="/onboarding" replace />
            ) : (
              <CreatorShell />
            )
          }
        />

        <Route
          path="*"
          element={<Navigate to={isAuthenticated ? signedInPath : "/login"} replace />}
        />
      </Routes>
    </BrowserRouter>
  );
}
