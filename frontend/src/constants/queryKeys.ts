export const QUERY_KEYS = {
  tracks: {
    all: () => ['tracks'] as const,
    byId: (trackId: string) => ['track', trackId] as const,
  },
  regionSets: {
    byTrack: (trackId: string) => ['regionSets', 'track', trackId] as const,
    byId: (regionSetId: string) => ['regionSet', regionSetId] as const,
  },
  regions: {
    bySet: (setId: string) => ['regions', 'region-set', setId] as const,
    byId: (regionId: string) => ['region', regionId] as const,
  },
  graphs: {
    byId: (graphId: string) => ['graph', graphId] as const,
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
};
