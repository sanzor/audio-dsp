import { useMutation } from "@tanstack/react-query";
import { resendVerification } from "../../Services/auth/authService";

export function useResendVerification() {
  const resendMutation = useMutation({
    mutationFn: resendVerification,
  });

  return {
    resend: resendMutation.mutate,
    isResending: resendMutation.isPending,
    resendSuccess: resendMutation.isSuccess,
    resendError: resendMutation.error,
  };
}
