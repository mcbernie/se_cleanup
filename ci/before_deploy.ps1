# This script takes care of packaging the build artifacts that will go in the
# release zipfile

$SRC_DIR = $PWD.Path
$STAGE = [System.Guid]::NewGuid().ToString()

Set-Location $Env:Temp

# install Versions tool
Invoke-WebRequest "https://downloads.vigem.org/other/pavel-a/ddverpatch/verpatch-1.0.15.1-x86-codeplex.zip" -OutFile verpatch-1.0.15.1-x86-codeplex.zip
Expand-Archive verpatch-1.0.15.1-x86-codeplex.zip -DestinationPath $Env:Temp

# set stage folder
New-Item -Type Directory -Name $STAGE
Set-Location $STAGE

$ZIP = "$SRC_DIR\$($Env:CRATE_NAME)-$($Env:APPVEYOR_REPO_TAG_NAME)-$($Env:TARGET).zip"

$VERSION = $Env:APPVEYOR_REPO_TAG_NAME -replace '^v(\d+)\.(\d+)\.(\d+).*$','$1.$2.$3.0'

# 
Invoke-Expression '& "$($Env:Temp)\verpatch.exe" "$SRC_DIR\target\$($Env:TARGET)\release\se_shell.exe" "$VERSION" /va /s description "Star Entertainer Shell" /s company "zwei&40 GmbH" /s copyright "(c) 2018"'
Invoke-Expression '& "$($Env:Temp)\verpatch.exe" "$SRC_DIR\target\$($Env:TARGET)\release\se_shell.exe" /pv "$VERSION"'

# TODO Update this to package the right artifacts
Copy-Item "$SRC_DIR\target\$($Env:TARGET)\release\se_shell.exe" '.\'

7z a "$ZIP" *

Push-AppveyorArtifact "$ZIP"

Remove-Item *.* -Force
Set-Location ..
Remove-Item $STAGE
Set-Location $SRC_DIR