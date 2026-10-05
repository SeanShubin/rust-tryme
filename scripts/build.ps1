$start = Get-Date
Get-Date
& "$PSScriptRoot/_build.ps1"
Get-Date
Write-Host "elapsed: $((Get-Date) - $start)"
