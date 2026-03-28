export type PasteGraphParams={
  source:SourcePasteGraphParams,
  destination:DestinationPasteGraphParams
}
type SourcePasteGraphParams={
  graphId:number
}
type DestinationPasteGraphParams={
  regionId:number
}

export type PasteRegionParams={
  source:SourcePasteRegionParams,
  destination:DestinationPasteRegionParams
}
type SourcePasteRegionParams={
  regionId:number
}
type DestinationPasteRegionParams={
  regionSetId:number
}

export type PasteRegionSetParams={
  source:SourcePasteRegionSetParams,
  destination:DestinationPasteRegionSetParams
}

type SourcePasteRegionSetParams={
  regionSetId:number,
}
type DestinationPasteRegionSetParams={
  trackId:number,
}
