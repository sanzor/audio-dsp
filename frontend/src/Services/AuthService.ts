// /auth/auth-service.ts

import type {
  SessionResponse,
  RefreshResponse,
} from '../Auth/AuthTypes';

const BASE_URL = import.meta.env.VITE_API_BASE_URL;

/**
 * Fetches the current user session.
 */
export async function getSession(): Promise<SessionResponse> {
  const url = `${BASE_URL}/auth/google/session`;
  // console.log("📡 Fetching session from:", url);

  try {
    const res = await fetch(url, {
      method: 'GET',
      credentials: 'include',
    });

    // console.log("📥 Response status:", res.status);

    if (!res.ok) {
      // const text = await res.text(); // helpful for debugging backend errors
      // console.error("❌ Response not OK:", res.status, text);
      throw new Error(`Failed to fetch session: ${res.status}`);
    }

    const session = await res.json();
    // console.log("✅ Session received:", session);
    return session;
  } catch (error) {
    console.error("❌ Fetch failed:", error);
    throw error;
  }
}

/**
 * Refreshes the access token using the refresh token (cookie).
 */
export async function refreshToken(): Promise<RefreshResponse> {
  const res = await fetch(`${BASE_URL}/auth/google/refresh`, {
    method: 'POST',
    credentials: 'include',
  });

  if (!res.ok) throw new Error('Refresh token failed');
  return res.json();
}

/**
 * Logs out the user by clearing auth-related cookies.
 */
export async function logout(): Promise<void> {
  const res = await fetch(`${BASE_URL}/auth/google/logout`, {
    method: 'POST',
    credentials: 'include',
  });

  if (!res.ok) throw new Error('Logout failed');
}

/**
 * Returns the backend's Google OAuth redirect URL.
 * Use this to open a popup or redirect the browser.
 */
export function getGoogleOAuthUrl(): string {
  return `${BASE_URL}/auth/google/`;
}
