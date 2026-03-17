import { useMutation } from "@tanstack/react-query";
import { verifyEmail } from "../../Services/auth/authService";
import { useAuthStore } from "../../Stores/authStore";

export function useVerify() {
  const setVerified = useAuthStore((state) => state.setVerified);

  const verifyMutation = useMutation({
    mutationFn: verifyEmail,
    onSuccess: () => {
      setVerified();
    },
  });

  return {
    verify: verifyMutation.mutate,
    isVerifying: verifyMutation.isPending,
    verifyError: verifyMutation.error,
    verifySuccess: verifyMutation.isSuccess
  };
}
