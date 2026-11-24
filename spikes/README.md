# Spikes (phase 0)

Throwaway programs that answer the PRD open questions (`docs/PRD.md`, "Perguntas em aberto"). Each folder is a self-contained crate, outside the `desktop/` workspace, and its README records the answer (ADR 0001).

| Folder | Answers |
|---|---|
| [q1-xinput-probe](q1-xinput-probe/) | Q1 (read the Nova 2 Lite while HidHide hides it) and Q2 (what the back-button macro sends) |
| [q4-vigem-pad](q4-vigem-pad/) | Q4 (ViGEmBus + `vigem-rust` work) and the rumble half of R14 |
| [q5-scancode-keys](q5-scancode-keys/) | Q5 (scan-code keys reach the games) |

Two scripts run the checks and print a YES/NO per question (run as administrator, `-Bin` = folder with the `.exe` files):

| Script | Where | Covers |
|---|---|---|
| [winlab-check.ps1](winlab-check.ps1) | any Windows with ViGEmBus and HidHide, VM included; no controller | Q4, Q1 mechanism (a ViGEm pad stands in for the Nova 2 Lite) |
| [dual-boot-check.ps1](dual-boot-check.ps1) | bare-metal Windows with the Nova 2 Lite in XBOX mode | Q1 with the real dongle, Q2, rumble on the Nova |

Q3 and Q7 are answered without code, and Q6 is deferred to the end of phase 1 (see the PRD).

## Getting the .exe files

CI builds every spike on `windows-latest` (workflow `spikes`, also runnable by hand from the Actions tab):

```sh
gh run download --name spikes-windows
```

There is no installer and no signature, so Windows SmartScreen may ask for confirmation.
