import { useLogin } from "./useLogin";
import { useRegister } from "./useRegister";
import { useVerify } from "./useVerify";
import { useResendVerification } from "./useResendVerification";

export function useAuthActions() {
  const login = useLogin();
  const register = useRegister();
  const verify = useVerify();
  const resend = useResendVerification();

  return {
    ...login,
    ...register,
    ...verify,
    ...resend
  };
}
