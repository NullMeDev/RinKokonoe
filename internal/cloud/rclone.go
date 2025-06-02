package cloud

import (
    "fmt"
    "os"
    "os/exec"
    "path/filepath"
    "time"
)

// RcloneSync handles syncing data to Google Drive using rclone
type RcloneSync struct {
    drivePath   string
    localPath   string
    logFile     string
    concurrency int
}

// NewRcloneSync creates a new RcloneSync instance
func NewRcloneSync(drivePath, localPath, logDir string) *RcloneSync {
    return &RcloneSync{
        drivePath:   drivePath,
        localPath:   localPath,
        logFile:     filepath.Join(logDir, "rclone.log"),
        concurrency: 2,
    }
}

// SyncToGDrive syncs local files to Google Drive
func (r *RcloneSync) SyncToGDrive() error {
    // Ensure local directory exists
    if err := os.MkdirAll(r.localPath, 0755); err != nil {
        return fmt.Errorf("failed to create local directory: %w", err)
    }

    // Run rclone sync command
    cmd := exec.Command(
        "rclone",
        "sync",
        r.localPath,
        r.drivePath,
        "--transfers", fmt.Sprintf("%d", r.concurrency),
        "-v",
        "--log-file", r.logFile,
    )

    output, err := cmd.CombinedOutput()
    if err != nil {
        return fmt.Errorf("rclone sync failed: %w\nOutput: %s", err, string(output))
    }

    return nil
}

// SaveReport saves a report to local storage and syncs to GDrive
func (r *RcloneSync) SaveReport(report []byte, reportType string) error {
    // Create filename with timestamp
    timestamp := time.Now().Format("2006-01-02_15-04-05")
    filename := fmt.Sprintf("%s_%s.json", reportType, timestamp)

    // Create directory for report type
    reportDir := filepath.Join(r.localPath, reportType)
    if err := os.MkdirAll(reportDir, 0755); err != nil {
        return fmt.Errorf("failed to create report directory: %w", err)
    }

    // Write report to file
    reportPath := filepath.Join(reportDir, filename)
    if err := os.WriteFile(reportPath, report, 0644); err != nil {
        return fmt.Errorf("failed to write report: %w", err)
    }

    // Sync to GDrive
    return r.SyncToGDrive()
}
