import React from "react";
import { createRoot } from "react-dom/client";
import { BrowserRouter, Navigate, Route, Routes } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import "./index.css";

// Guards
import RequireAuth from "./components/routing/RequireAuth";
import RequireSuperadmin from "./components/routing/RequireSuperadmin";
import CatchAllRedirect from "./components/routing/CatchAllRedirect";

// Layouts
import AdminShell from "./components/layout/admin/AdminShell";

// Auth pages
import SignIn from "./pages/auth/SignIn";
import SignUp from "./pages/auth/SignUp";
import Verify from "./pages/Verify";
import Onboarding from "./pages/Onboarding";

// App pages
import { Dashboard } from "./components/dashboard/dashboard";

// Providers
import { AudioPlaybackCacheProvider } from "./Providers/AudioPlaybackCacheContext";

const queryClient = new QueryClient({
  defaultOptions: { queries: { staleTime: 1000 * 60 * 5 } },
});

function AppRouter() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Navigate to="/login" replace />} />

        {/* Public pages */}
        <Route path="/login" element={<SignIn />} />
        <Route path="/register" element={<SignUp />} />
        <Route path="/verify" element={<Verify />} />
        <Route path="/onboarding" element={<Onboarding />} />

        {/* Platform admin — guarded by RequireSuperadmin */}
        <Route element={<RequireSuperadmin />}>
          <Route element={<AdminShell />}>
            <Route path="/admin" element={<Navigate to="/admin/users" replace />} />
          </Route>
        </Route>

        {/* App — guarded by RequireAuth */}
        <Route element={<RequireAuth />}>
          <Route path="/dashboard" element={<Dashboard />} />
        </Route>

        {/* Catch-all */}
        <Route path="*" element={<CatchAllRedirect />} />
      </Routes>
    </BrowserRouter>
  );
}

createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <AudioPlaybackCacheProvider>
        <AppRouter />
      </AudioPlaybackCacheProvider>
    </QueryClientProvider>
  </React.StrictMode>
);
