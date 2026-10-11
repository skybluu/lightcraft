# UI parity

> **Last reviewed:** 2026-10-11 · **Last updated:** 2026-10-11 · **Change:** minor (budgeted native reuse, neighbour prefetch and scrub zoom; Compare/Reference retained) · **Target:** Adobe Lightroom Classic 15.5 / Lightroom 9.6

How LightCraft's interaction compares with Lightroom's: views, panels, tools and handles, modifier
keys, shortcuts, zoom, context menus and feel. Rows for individual menu items and shortcuts live in
[parity-checklist.md](parity-checklist.md) (sections Y and Z, Classic keys); this page is the
assessment and the notes that don't fit a row. Visual reference for agents: `plan/lightroom/`
(local, gitignored; `10-observed-ui.md` and screenshots).

## Measured counts (2026-10-10)

- 226 engine commands (`crates/engine/src/cmd/`, 30 modules) and 143 UI commands (`UI_COMMANDS` in
  `crates/ui-egui/src/menus.rs`, 73 with a shortcut) plus 10 language commands; 23 secondary
  shortcut bindings (`ALIASES`, `crates/ui-egui/src/shortcuts.rs`); a keymap editor.
- 122 develop controls plus 10 indexed templates (`crates/develop/src/controls.rs`).
- Checklist: menus 95.3% (86 in-scope rows), desktop shortcuts 94.4% (81 rows), Classic extras
  48.2% (82 rows), measured by recounting [parity-checklist.md](parity-checklist.md).
- 31 panel modules (`crates/ui-egui/src/panels/`), 26 context-menu sites.

## Assessment by area

