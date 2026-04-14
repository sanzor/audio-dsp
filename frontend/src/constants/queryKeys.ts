export const QUERY_KEYS = {
  tracks: {
    all: () => ['tracks'] as const,
    byId: (trackId: number) => ['track', trackId] as const,
  },
  workspace: {
    byProject: (projectId: number) => ['workspace', 'project', projectId] as const,
  },
  regionSets: {
    all: () => ['regionSets'] as const,
    byTrack: (trackId: number) => ['regionSets', 'track', trackId] as const,
    byId: (regionSetId: number) => ['regionSet', regionSetId] as const,
  },
  regions: {
    bySet: (setId: number) => ['regions', 'region-set', setId] as const,
    byId: (regionId: number) => ['region', regionId] as const,
  },
  graphs: {
    byId: (graphId: number) => ['graph', graphId] as const,
  },
  products: {
    active: () => ['products', 'active'] as const,
    all: () => ['products'] as const,
  },
  subscriptions: {
    mine: () => ['subscriptions', 'mine'] as const,
    active: () => ['subscriptions', 'mine', 'active'] as const,
  },
  purchases: {
    mine: () => ['purchases', 'mine'] as const,
  },
  invoices: {
    mine: (offset: number) => ['invoices', 'mine', offset] as const,
  },
  usage: {
    mine: () => ['usage', 'mine'] as const,
  },
  tierConfigs: {
    all: () => ['tier-configs'] as const,
  },
  transforms: {
    all: () => ['transforms'] as const,
    byId: (id: number) => ['transform', id] as const,
    wasm: (id: number) => ['transform', 'wasm', id] as const,
  },
  storedAudio: {
    byTrackId: (trackId: number) => ['stored-audio', trackId] as const,
  },
};
