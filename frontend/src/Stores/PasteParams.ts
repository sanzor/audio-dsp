export type PasteGraphParams={
  source:SourcePasteGraphParams,
  destination:DestinationPasteGraphParams
}
type SourcePasteGraphParams={
  graphId:string
}
type DestinationPasteGraphParams={
  regionId:string
}

export type PasteRegionParams={
  source:SourcePasteRegionParams,
  destination:DestinationPasteRegionParams
}
type SourcePasteRegionParams={
  regionId:string
}
type DestinationPasteRegionParams={
  regionSetId:string
}

export type PasteRegionSetParams={
  source:SourcePasteRegionSetParams,
  destination:DestinationPasteRegionSetParams
}

type SourcePasteRegionSetParams={
  regionSetId:string,
}
type DestinationPasteRegionSetParams={
  trackId:string,
}