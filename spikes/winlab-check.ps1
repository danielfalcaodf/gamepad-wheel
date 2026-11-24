<#
Phase 0 checks that need Windows but no physical controller (#11): Q4 and the
Q1 mechanism. The ViGEm pad from q4-vigem-pad stands in for the Nova 2 Lite in
XBOX mode, because Windows drives both with the same Xbox 360 driver (xusb22,
device class XnaComposite, USB\VID_045E&PID_028E).

Needs ViGEmBus and HidHide. Run as administrator:
  powershell -ExecutionPolicy Bypass -File winlab-check.ps1 -Bin <folder with the spike .exe files>
Leaves HidHide as it found it: removes only what it added and restores the cloaking state.
#>
param([string]$Bin = $PSScriptRoot)
$ErrorActionPreference = 'Stop'
$tmp = Join-Path $env:TEMP 'gw-winlab'; New-Item -ItemType Directory -Force $tmp | Out-Null

Add-Type -Namespace GW -Name XInput -MemberDefinition @'
[StructLayout(LayoutKind.Sequential)] public struct Vibration { public ushort Left; public ushort Right; }
[DllImport("xinput1_4.dll")] public static extern uint XInputSetState(uint slot, ref Vibration v);
[DllImport("xinput1_4.dll")] public static extern uint XInputGetState(uint slot, byte[] state);
'@

function Run-Probe($exe, $tag, $ms = 1500) {
    $out = Join-Path $tmp "$tag.txt"
    $p = Start-Process $exe -RedirectStandardOutput $out -PassThru -WindowStyle Hidden
    Start-Sleep -Milliseconds $ms; Stop-Process -Id $p.Id -Force; Start-Sleep -Milliseconds 300
    Get-Content $out
}
function Sees-Pad($lines) { [bool]($lines -match 'CONNECTED') }

$win = Get-ItemProperty 'HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion'
"Windows $($win.DisplayVersion) build $([Environment]::OSVersion.Version)"
foreach ($d in 'Virtual Gamepad Emulation Bus', 'HidHide') {
    $v = (Get-CimInstance Win32_PnPSignedDriver | Where-Object DeviceName -match $d | Select-Object -First 1).DriverVersion
    "{0}: {1}" -f $d, ($(if ($v) { $v } else { 'NOT INSTALLED' }))
}

# --- Q4: plug the pad, read it back through XInput, send rumble to it.
$padLog = Join-Path $tmp 'pad.txt'
$pad = Start-Process (Join-Path $Bin 'q4-vigem-pad.exe') -RedirectStandardOutput $padLog -PassThru -WindowStyle Hidden
try {
    Start-Sleep 3
    $slot = 0..3 | Where-Object { [GW.XInput]::XInputGetState($_, (New-Object byte[] 16)) -eq 0 } | Select-Object -First 1
    if ($null -eq $slot) { throw 'Q4: no XInput slot connected after plugging the ViGEm pad' }
    $sweep = Run-Probe (Join-Path $Bin 'q1-xinput-probe.exe') 'q4-probe' 2000
    $changes = ($sweep -match 'buttons=').Count
    foreach ($v in @(@(65535, 0), @(0, 32768), @(0, 0))) {
        $vib = New-Object GW.XInput+Vibration; $vib.Left = $v[0]; $vib.Right = $v[1]
        [GW.XInput]::XInputSetState($slot, [ref]$vib) | Out-Null
        Start-Sleep -Milliseconds 500
    }
    Start-Sleep 1
    $rumble = Get-Content $padLog | Where-Object { $_ -match 'rumble large=255 small=  0' -or $_ -match 'rumble large=  0 small=128' }
    "Q4 pad on XInput slot $slot, $changes state changes read in 2 s"
    "Q4 rumble received by the spike: " + ($rumble.Count -ge 2)
    "Q4 = " + ($(if ($changes -gt 100 -and $rumble.Count -ge 2) { 'YES' } else { 'NO' }))

    # --- Q1 mechanism: hide the pad, then whitelist one probe.
    $cli = Get-ChildItem "$env:ProgramFiles\Nefarius Software Solutions" -Recurse -Filter HidHideCLI.exe |
        Where-Object FullName -match '\\x64\\' | Select-Object -First 1 -ExpandProperty FullName
    $allowed = Join-Path $Bin 'q1-xinput-probe.exe'
    $other = Join-Path $tmp 'probe-not-allowed.exe'; Copy-Item $allowed $other -Force
    $devs = (Get-PnpDevice -PresentOnly | Where-Object InstanceId -match 'VID_045E&PID_028E').InstanceId
    $cloak = (@(& $cli --cloak-state) -join ' ').Trim()  # prints --cloak-on or --cloak-off

    $before = Sees-Pad (Run-Probe $other 'q1-before')
    foreach ($d in $devs) { & $cli --dev-hide $d | Out-Null }
    & $cli --cloak-on | Out-Null
    $hidden = Sees-Pad (Run-Probe $other 'q1-hidden')
    & $cli --app-reg $allowed | Out-Null
    $whitelisted = Sees-Pad (Run-Probe $allowed 'q1-allowed')
    $stillHidden = Sees-Pad (Run-Probe $other 'q1-other')

    "Q1 not hidden, any exe sees the pad:        $before"
    "Q1 hidden, exe not whitelisted sees it:     $hidden"
    "Q1 hidden, whitelisted exe sees it:         $whitelisted"
    "Q1 hidden, another exe still sees it:       $stillHidden"
    "Q1 mechanism = " + ($(if ($before -and -not $hidden -and $whitelisted -and -not $stillHidden) { 'YES' } else { 'NO' }))
}
finally {
    if ($cli) {
        & $cli --app-unreg $allowed | Out-Null
        foreach ($d in $devs) { & $cli --dev-unhide $d | Out-Null }
        if ($cloak -match '^--cloak-(on|off)$') { & $cli $cloak | Out-Null }
    }
    Stop-Process -Id $pad.Id -Force -ErrorAction SilentlyContinue
}
