# The FX Control page

The plugin registers its own page, titled **FX Control**, in
[Online Deluxe](https://git.syntaxbyexample.com/intpa/ssbu-online-deluxe-rewrite)'s
window. Deluxe is optional: without the page, the same settings live in the
[INI](CONFIGURATION.md). The page edits the same table the INI loads.

## Host versions

The page uses plugin descriptor version 4 (SDK pinned to Deluxe 0.5.4,
`a63bc8c`).

- **Deluxe 0.5.3 or later** shows the page.
- **Deluxe 0.5.0 to 0.5.2** give the online signal but refuse the version 4
  registration: there is no page, and effects still follow the INI.
- **Deluxe older than 0.5.0**, or a legacy SsbuSync host: no page and no online
  signal, so Scope Online never applies
  ([scope and online detection](CONFIGURATION.md#scope-and-online-detection)).
- **Deluxe 0.5.6 or later** changes a value only while **ZR** is held and **ZL** released, in every
  scene; a plain Left/Right is ignored.
- **Deluxe 0.6.0 or later** has the Window Options page, whose OwnPages row can
  take plugin pages such as FX Control out of the L+R+Down cycle.
- The [media](MEDIA.md) were captured with the released Deluxe 0.7.0.

Page order, the window chords and the in-match strip belong to Deluxe; its
[Controls](https://git.syntaxbyexample.com/intpa/ssbu-online-deluxe-rewrite/src/branch/main/docs/CONTROLS.md)
document is the reference for the host you run.

## Editing

1. Hold **L+R** and tap **D-pad Down** until **FX Control** is shown.
2. **Up/Down** selects a row. Group headers are skipped.
3. Hold **ZR**, release **ZL** and press **Left/Right** to change the selected row.
4. Changes apply to new effect requests at once and are saved to `config.ini`
   ([who writes the file](CONFIGURATION.md#who-writes-the-file)).

## Rows

The page has 33 rows:

| Row | Choices | Notes |
| --- | --- | --- |
| Scope | Online / All | Online: online play outside Training. Deluxe 0.5.0 to 0.5.2 gives the online signal but refuses the page, so the INI alone sets the scope. With a Deluxe older than 0.5.0 there is no online signal either: Online never applies (effects stay original until `scope=all` is set in the INI). |
| All | On / Off / Reduced | Sets the ten fighter rows. Shows `Unavailable` while the rows differ. Reduced on a row without a variant sets that row On ([All](FX.md#all)). |
| Captain Falcon, Ness, Lucas, Palutena, Sephiroth, Pyra, Mythra, Byleth, Inkling, Kirby | On / Off / Reduced | One row per fighter with a move on the ledger (`src/moves.rs`), in this order. On is green, Reduced yellow. Inkling offers On / Off only (the roller has no variant). Kazuya, Ike, Chrom, Roy and Ganondorf are not covered: no row, requests pass through untouched. |
| Group headers | — | One header per fighter with variants; not selectable, Up/Down skip it. |
| Falcon Punch, PK Thunder, PK Fire, … Hammer | `Original`, `Variant 1` .. `Variant N` | One value row per move under its fighter's header, showing the selected variant (the `variant.<move>` value); Left/Right change it in place. It takes effect at once while the fighter's row is Reduced; Original leaves that move untouched. Needs Deluxe 0.5.3 (plugin API v4). |

The moves and what each variant removes are listed in
[Moves and variants](FX.md#moves-and-variants).
