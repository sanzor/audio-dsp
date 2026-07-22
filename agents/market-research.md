# Market Research

Findings from `business-analyst` runs, tracking how well the product bet in `agents/mission.md` holds up against evidence. This is a living doc — append new findings with a date rather than silently overwriting old ones, so contradictions are visible instead of erased.

## Round 1 — initial segments/competitors/TAM pass

Framing at the time: two distinct populations (creators who write code, editors who don't).

### Segments considered

| Segment | Surface | Fit |
|---|---|---|
| Bedroom/home producers | Editor only | Largest by volume, price-sensitive, already served by BandLab/Soundtrap |
| Educators/students | Editor only | Where web-DAWs have *actually* won commercially (Soundtrap is Spotify-owned, ed-focused) |
| Mixing/mastering engineers | Editor only | High trust bar — open question whether they'd trust browser WASM for paid client work |
| Sound designers (games/film) | Weak fit either way | Dominated by Wwise/FMOD middleware instead |
| Plugin/DSP developers | Creator only | Already monetize on the whole desktop market via KVR/Plugin Boutique — unclear incentive to author for one platform's smaller catalog |
| Coder-musicians (Max4Live/Reaktor/Faust) | Both, same person | Closest analogue found — but as one person, not a two-sided market |

### Competitive landscape

| Competitor | Type | User-authored custom DSP → shared catalog? | Notes |
|---|---|---|---|
| Ableton Live / FL Studio / Logic / Reaper | Desktop incumbent | No — fixed VST/AU format, authored by third parties outside the DAW | $60–$749 one-time |
| BandLab | Web DAW | No — fixed effects | Freemium, huge casual/mobile-first base |
| Soundtrap (Spotify) | Web DAW | No | Subscription, go-to-market is largely education |
| Soundation | Web DAW | No, but already runs Web Audio API + WASM DSP + Audio Worklets internally, just not exposed to users | $9.99–$49.99/mo, free for students |
| **Audiotool (NEXUS, open beta Jan 2026)** | Web DAW | **Yes — closest direct competitor.** "Every user can potentially become a developer of their own DAW extensions," free, open API into routing/signal processing | Targets "producers for whom music and technology is second nature" |
| openDAW (same creator as Audiotool) | Web DAW | Partial — TypeScript "Modular System," education-focused, no clear publish-to-marketplace flow found | Free/open-source, no monetization model found |
| Faust Web IDE + Web Audio Modules (WAM) | Dev toolchain, not a SaaS product | Yes, technically — Faust compiles DSP to WASM in-browser, exportable as a WAM plugin; open community index | Dev-facing, no consumer-facing catalog or non-technical editor UX |
| Splice | Not a DAW | N/A — sample/loop marketplace + rent-to-own plugins; own browser "Studio" DAW effort de-emphasized | Credit-based subscription |

**Key finding:** Audiotool/NEXUS is already building close to this exact bet, live, with a real community. Everyone else in web-DAW space offers only a fixed effects library.

### TAM — low confidence throughout

No clean number found for "web DAW with user-extensible effects" specifically. Proxies:
- Global audio-plugin market: one estimate $2.1B (2024)→$5.8B (2033), another $4.34B→$4.94B (2023–24) — sources disagree ~2x; treat the qualitative signal (multi-billion, double-digit CAGR) as more trustworthy than any digit.
- "~2M active music producers/sound engineers/composers globally, 63% home-studio" — sourced from unmethodical market-report-mill sites, very low confidence, order-of-magnitude gut check only.
- Splice user/revenue figures are themselves 2021 extrapolations, not current numbers.

**Reasoned inference:** a market for "browser DAW" (proven — BandLab/Soundtrap/Soundation/Audiotool all sustain real user bases) and for "custom effects" (proven — plugin economy is real and large) both plausibly exist. The specific intersection — people willing to author *for strangers via a platform catalog* — has no number anywhere.

### Open questions from round 1

- Would a mixing/mastering engineer trust in-browser WASM processing for paid client work, or does browser-DAW usage ceiling at hobbyist/casual regardless of audio quality?
- Given Audiotool NEXUS already exists with the same bet and a live community, why would a DSP author choose this platform's catalog over NEXUS, WAM, or a standalone VST?
- Among people who already write custom DSP (Faust/Max4Live/Reaktor users), what fraction would do so *for other people's use* rather than their own — this number validates or kills a strict two-population split.
- Is the real early-adopter wedge education (where Soundtrap/Soundation/BandLab already win), not "creator" or "professional editor" as originally framed?

## Round 1.5 — reframe: marketplace + personal experimentation tool, not strict two-population split

`agents/mission.md` was updated to reflect a sharper framing the user gave directly, ahead of new research: the product is two-fold — (a) a **marketplace** for transform creators (nodes usable by anyone, authored by you or others), and (b) a **hands-on experimentation tool** where a single editor user applies transforms (destructively or non-destructively) over track regions with live preview, using only their own published nodes if they want, no marketplace participation required.

This directly addresses round 1's biggest open risk: the product no longer depends on two distinct populations transacting — a single user being both creator and editor is an expected, valid usage pattern, not a failure of the premise. What's now open: does the *marketplace* half (other people's nodes having value to strangers) still have a real market beyond the same-person pattern, and does "non-destructive DAG editing with instant preview" as the core editor experience change who the realistic segments/competitors are (e.g. Bitwig's Grid, node-based tools like Node-RED, non-destructive editing conventions in Ableton's own device chains).

Next research pass should re-run segments/TAM/competitors against this sharper framing rather than the original strict-split one.

## Round 2 — marketplace + experimentation framing

### 1. Does non-destructive, region-scoped, node editing change the segments?

| Analogue | What it is | Non-destructive? | Region-scoped (partial-track)? | Who actually uses it |
|---|---|---|---|---|
| Bitwig "The Grid" | Modular node device inside a commercial DAW | Yes (patch stays live) | No — applies at device/track level, not to an arbitrary sub-span of a clip | Sourced material calls it "a significant reason producers choose Bitwig" — reads as a differentiator that draws power users/sound designers in, not a mainstream feature most owners touch daily. No usage-share number exists (confirmed absent). Confidence: medium, single-source characterization. |
| Ableton Audio Effect Racks / Macros | Non-destructive chain-building, parallel processing, macro-mapped | Yes | No — rack-level, not sub-clip | Ubiquitous in tutorials/marketing; no published %-of-users data exists anywhere (confirmed absent). Reasoned inference only: common among intermediate+ users, not proven. |
| **VCV Rack** (new this round) | Free/open modular-synth patcher, literal node-cable graph | Yes, inherently | N/A (instrument-level) | Real, sustained third-party marketplace (VCV Library): 2,000+ modules, free and paid mixed, independent devs (Vult — 39 paid modules at $25; Instruō ported ~17 free modules from hardware). **Closest structural analogue found to "node marketplace + non-destructive patching."** Confidence: medium-high on scale, low on creator/consumer ratio (unpublished). |
| Node-RED | Node-based flow marketplace, general (not audio) | Yes | N/A | 5,000+ community nodes, 5,700+ shared flows. No contributor-vs-consumer ratio published anywhere. |

**Bottom line:** node-based non-destructive editing is a recognized draw for a *specific* segment (modular-synth-minded producers, sound designers), not a universal upgrade over linear effect chains — every analogue sits alongside, not instead of, a simpler default workflow. **Region-scoping — applying a graph to an arbitrary highlighted span inside a clip, not just a whole track/device — was not found in any competitor**, not Bitwig Grid, not Ableton Racks, not Audiotool. Possibly genuine structural differentiation, possibly just under-documented elsewhere; absence of evidence at web-search depth isn't proof of uniqueness.

### 2. Does the marketplace half have evidence independent of same-person usage?

Strongest hard number found across both rounds: Native Instruments' Reaktor User Library reported the **"VHS Audio Degradation Suite" ensemble was downloaded over 11,800 times in 2018 alone**, out of a library of ~4,000 total ensembles — one author's patch, used by thousands of strangers, in a single year. This directly answers Round 1's open question with a concrete figure. Confidence: medium-high (single NI blog post, and a top-decile outlier, not a median).

Maxforlive.com (10,000+ devices) and VCV Library (2,000+ modules, real paid transactions) corroborate the *qualitative* pattern at real scale, but neither publishes creator-count vs. downloader-count — the ratio itself stays unknown across every analogue checked. Splice's Rent-to-Own program is a weaker analogue than it first looks: professional vendors (Xfer/Serum, Spitfire) renting existing commercial products, not amateur creators getting picked up by strangers — don't lean on it here.

### 3. Sharper segments

Reasoned inference, not evidenced: this framing plausibly broadens the addressable segment beyond Round 1's "artists who don't code" to people who already think in patch/node terms and want fast, reversible iteration — Eurorack/VCV-adjacent producers, sound designers/installation artists already using Max/MSP or TouchDesigner specifically for quick non-destructive exploration. Plausible, not validated.

### 4. TAM

No new number materializes. Bitwig has no clean market-share digit (one source: Pro Tools 37.2% of professional studios, Ableton/FL/Logic ~58% combined, Bitwig/Reaper/Studio One left as unquantified "niche"). No %-of-Ableton-users-on-Max4Live/Racks figure exists anywhere searched. VCV Library's 2,000+ module count is the best available proxy for "how big a non-destructive node marketplace can get" — but it's a supply-side proxy, not demand. Same low-confidence caveat as Round 1 stands.

### 5. What's genuinely unknown (this framing specifically)

- Whether region-scoped graph application (arbitrary sub-span of a track) exists in any competitor at all — not found; could be real differentiation or just under-documented.
- Whether Audiotool NEXUS's patching is destructive or non-destructive — could not confirm from public docs/press despite direct attempts. Needs hands-on testing or asking Audiotool directly, not more search.
- The actual creator-to-consumer ratio in any node/patch marketplace (Reaktor, Max4Live, VCV) — no source publishes it.
- Whether "consumer-only" participants (use others' nodes, never publish) are a durable population, or whether everyone drifts toward publishing eventually — no data either way.
