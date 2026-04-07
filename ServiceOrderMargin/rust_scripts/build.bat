@echo off
echo Building GL Tie-Out Rust Project...
echo.

cd /d "%~dp0"

if not exist "data" (
    echo ERROR: data directory not found!
    echo Please ensure the data directory with CSV files exists.
    pause
    exit /b 1
)

cargo build --release

if %ERRORLEVEL% EQU 0 (
    echo.
    echo Build successful!
    echo The executable will be at: target\release\gl_tie_out.exe
    echo.
    echo To run the program:
    echo   target\release\gl_tie_out.exe
) else (
    echo.
    echo Build failed. Please check the error messages above.
)

pause
