export type AuthUser = {
  id: string;
  email: string;
  name: string;
  is_admin: boolean;
  is_active: boolean;
  is_verified: boolean;
};

export type LoginInput = {
  email: string;
  password_hash: string;
};

export type LoginResult = {
  user: AuthUser;
};

export type RegisterInput = {
  email: string;
  password_hash: string;
  name: string;
};

export type RegisterResult = {
  user: AuthUser;
};

export type VerifyInput = {
  token: string;
};

export type VerifyResult = {
  user: AuthUser;
};

export type ResendVerificationInput = {
  email: string;
};

export type AcceptInviteInput = {
  invite_token: string;
};

export type AcceptInviteResult = {
  user_id: string;
  project_id: string;
  role: string;
};
