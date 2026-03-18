import AuthSplitLayout from "@/components/auth/AuthSplitLayout";
import { Button } from "@/components/ui/button";
import { useNavigate } from "react-router-dom";

export default function Onboarding() {
  const navigate = useNavigate();

  return (
    <AuthSplitLayout title="You're all set" subtitle="Your account has been created.">
      <div className="mt-6">
        <Button className="w-full" onClick={() => navigate("/dashboard")}>
          Go to dashboard
        </Button>
      </div>
    </AuthSplitLayout>
  );
}