| Area | Target | LightCraft | Ready | Main gaps |
|---|---|---|---:|---|
| Layout and views | Lightroom: grid / detail / edit; Classic: Library, Develop, Map, Book, Slideshow, Print, Web modules, filmstrip, panels | Lightroom-style single window: Photo Grid, Square Grid, Detail, Compare, Survey (≤ 48), Before / After, People, quick slideshow, second window, filmstrip | 75% | no Classic module switcher (by design) or output modules; dark theme only |
| Develop panels and sliders | Light, Color, Effects, Detail, Optics, Geometry, Lens Blur, Calibration | every global slider; scrub, double-click reset, Alt-drag previews where implemented | 80% | Lens Blur, Calibration panel depth; slider feel (acceleration, fine Shift-drag) unmeasured |
| Crop, Upright, transform handles | handles, aspect lock, angle, overlays (O / ⇧O), guided Upright | ✅ all of them; overlay cycle on ⇧O | 85% | crop opacity of cut-out area (15.5) |
| Masking tools | brush (A / B, flow, density, auto mask, pressure), linear / radial, range masks, AI masks, pins | brush, linear, radial, colour / luminance range, add / subtract / intersect / invert, masks panel, pins | 60% | brush live preview (#517 / #518), no pen pressure, no depth range, AI masks, Feather / Edge controls (15.5) |
| Remove / heal | content-aware remove, heal, clone, visualize spots | heal, clone, auto source, visualize spots, red / pet eye | 65% | content-aware (patch synthesis) remove, generative remove, detect distractions |
| Zoom and navigation | Fit / Fill / 1:1 / 2:1 … 11:1, Navigator, spacebar pan, Shift-drag scrub | Fit to 800%, pinch and wheel zoom, pan, Navigator-like, magnifier and anchored scrub; byte-budgeted native reuse and qualified neighbour warming | 60% | cold decoding and low-budget eviction; original-pixel detail with soft proofing (no zoom window there; LR-VIEW-ZOOM) |
| Library culling and metadata | ratings, flags, labels with keys, painter, keywording, metadata panel, filter bar | ratings / flags / labels and advance, keyword list and sets, keywording box, metadata fields, smart albums, filter bar | 85% | painter tool, quick collection, some Classic keys |
| Shortcuts | Lightroom desktop and Classic keymaps | Lightroom-desktop keymap with Classic additions, editable | 85% | deliberate differences below; no Classic keymap layer |
| Context menus | everywhere | 26 sites (grid, filmstrip, albums, folders, keywords, masks, presets…) | 70% | unaudited against Lightroom's menus item by item |
| Text fields | native editing behaviour | custom text field (LR-BEHAV-TEXTFIELD) | 70% | IME and selection edge cases |
| Accessibility | VoiceOver / Narrator, keyboard navigation | AccessKit: sliders, buttons, dropdowns, section headers announce | 30% | canvas tools pointer-only; not audited with a screen reader |
| Feel and responsiveness | GPU-interactive sliders (15.3) | ≤ 16 ms slider budget on GPU; background renders | 75% | colour NR at half resolution, 100k-photo grid test |
| **UI/UX overall** | | | **~70%** | estimated, weighted by use (develop and library 60%, tools 25%, rest 15%) |

Remaining: **40–70 h** (zoom detail 8–15 h, brush and masking feel 8–12 h, accessibility audit 10–20
h, Classic keymap layer 4–8 h, context-menu audit 4–8 h, text-field and IME polish 4–8 h). Output
modules and AI tools are counted with their features in [target-app-parity.md](target-app-parity.md).

## Shortcuts

Compared `plan/lightroom/06-shortcuts.md` (desktop part) with our bindings: command specs in `crates/engine/src/cmd/`,
`UI_COMMANDS` in `crates/ui-egui/src/menus.rs` and secondary bindings (`ALIASES`) in `crates/ui-egui/src/shortcuts.rs`.
`no_conflicting_bindings` (same file) fails when one key fires two actions.

**Fixed (M16.1):** added Lightroom-desktop keys as secondary bindings for existing commands — ⌘D Select None, ⇧E
Export dialog, Space Toggle zoom, ⇧M Create Version, ⇧X Reject + advance, ⇧U Unflag + advance. `⇧Y` fired both
Before/After Split and the History panel; History lost the binding. `W` and `⇧⌘I` also fired the engine command
under the UI command that wraps it (a no-op error); the UI command now wins.

**Deliberate differences (our key → Lightroom desktop key)** — each is a conflict with another binding we have:

| Action | Ours | Lightroom desktop | Why |
|---|---|---|---|
| Pick flag | P | Z | Z = toggle zoom (Classic convention); P is the Classic pick key |
| Photos panel (left) | ⌘⇧L | P | P = pick |
| Expand/collapse edit sections | ⌘⌥1–5 | ⌘1–6 | ⌘0 = Zoom to Fit, ⌘1 = Zoom 100 %; Geometry lives in the Crop panel |
| Histogram | ⌘⇧H | ⌘0 | ⌘0 = Zoom to Fit |
| Square Grid | ⇧G | (none; ⇧G = Guided Upright) | Guided Upright is a button in the Crop panel |
| Export dialog | ⌘⇧E (+ ⇧E) | ⇧E | ⌘⇧E is "Edit in Photoshop" there; no external-editor command yet |
| Crop overlay cycle | ⇧O | O | O = mask overlay; ⇧O (mask colour / overlay orientation) unused otherwise |
| Create Version | ⌘⇧S (+ ⇧M) | ⇧M (Windows: Ctrl+⇧S) | — |
| Select None | ⌘⇧A (+ ⌘D) | ⌘D | — |

**Library culling (KEYC-RATING):** `0–5`, `6–9` and `P/X/U` reach egui on macOS even when displayed in the native menu (adapted from PR #261; issue #283). `Shift+0–9` applies and advances once; `Shift+P` picks and advances in Photo Grid / Square Grid and retains Presets elsewhere. See [library shortcuts](library-shortcuts.md) for the verified Classic mapping and regression coverage. Remaining Classic gaps: rating `[` / `]`, flag cycling and filter-bar keys.

**Still missing / broken:**
- No command yet: F1 help (verify the rest of the old list: full screen, settings, stacks,
  visualize spots and merges have commands now).
- `H` opens Remove; Lightroom also uses it (Classic) to hide pins — pins toggle from View → Show Mask Pins.
- ⌘M / ⌘H / ⌘Q / ⌘W rely on the platform window defaults (unverified).


## RAW browsing and scrub zoom verification (2026-10-11)

The cache and input changes are ported onto upstream `76b5377868d912db69a777bda253a5b1b96b22bd`,
not the older 0.6.0 release source. Current upstream still caps Detail at one decoded original
and prefetches preview sources only; its two-source Compare/Reference cache is not a browsing cache.
The port retains originals under the shared byte budget, preserves decoder facts and denoised twins,
and protects the foreground while accepting late neighbour completions. Compare and Reference
retain both native sources when they fit; leaving those views no longer discards reusable originals.

`tool.zoom` selects the bottom-bar magnifier. Horizontal press-drag zoom holds the image point at
the press position; right zooms in, left zooms out. Shift-drag works with no editing tool selected;
ordinary dragging still pans, Space-drag temporarily pans with the magnifier selected, and click
still toggles Fit / the chosen ratio. Crop, masking, remove and colour-picking gestures retain input.
The current upstream wheel/pinch behavior and Before/After pane clipping are retained.

Native Windows smoke used the same three Sony A7R IV 9504 × 6336 RAWs, NVIDIA RTX 5080 DX12,
an 8192 MiB budget, 100% zoom and 5 seconds idle between selections. Fit browsing performed zero
full decodes. At 1536 MiB only one original fit and all eight selections decoded; cold loading
and limited-budget eviction remain expensive. This is not a Lightroom-parity or cold-load claim.

| Current-main comparison, five revisits | Unmodified upstream | Ported fork |
|---|---:|---:|
| Median selection to settled native window | 2622 ms | 412 ms |
| Full RAW developments on revisits | 5 | 0 |
| Retained originals after the sequence | 1 | 3 |

Compare and Reference smoke retained both native originals and performed zero additional full
decodes during panning in each view. These measurements cover cached browsing after neighbour
preparation, not uninterrupted rapid scrolling or uncached first loading. The first ported
selection in this run took 7065 ms; cold performance is not improved by this patch.

Verification: 21 engine media tests, 3 thumbnail/content-key tests, 11 zoom/input tests, 2 Compare
interaction tests and 46 preview/window tests passed (1 profiling test ignored). Native Windows
smoke covered Detail browsing, Fit, a 1536 MiB budget, Compare/Reference panning and magnifier,
Shift-drag, ordinary pan and held-Space pan; the magnifier was visually inspected in the toolbar.
Formatting, parity-reference validation, dependency layers and asset attribution checks passed.
The prescribed `cargo xtask ci` stops at clippy with unchanged upstream findings:
`crates/pipeline/src/spots.rs:250` (`chunks_exact_to_as_chunks`) and the Windows test-only unused
`descriptor_copy` in `crates/catalog/src/lock.rs:140`. No unrelated lint suppressions were added.
This is not a full-CI-green claim; the full workspace suite and WASM gate were not reached.
Current upstream upgrades catalog format v6 to v7; the port was tested against a backed-up copy
of a 148-photo catalog, leaving the original user library unmodified during verification.



## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-10-11 | minor | Ported shared-budget native caching and scrub zoom onto current upstream; retained Compare/Reference, wheel navigation and pane clipping |
| 2026-10-11 | minor | Zoom row: Compare and Reference get zoom windows; what remains of LR-VIEW-ZOOM is overlays and soft proofing |
| 2026-10-10 | major | Created: measured command and shortcut counts, assessment by area, estimates; moved the shortcut conflicts section here from the checklist |
