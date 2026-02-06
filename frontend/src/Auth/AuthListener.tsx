import { useEffect } from "react";
import { useNavigate } from "react-router-dom"; // or just use window.location.href

// eslint-disable-next-line react-refresh/only-export-components
export function AuthListener() {
  const navigate = useNavigate();

  useEffect(() => {
    const handler = (event: MessageEvent) => {
      if (event.data === "google_login_success") {
        console.log("✅ Google login succeeded, fetching session...");

        // Option 1: Re-fetch session and redirect
        // await getSession().then(() => navigate("/dashboard"));

        // Option 2: Just navigate to dashboard (session will load there)
      setTimeout(() => {
        window.location.href = "/dashboard";
      }, 200); //

        // Optional: show a loading spinner, etc.
      }
    };

    window.addEventListener("message", handler);
    return () => window.removeEventListener("message", handler);
  }, [navigate]);

  return null; // It’s just a listener
}