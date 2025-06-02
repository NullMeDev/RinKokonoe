package discord

import (
    "fmt"
    "log"
    "strings"

    "github.com/bwmarrin/discordgo"
)

// Bot represents the Discord bot
type Bot struct {
    session      *discordgo.Session
    commands     map[string]CommandHandler
    commandDescs map[string]string
}

// CommandHandler is a function that handles a Discord command
type CommandHandler func(s *discordgo.Session, m *discordgo.MessageCreate, args []string) error

// New creates a new Discord bot
func New(token string) (*Bot, error) {
    session, err := discordgo.New("Bot " + token)
    if err != nil {
        return nil, fmt.Errorf("failed to create Discord session: %w", err)
    }

    bot := &Bot{
        session:      session,
        commands:     make(map[string]CommandHandler),
        commandDescs: make(map[string]string),
    }

    // Register message handler
    session.AddHandler(bot.messageCreate)

    // Register basic commands
    bot.RegisterCommand("help", "Display available commands", bot.helpCommand)
    bot.RegisterCommand("ping", "Check if bot is alive", bot.pingCommand)

    return bot, nil
}

// Start starts the Discord bot
func (b *Bot) Start() error {
    return b.session.Open()
}

// Stop stops the Discord bot
func (b *Bot) Stop() error {
    return b.session.Close()
}

// RegisterCommand registers a new command
func (b *Bot) RegisterCommand(name, description string, handler CommandHandler) {
    name = strings.ToLower(name)
    b.commands[name] = handler
    b.commandDescs[name] = description
}

// SendMessage sends a message to a channel
func (b *Bot) SendMessage(channelID, content string) (*discordgo.Message, error) {
    return b.session.ChannelMessageSend(channelID, content)
}

// messageCreate handles incoming messages
func (b *Bot) messageCreate(s *discordgo.Session, m *discordgo.MessageCreate) {
    // Ignore messages from the bot itself
    if m.Author.ID == s.State.User.ID {
        return
    }

    // Check if message starts with "!" for commands
    if !strings.HasPrefix(m.Content, "!") {
        return
    }

    // Parse command and arguments
    parts := strings.Fields(m.Content[1:])
    if len(parts) == 0 {
        return
    }

    cmd := strings.ToLower(parts[0])
    args := parts[1:]

    // Execute command if registered
    if handler, ok := b.commands[cmd]; ok {
        if err := handler(s, m, args); err != nil {
            log.Printf("Error executing command %s: %v", cmd, err)
            s.ChannelMessageSend(m.ChannelID, fmt.Sprintf("Error: %v", err))
        }
    }
}

// helpCommand handles the help command
func (b *Bot) helpCommand(s *discordgo.Session, m *discordgo.MessageCreate, args []string) error {
    var sb strings.Builder
    sb.WriteString("**Available Commands:**\n")

    for cmd, desc := range b.commandDescs {
        sb.WriteString(fmt.Sprintf("• `!%s` - %s\n", cmd, desc))
    }

    _, err := s.ChannelMessageSend(m.ChannelID, sb.String())
    return err
}

// pingCommand handles the ping command
func (b *Bot) pingCommand(s *discordgo.Session, m *discordgo.MessageCreate, args []string) error {
    _, err := s.ChannelMessageSend(m.ChannelID, "Pong!")
    return err
}
