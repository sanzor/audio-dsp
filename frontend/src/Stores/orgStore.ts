import { create } from "zustand";
import { persist } from "zustand/middleware";

export interface MembershipDTO {
  org_id: number;
  org_name: string;
  org_slug: string;
  roles: string[];
  permissions: string[];
}

interface OrgState {
  activeMembership: MembershipDTO | null;
  setActiveMembership: (membership: MembershipDTO) => void;
  hasPermission: (permission: string) => boolean | undefined;
  logout: () => void;
}

export const useOrgStore = create<OrgState>()(
  persist(
    (set, get) => ({
      activeMembership: null,
      setActiveMembership: (membership) => set({ activeMembership: membership }),
      hasPermission: (permission: string) => {
        const m = get().activeMembership;
        return m?.permissions.includes(permission);
      },
      logout: () => set({ activeMembership: null }),
    }),
    {
      name: "vigul-org-storage",
      version: 3,
      migrate: (persistedState, version) => {
        const state = persistedState as any;
        if (version < 2) {
          delete state?.memberships;
          const active = state?.activeMembership;
          if (active && !Array.isArray(active.roles)) {
            const roleName = typeof active.role_name === "string" ? active.role_name : undefined;
            active.roles = roleName ? [roleName] : [];
            delete active.role_name;
          }
        }
        if (version < 3) {
          const active = state?.activeMembership;
          if (active && typeof active.org_slug !== "string") {
            active.org_slug = "";
          }
        }
        return state;
      }
    }
  )
);
