export type Instruction =
  | { kind: 'FetchInput'; targetBufIdx: number }
  | { kind: 'FetchFeedback'; regIdx: number; targetBufIdx: number }
  | { kind: 'ExecuteTransform'; instanceIdx: number; inputBufIdx: number; outputBufIdx: number; paramOffset: number }
  | { kind: 'Sum'; sourceBufIdx: number; targetBufIdx: number }
  | { kind: 'StoreFeedback'; sourceBufIdx: number; regIdx: number }
  | { kind: 'WriteOutput'; sourceBufIdx: number }
