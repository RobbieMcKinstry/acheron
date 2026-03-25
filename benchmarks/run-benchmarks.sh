#!/usr/bin/env bash
set -euo pipefail

# ─── Configuration ──────────────────────────────────────────────────────────
# Add git tags here as new versions are released.
TAGS=(
  "v0.1.0"
  "v0.2.0"
  "v0.3.0"
)
# ────────────────────────────────────────────────────────────────────────────

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
RESULTS_FILE="$SCRIPT_DIR/results.json"

cd "$REPO_ROOT"

# Guard: refuse to run with uncommitted changes.
if ! git diff --quiet || ! git diff --cached --quiet; then
  echo "Error: working tree has uncommitted changes. Commit or stash first." >&2
  exit 1
fi

if [ ${#TAGS[@]} -eq 0 ]; then
  echo "Error: no tags configured in TAGS array. Edit this script to add tags." >&2
  exit 1
fi

# Record current position so we can restore it on exit.
ORIGINAL_REF="$(git symbolic-ref --short HEAD 2>/dev/null || git rev-parse HEAD)"

restore_branch() {
  echo "Restoring $ORIGINAL_REF..."
  git checkout "$ORIGINAL_REF" --quiet
}
trap restore_branch EXIT

# Build the results JSON array incrementally.
RESULTS="["
FIRST=true

for tag in "${TAGS[@]}"; do
  echo "══════════════════════════════════════════════════════"
  echo "  Benchmarking $tag"
  echo "══════════════════════════════════════════════════════"

  git checkout "$tag" --quiet

  echo "Building (release)..."
  cargo build --release --bin throughput 2>&1

  echo "Running throughput benchmark (60 s)..."
  JSON="$(cargo run --release --bin throughput 2>/dev/null | grep '^{"solved"')"

  if [ "$FIRST" = true ]; then
    FIRST=false
  else
    RESULTS+=","
  fi

  # Inject the tag name into the JSON object.
  # Input:  {"solved":N,"elapsed_secs":X.XX}
  # Output: {"tag":"v0.1.0","solved":N,"elapsed_secs":X.XX}
  ENTRY="$(echo "$JSON" | sed "s/^{/{\"tag\":\"$tag\",/")"
  RESULTS+="$ENTRY"

  echo "  → $JSON"
  echo ""
done

RESULTS+="]"

echo "$RESULTS" > "$RESULTS_FILE"
echo "Results written to $RESULTS_FILE"

# Generate visualization.
if command -v node &>/dev/null; then
  echo "Generating SVG visualization..."
  (cd "$SCRIPT_DIR" && node visualize.mjs)
  echo "Done. See benchmarks/throughput.svg"
else
  echo "Node.js not found — skipping visualization. Run 'node benchmarks/visualize.mjs' manually."
fi
