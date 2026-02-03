#!/bin/bash
set -e

# -------- CONFIG --------
PROJECT_DIR="$HOME/attendance_bot"
BINARY_NAME="attendance_bot"
RUN_SCRIPT="$PROJECT_DIR/run.sh"
LOG_FILE="$PROJECT_DIR/cron.log"
LOCK_FILE="/tmp/attendance_bot.lock"
# ------------------------

cd "$PROJECT_DIR"
cargo build --release

cat > "$RUN_SCRIPT" <<EOF
#!/bin/bash
set -e
cd "$PROJECT_DIR"
./target/release/$BINARY_NAME
EOF

chmod +x "$RUN_SCRIPT"

CRON_JOB="* * * * * flock -n $LOCK_FILE $RUN_SCRIPT >> $LOG_FILE 2>&1"

(
  crontab -l 2>/dev/null || true
  echo "$CRON_JOB"
) | crontab -

echo "Setup complete."
echo "Binary will run every minute (first run in ~60 seconds)."
echo "Logs: $LOG_FILE"
