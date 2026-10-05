$start = Get-Date
Get-Date
& "$PSScriptRoot/_doc.ps1"
Get-Date
Write-Host "elapsed: $((Get-Date) - $start)"
