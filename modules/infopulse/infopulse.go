package infopulse

import (
    "rinkokonoe/internal/config"
    "rinkokonoe/internal/database"
    "rinkokonoe/internal/discord"
)

// Module represents the infopulse module
type Module struct {
    name   string
    db     *database.DB
    bot    *discord.Bot
    config *config.Config
}

// New creates a new infopulse module
func New(db *database.DB, bot *discord.Bot, cfg *config.Config) *Module {
    return &Module{
        name:   "infopulse",
        db:     db,
        bot:    bot,
        config: cfg,
    }
}

// Name returns the module name
func (m *Module) Name() string {
    return m.name
}

// Schedule returns the cron schedule
func (m *Module) Schedule() string {
    return "0 */6 * * *"  // Every 6 hours by default
}

// Execute runs the module tasks
func (m *Module) Execute() error {
    return nil
}
