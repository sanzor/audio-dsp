import React from "react";
import { createRoot } from "react-dom/client";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import App from "./App";
import "./index.css";

// Providers
import { AudioPlaybackCacheProvider } from "./Providers/AudioPlaybackCacheContext";

const queryClient = new QueryClient({
  defaultOptions: { queries: { staleTime: 1000 * 60 * 5 } },
});

createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <AudioPlaybackCacheProvider>
        <App />
      </AudioPlaybackCacheProvider>
    </QueryClientProvider>
  </React.StrictMode>
);
