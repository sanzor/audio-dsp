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
};
