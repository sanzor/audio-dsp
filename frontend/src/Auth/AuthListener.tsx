export function AuthListener() {
  const loginSuccess = useAuthStore(s => s.loginSuccess);
  const navigate = useNavigate();

  useEffect(() => {
    const handler = (event: MessageEvent) => {
      if (event.data === "google_login_success") {
        loginSuccess(); 
        navigate("/dashboard");
      }
    };
    window.addEventListener("message", handler);
    return () => window.removeEventListener("message", handler);
  }, [loginSuccess, navigate]);

  return null;
}