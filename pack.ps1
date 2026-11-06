# Packs mixins/kyro into dist/kyro-<version>.hnmod: the one file a release needs.
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.IO.Compression
Add-Type -AssemblyName System.IO.Compression.FileSystem

$id = 'kyro'
$version = '2.1.0'
$bundle = Join-Path $PSScriptRoot "mixins\$id"
$dist = Join-Path $PSScriptRoot 'dist'
New-Item -ItemType Directory -Force $dist | Out-Null
$out = Join-Path $dist "$id-$version.hnmod"
if (Test-Path $out) { Remove-Item $out -Force }

$manifest = [ordered]@{
    schema  = 1
    id      = $id
    version = $version
    game    = '26.3'
    horizon = 'c9d474dd9116e84c243f8992d75c11314df56f06'
    mixins  = 'e8cd0aa38cffef49cb2e088ef716baa107148c02'
} | ConvertTo-Json

$zip = [System.IO.Compression.ZipFile]::Open($out, 'Create')
try {
    $entry = $zip.CreateEntry('horizon-native.json')
    $writer = New-Object System.IO.StreamWriter($entry.Open(), (New-Object System.Text.UTF8Encoding($false)))
    $writer.Write($manifest)
    $writer.Dispose()
    Get-ChildItem $bundle -Recurse -File | ForEach-Object {
        $name = $_.FullName.Substring($bundle.Length + 1).Replace('\', '/')
        [System.IO.Compression.ZipFileExtensions]::CreateEntryFromFile($zip, $_.FullName, "mixins/$id/$name") | Out-Null
    }
} finally {
    $zip.Dispose()
}
Write-Host "Packed $out"
