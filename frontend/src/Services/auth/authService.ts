import { http } from "../http";
import type {
  AcceptInviteInput,
  AcceptInviteResult,
  LoginInput,
  LoginResult,
  RegisterInput,
  RegisterResult,
  ResendVerificationInput,
  VerifyInput,
  VerifyResult,
} from "./authTypes";

export async function login(input: LoginInput) {
  return http.post<LoginResult, LoginInput>("/auth/login", input);
}

export async function registerUser(input: RegisterInput) {
  return http.post<RegisterResult, RegisterInput>("/auth/register", input);
}

export async function verifyEmail(input: VerifyInput) {
  return http.post<VerifyResult, VerifyInput>("/auth/verify", input);
}

export async function resendVerification(input: ResendVerificationInput) {
  return http.post<void, ResendVerificationInput>("/auth/resend-verification", input);
}

export async function acceptInvite(input: AcceptInviteInput) {
  return http.post<AcceptInviteResult, AcceptInviteInput>("/auth/accept-invite", input);
}

export async function logout() {
  return http.post<void, Record<string, never>>("/auth/logout", {});
}
