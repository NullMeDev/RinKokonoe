package main

import (
    "flag"
    "fmt"
    "log"
    "os"
    "os/signal"
    "syscall"

    "rinkokonoe/internal/config"
    "rinkokonoe/internal/database"
    "rinkokonoe/internal/discord"
    "rinkokonoe/modules/infopulse"
    "rinkokonoe/modules/specter"
    "rinkokonoe/modules/validator"
)

func main() {
    // Parse command line flags
    configPath := flag.String("config", ".env", "Path to configuration file")
    flag.Parse()

    // Initialize configuration
    cfg, err := config.Load(*configPath)
    if err != nil {
        log.Fatalf("Failed to load configuration: %v", err)
    }

    // Initialize database
    db, err := database.New(cfg.DatabasePath)
    if err != nil {
        log.Fatalf("Failed to initialize database: %v", err)
    }
    defer db.Close()

    // Initialize Discord bot
    bot, err := discord.New(cfg.DiscordToken)
    if err != nil {
        log.Fatalf("Failed to initialize Discord bot: %v", err)
    }

    // Initialize modules
    infopulseModule := infopulse.New(db, bot, cfg)
    specterModule := specter.New(db, bot, cfg)
    validatorModule := validator.New(db, bot, cfg)

    // Start Discord bot
    if err = bot.Start(); err != nil {
        log.Fatalf("Failed to start Discord bot: %v", err)
    }
    defer bot.Stop()

    // Run initial module tasks
    if err := infopulseModule.Execute(); err != nil {
        log.Printf("Error running infopulse module: %v", err)
    }
    if err := specterModule.Execute(); err != nil {
        log.Printf("Error running specter module: %v", err)
    }
    if err := validatorModule.Execute(); err != nil {
        log.Printf("Error running validator module: %v", err)
    }

    fmt.Println("Bot is now running. Press CTRL+C to exit.")

    // Wait for interrupt signal
    sc := make(chan os.Signal, 1)
    signal.Notify(sc, syscall.SIGINT, syscall.SIGTERM, os.Interrupt)
    <-sc

    fmt.Println("Shutting down...")
}
