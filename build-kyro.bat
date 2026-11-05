@echo off
rem Builds Kyro into Horizon. Close Horizon first. The first build takes 10-30 minutes.
set "LAUNCHER="
for /f "delims=" %%f in ('where horizon-launcher.exe 2^>nul') do if not defined LAUNCHER set "LAUNCHER=%%f"
if not defined LAUNCHER for /f "delims=" %%f in ('dir /b /s "%USERPROFILE%\Downloads\horizon-launcher.exe" 2^>nul') do if not defined LAUNCHER set "LAUNCHER=%%f"
if not defined LAUNCHER (
    echo Could not find horizon-launcher.exe on PATH or in Downloads.
    pause
    exit /b 1
)
"%LAUNCHER%" native-build "%~dp0mixins\kyro" --trust-native-code
echo.
echo Done. If you see an error above, copy the last lines and send them.
pause
