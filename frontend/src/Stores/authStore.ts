import { create } from "zustand";
import { persist } from "zustand/middleware";

export type AuthUser = {
  id: string;
  email: string;
  name: string;
  is_admin: boolean;
  is_active: boolean;
  is_verified: boolean;
};

interface AuthState {
  user: AuthUser | null;
  setSession: (user: AuthUser) => void;
  setVerified: () => void;
  clearSession: () => void;
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set) => ({
      user: null,
      setSession: (user) => set({ user }),
      setVerified: () =>
        set((state) => ({
          user: state.user ? { ...state.user, is_verified: true } : null,
        })),
      clearSession: () => set({ user: null }),
    }),
    { name: "audio-dsp-auth" }
  )
);
