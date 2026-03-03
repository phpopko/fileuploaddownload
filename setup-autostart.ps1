#Requires -Version 5.1
$ErrorActionPreference = "Stop"

$ProjectDir = $PSScriptRoot
$CargoExe   = "$env:USERPROFILE\.cargo\bin\cargo.exe"
$BinPath    = "$ProjectDir\target\release\transfer.exe"
$VbsPath    = "$ProjectDir\launcher.vbs"
$TaskName   = "FileTransferServer"

# Build release binary
Write-Host "Building release binary..." -ForegroundColor Cyan
& $CargoExe build --release --manifest-path "$ProjectDir\Cargo.toml"

if (-not (Test-Path $BinPath)) {
    Write-Error "Build failed - binary not found at $BinPath"
    exit 1
}

# Register scheduled task
Write-Host "Registering startup task..." -ForegroundColor Cyan

$Action = New-ScheduledTaskAction `
    -Execute "wscript.exe" `
    -Argument ('"' + $VbsPath + '"') `
    -WorkingDirectory $ProjectDir

$Trigger = New-ScheduledTaskTrigger -AtLogOn -User $env:USERNAME

$Settings = New-ScheduledTaskSettingsSet `
    -ExecutionTimeLimit 0 `
    -RestartCount 3 `
    -RestartInterval (New-TimeSpan -Minutes 1) `
    -MultipleInstances IgnoreNew

Register-ScheduledTask `
    -TaskName $TaskName `
    -Action   $Action `
    -Trigger  $Trigger `
    -Settings $Settings `
    -RunLevel Limited `
    -Force | Out-Null

# Kill any existing instance and start fresh
Get-Process "transfer" -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue
Start-ScheduledTask -TaskName $TaskName

# Get local IP for display
$IP = (Get-NetIPAddress -AddressFamily IPv4 |
    Where-Object { $_.InterfaceAlias -notlike '*Loopback*' -and $_.InterfaceAlias -notlike '*WSL*' } |
    Select-Object -First 1).IPAddress

Write-Host ""
Write-Host "Done! Server starts automatically on every login." -ForegroundColor Green
Write-Host "Running now -> http://${IP}:5000" -ForegroundColor Green
Write-Host ""
Write-Host "Useful commands:" -ForegroundColor Yellow
Write-Host '  Stop:      Stop-Process -Name transfer -Force' -ForegroundColor Yellow
Write-Host '  Uninstall: Unregister-ScheduledTask -TaskName FileTransferServer -Confirm:$false' -ForegroundColor Yellow
