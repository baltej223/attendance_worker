#!/bin/bash
set -e

# -------- CONFIG --------
BINARY_NAME="attendance_bot"
# ------------------------

# Resolve project directory from THIS script location
PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

RUN_SCRIPT="$PROJECT_DIR/run.sh"
LOG_FILE="$PROJECT_DIR/cron.log"

cd "$PROJECT_DIR"

echo "Building project in $PROJECT_DIR"
cargo build --release

# Create run.sh
cat > "$RUN_SCRIPT" <<EOF
#!/bin/bash
set -e

PROJECT_DIR="\$(cd "\$(dirname "\${BASH_SOURCE[0]}")" && pwd)"
cd "\$PROJECT_DIR"

./target/release/$BINARY_NAME
EOF

chmod +x "$RUN_SCRIPT"

# Cron: start a NEW worker every minute, regardless of others
CRON_JOB="* * * * * $RUN_SCRIPT >> $LOG_FILE 2>&1"

(
  crontab -l 2>/dev/null || true
  echo "$CRON_JOB"
) | crontab -

echo "Setup complete."
echo "A new worker will start every minute (no locking)."
echo "Logs: $LOG_FILE"
