@echo off
set GCC=C:\msys64\mingw64\bin\gcc.exe

if not exist "%GCC%" (
    echo ERREUR : gcc non trouve : %GCC%
    pause
    exit /b 1
)

echo Compilation 0da_to_bin...
"%GCC%" -O2 -Wall 0da_to_bin.c -o 0da_to_bin.exe
if errorlevel 1 ( echo ECHEC & pause & exit /b 1 )

echo Compilation 0pa_to_bin...
"%GCC%" -O2 -Wall 0pa_to_bin.c -o 0pa_to_bin.exe
if errorlevel 1 ( echo ECHEC & pause & exit /b 1 )

echo Compilation fullbin_to_tune...
"%GCC%" -O2 -Wall fullbin_to_tune.c -o fullbin_to_tune.exe
if errorlevel 1 ( echo ECHEC & pause & exit /b 1 )

echo.
echo OK - 3 executables produits :
for %%f in (*.exe) do echo   %%f
pause
