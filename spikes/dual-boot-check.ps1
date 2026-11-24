<#
Phase 0 checks that need the real Nova 2 Lite in XBOX mode (#11), on bare-metal
Windows: Q1 with the real dongle, Q2 (does the back-button macro stay held) and
rumble on the Nova. The script asks before each step and saves everything to
dual-boot-results.txt next to the .exe files.

Needs HidHide (winget install Nefarius.HidHide, then reboot). No game needed.
Run as administrator from the folder that holds q1-xinput-probe.exe:
  powershell -ExecutionPolicy Bypass -File .\dual-boot-check.ps1
Restores HidHide afterwards: only the devices and apps this script added are removed,
and cloaking goes back to its previous state.

-Q2Log <file> only analyses a saved probe log (for testing the analysis).
#>
param([string]$Bin = $PSScriptRoot, [string]$Q2Log)
$ErrorActionPreference = 'Stop'
$results = Join-Path $Bin 'dual-boot-results.txt'
$probe = Join-Path $Bin 'q1-xinput-probe.exe'

function Log($m) { $m; Add-Content $results $m -Encoding utf8 }
function Ask($m) { Read-Host "`n>>> $m" }

# Each interval where LB, RB and RS are all down: start and length in ms (-1 = still down at the end).
function Chord-Runs($lines) {
    $on = $null
    foreach ($l in $lines) {
        if ($l -notmatch '^\s*(\d+) ms\s+slot \d: buttons=\S+ \[([^\]]*)\]') { continue }
        $ms = [int64]$Matches[1]; $b = $Matches[2] -split ' '
        $all = ($b -contains 'LB') -and ($b -contains 'RB') -and ($b -contains 'RS')
        if ($all -and $null -eq $on) { $on = $ms }
        elseif (-not $all -and $null -ne $on) { [pscustomobject]@{ start = $on; ms = $ms - $on }; $on = $null }
    }
    if ($null -ne $on) { [pscustomobject]@{ start = $on; ms = -1 } }
}
function Q2-Verdict($lines) {
    $runs = @(Chord-Runs $lines)
    Log ("Q2 chord LB+RB+RS intervals (start ms / length ms): " + (($runs | ForEach-Object { "$($_.start)/$($_.ms)" }) -join ', '))
    $long = @($runs | Where-Object { $_.ms -ge 2000 -or $_.ms -eq -1 })
    $verdict = if ($runs.Count -eq 0) { 'NO CHORD SEEN (macro not set, or the buttons differ)' }
               elseif ($long.Count -ge 1) { 'HELD (plan A works)' }
               else { 'PULSED (plan A fails, use plan B: View as the layer)' }
    Log "Q2 = $verdict"
}

if ($Q2Log) { Q2-Verdict (Get-Content $Q2Log); return }

Add-Type -Namespace GW -Name XInput -MemberDefinition @'
[StructLayout(LayoutKind.Sequential)] public struct Vibration { public ushort Left; public ushort Right; }
[DllImport("xinput1_4.dll")] public static extern uint XInputSetState(uint slot, ref Vibration v);
[DllImport("xinput1_4.dll")] public static extern uint XInputGetState(uint slot, byte[] state);
'@

function Run-Probe($exe, $seconds, $tag) {
    $out = Join-Path $Bin "probe-$tag.txt"
    $p = Start-Process $exe -RedirectStandardOutput $out -PassThru -WindowStyle Hidden
    for ($i = $seconds; $i -gt 0; $i--) { Write-Host -NoNewline "$i.. "; Start-Sleep 1 }
    Write-Host ''
    Stop-Process -Id $p.Id -Force; Start-Sleep -Milliseconds 300
    Get-Content $out
}
function Changes($lines) { ($lines -match 'slot \d: buttons=').Count }

Set-Content $results "dual-boot-check $(Get-Date -Format s)" -Encoding utf8
$win = Get-ItemProperty 'HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion'
Log "Windows $($win.DisplayVersion) build $([Environment]::OSVersion.Version)"

Ask 'Plug the dongle, turn the Nova 2 Lite on in XBOX mode (Home + X, green light). Enter when it is connected' | Out-Null
$pads = Get-PnpDevice -PresentOnly | Where-Object InstanceId -match 'VID_045E&PID_028E'
$virtual = $pads | Where-Object {
    (Get-PnpDeviceProperty -InstanceId $_.InstanceId -KeyName DEVPKEY_Device_Parent).Data -match '^ROOT\\'
}
if ($virtual) { Log 'STOP: a virtual Xbox 360 pad is plugged (close DS4Windows, q4-vigem-pad or similar) and run again.'; return }
if (-not $pads) { Log 'STOP: no Xbox 360 device (045E:028E). Is the Nova in XBOX mode on the dongle?'; return }
Log ("Nova devices: " + ($pads.InstanceId -join '; '))
$slot = 0..3 | Where-Object { [GW.XInput]::XInputGetState($_, (New-Object byte[] 16)) -eq 0 } | Select-Object -First 1
Log "Nova on XInput slot $slot"

# --- Q1 with the real dongle.
$cli = Get-ChildItem "$env:ProgramFiles\Nefarius Software Solutions" -Recurse -Filter HidHideCLI.exe -ErrorAction SilentlyContinue |
    Where-Object FullName -match '\\x64\\' | Select-Object -First 1 -ExpandProperty FullName
if (-not $cli) {
    Log 'Q1 skipped: HidHide is not installed (winget install Nefarius.HidHide, reboot, run again).'
} else {
    $other = Join-Path $Bin 'probe-not-allowed.exe'; Copy-Item $probe $other -Force
    $wasHidden = @(& $cli --dev-list) -join "`n"
    $wasApps = @(& $cli --app-list) -join "`n"
    $wasCloak = (@(& $cli --cloak-state) -join ' ').Trim()
    Log "HidHide before: cloak=$wasCloak"
    $added = @($pads.InstanceId | Where-Object { $wasHidden -notmatch [regex]::Escape($_) })
    try {
        Ask 'Q1 step 1: keep moving the LEFT STICK for 5 s after Enter' | Out-Null
        $c1 = Changes (Run-Probe $other 5 'q1-1-visible')
        foreach ($d in $added) { & $cli --dev-hide $d | Out-Null }
        & $cli --cloak-on | Out-Null
        Ask 'Q1 step 2 (Nova hidden): keep moving the LEFT STICK for 5 s after Enter' | Out-Null
        $c2 = Changes (Run-Probe $other 5 'q1-2-hidden')
        $regApp = $wasApps -notmatch [regex]::Escape($probe)
        if ($regApp) { & $cli --app-reg $probe | Out-Null }
        Ask 'Q1 step 3 (hidden, probe whitelisted): keep moving the LEFT STICK for 5 s after Enter' | Out-Null
        $c3 = Changes (Run-Probe $probe 5 'q1-3-whitelisted')
        Log "Q1 state changes: visible=$c1 hidden=$c2 whitelisted=$c3"
        $q1 = if ($c1 -gt 10 -and $c2 -eq 0 -and $c3 -gt 10) { 'YES' } else { 'NO (see probe-q1-*.txt)' }
        Log "Q1 = $q1"
        $joy = Ask 'While hidden: open joy.cpl (Win+R, joy.cpl). Is the Nova listed? (y/n)'
        Log "Q1 joy.cpl lists the hidden Nova: $joy"
    }
    finally {
        if ($regApp) { & $cli --app-unreg $probe | Out-Null }
        foreach ($d in $added) { & $cli --dev-unhide $d | Out-Null }
        if ($wasCloak -match '^--cloak-(on|off)$') { & $cli $wasCloak | Out-Null }  # prints --cloak-on/--cloak-off
        Log "HidHide restored: cloak=$((@(& $cli --cloak-state) -join ' '))"
    }
}

# --- Q2: the back-button macro.
Ask @'
Q2 setup: in GameSir Connect, set BOTH back buttons (L4 and R4) to a macro that presses
LB + RB + R3 (right stick click) together, save it to the controller, then close the app.
Enter when done
'@ | Out-Null
Ask 'Q2: after Enter you have 15 s. TAP L4 once, wait, HOLD L4 for 3 s, release, then HOLD R4 for 3 s' | Out-Null
$q2 = Run-Probe $probe 15 'q2'
Q2-Verdict $q2
$fw = Ask 'Q2: firmware version shown in GameSir Connect (Enter to skip)'
Log "Nova firmware: $fw"

# --- Rumble on the Nova (R7/R14 output side).
Ask 'Rumble: after Enter the Nova vibrates strong for 1 s, then weak for 1 s' | Out-Null
foreach ($v in @(@(65535, 0), @(0, 40000), @(0, 0))) {
    $vib = New-Object GW.XInput+Vibration; $vib.Left = $v[0]; $vib.Right = $v[1]
    [GW.XInput]::XInputSetState($slot, [ref]$vib) | Out-Null
    Start-Sleep 1
}
$felt = Ask 'Did you feel both vibrations? (y/n)'
Log "Rumble felt on the Nova: $felt"

Log "Done. Probe logs: $Bin\probe-*.txt"
