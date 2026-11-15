# q5-scancode-keys: Q5

Waits a few seconds (time to focus the game), then presses and releases **one** key through `SendInput` with a hardware scan code (`KEYEVENTF_SCANCODE`), which is plan A in the PRD.

```text
q5-scancode-keys <scancode-hex> [delay-s=5] [hold-ms=100]
```

Common Set 1 scan codes: `26` = L, `1a` = `[`, `1b` = `]`, `21` = F, `e04b` = Left arrow, `e04d` = Right arrow (use the `e0` prefix for extended keys).

1. Bind a test action in the game (e.g. ETS2 headlights) to a key, or use its default.
2. Run, for example, `q5-scancode-keys 26 5`, then switch to the game within 5 s.
3. **Expected:** the action fires once.
4. Repeat in each target group: ETS2/ATS, Forza/NFS, Assetto Corsa/BeamNG/F1, GTA.

If the game runs as administrator, run the spike as administrator too, because Windows blocks input from a lower privilege level (UIPI).

## Answer

<!-- Fill in per game: worked yes/no, key used, admin needed. -->
