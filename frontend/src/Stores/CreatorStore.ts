import { create } from "zustand";
import { persist } from "zustand/middleware";

interface CreatorState {
  selectedTransformId: number | null;
  setSelectedTransformId: (id: number | null) => void;
  // Last compile ticket issued per transform, so reopening/refreshing resumes
  // polling that ticket instead of losing track of an in-flight compile.
  activeTicketByTransform: Record<number, number>;
  setActiveTicket: (transformId: number, ticketId: number) => void;
}

export const useCreatorStore = create<CreatorState>()(
  persist(
    (set) => ({
      selectedTransformId: null,
      setSelectedTransformId: (id) => set({ selectedTransformId: id }),
      activeTicketByTransform: {},
      setActiveTicket: (transformId, ticketId) =>
        set((state) => ({
          activeTicketByTransform: { ...state.activeTicketByTransform, [transformId]: ticketId },
        })),
    }),
    {
      name: "audio-dsp-creator",
      partialize: (state) => ({ activeTicketByTransform: state.activeTicketByTransform }),
    }
  )
);
