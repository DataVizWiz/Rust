$ParentDir = "."

# Get all directories containing a Cargo.toml file
$projectDirs = Get-ChildItem -Path $ParentDir -Recurse -Filter Cargo.toml | Select-Object -ExpandProperty DirectoryName

foreach ($dir in $projectDirs) {
    Write-Host "Cleaning $dir"
    Set-Location $dir
    cargo clean
}

Write-Host "All projects cleaned."
