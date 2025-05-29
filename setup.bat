@echo off
:: RinKokonoe Project Setup Script for Windows
:: This script initializes the RinKokonoe coupon bot project by:
:: - Creating necessary directories
:: - Setting up environment variables
:: - Building the project

echo.
echo === RinKokonoe Coupon Bot Setup ===
echo This script will set up the RinKokonoe coupon bot project.
echo.

:: Check if Rust is installed
where cargo >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo Rust is not installed. Please install Rust first from:
    echo https://www.rust-lang.org/tools/install
    echo.
    pause
    exit /b 1
)

echo [✓] Rust is installed.

:: Create necessary directories
echo Creating necessary directories...
if not exist data mkdir data
if not exist rss mkdir rss
echo [✓] Created data and rss directories.

:: Copy .env.example to .env if it doesn't exist
if not exist .env (
    if exist .env.example (
        copy .env.example .env
        echo [✓] Copied .env.example to .env.
        echo NOTE: Please edit the .env file to configure your Discord token and other settings.
    ) else (
        copy .env .env.example
        echo [✓] Created .env.example from your current .env.
    )
) else (
    echo [✓] .env file already exists.
)

:: Build the project
echo Building the project with cargo...
cargo build --release
if %ERRORLEVEL% neq 0 (
    echo Failed to build the project. Please check the error messages above.
    pause
    exit /b 1
)
echo [✓] Project built successfully.

:: Final instructions
echo.
echo === Setup Complete ===
echo.
echo Next steps:
echo 1. Edit the .env file to set your Discord token or webhook URL
echo    notepad .env
echo.
echo 2. Run the bot with:
echo    cargo run --release
echo.
echo 3. Or use Docker Compose (if Docker is installed):
echo    docker-compose up -d
echo.
echo For more information, please refer to the README.md file.
echo.

pause

