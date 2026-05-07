@echo off
set GCC=C:\msys64\mingw64\bin\gcc.exe

if not exist "%GCC%" (
    echo ERREUR : gcc non trouve : %GCC%
    pause
    exit /b 1
)

echo Compilation msv70_tools...
"%GCC%" -O2 -Wall -fexec-charset=cp1252 msv70_tools.c -o msv70_tools.exe -lcomctl32 -lcomdlg32 -mwindows
if errorlevel 1 ( echo ECHEC & pause & exit /b 1 )

echo.
echo OK - msv70_tools.exe produit
pause
