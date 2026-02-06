export interface GoogleLoginResult {
  user_id: string;
  name: string;
  email: string;
  picture?: string;
}

export interface SessionResponse {
  user: {
    user_id: string;
    name: string;
    email: string;
    photo: string;
  } | null;
}
export interface RefreshResponse {
  status: 'refreshed';
}