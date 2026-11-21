@echo off
cd /d "%~dp0"
echo Iniciando Xenity Studio...
if exist "xenity-editor\target\release\xenity-editor.exe" (
    start "" "xenity-editor\target\release\xenity-editor.exe"
) else (
    start "" "xenity-editor\target\debug\xenity-editor.exe"
)
