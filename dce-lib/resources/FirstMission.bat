REM do not modify this file
REM The paths are to be modified only in the file Init/path.bat

call Init/path.bat

start "Generate First Mission" cmd /k ""%pathDCS%bin\luae.exe" ..\..\..\ScriptsMod.%versionPackageICM%\BAT_FirstMission.lua"