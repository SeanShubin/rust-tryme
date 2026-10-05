$start = Get-Date
Get-Date
& "$PSScriptRoot/_test.ps1"
Get-Date
Write-Host "elapsed: $((Get-Date) - $start)"
