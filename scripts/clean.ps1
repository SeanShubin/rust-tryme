$start = Get-Date
Get-Date
& "$PSScriptRoot/_clean.ps1"
Get-Date
Write-Host "elapsed: $((Get-Date) - $start)"
