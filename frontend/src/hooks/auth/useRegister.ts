import { useMutation } from "@tanstack/react-query";
import { registerUser } from "../../Services/auth/authService";
import { useAuthStore } from "../../Stores/authStore";

export function useRegister() {
  const setSession = useAuthStore((state) => state.setSession);

  const registerMutation = useMutation({
    mutationFn: registerUser,
    onSuccess: (result) => {
      setSession(result.user, result.token);
    },
  });

  return {
    signUp: registerMutation.mutate,
    isSigningUp: registerMutation.isPending,
    registerResult: registerMutation.data,
    isRegistered: registerMutation.isSuccess,
    registerError: registerMutation.error,
  };
}
