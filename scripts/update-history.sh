#!/usr/bin/env bash
set -euo pipefail

REPORT_FILE="${1:-}"
HISTORY_DIR="${2:-}"
RUN_ID="${3:-}"
COMMIT_SHA="${4:-}"
TIMESTAMP="${5:-}"

if [[ -z "$REPORT_FILE" || -z "$HISTORY_DIR" || -z "$RUN_ID" || -z "$COMMIT_SHA" || -z "$TIMESTAMP" ]]; then
    echo "Usage: $0 <report-file> <history-dir> <run-id> <commit-sha> <timestamp>"
    exit 1
fi

if ! command -v jq &> /dev/null; then
    echo "Error: jq is required but not installed."
    exit 1
fi

if [[ ! -f "$REPORT_FILE" ]]; then
    echo "Error: report file $REPORT_FILE not found."
    exit 1
fi

# Validate report JSON and extract fields
if ! jq -e '.profile and .fixture and .status and .summary and .results' "$REPORT_FILE" >/dev/null 2>&1; then
    echo "Error: report file is invalid or missing required fields."
    exit 1
fi

PROFILE=$(jq -r '.profile' "$REPORT_FILE")
FIXTURE=$(jq -r '.fixture' "$REPORT_FILE")
STATUS=$(jq -r '.status' "$REPORT_FILE")
SUMMARY=$(jq -c '.summary' "$REPORT_FILE")

mkdir -p "$HISTORY_DIR/history"

MANIFEST_FILE="$HISTORY_DIR/history/manifest.json"

if [[ ! -f "$MANIFEST_FILE" ]]; then
    echo '{"runs":[]}' > "$MANIFEST_FILE"
fi

# Validate manifest
if ! jq -e '.runs | type == "array"' "$MANIFEST_FILE" >/dev/null 2>&1; then
    echo "Error: manifest file is malformed."
    exit 1
fi

REPORT_DEST="$HISTORY_DIR/history/${RUN_ID}.json"
cp "$REPORT_FILE" "$REPORT_DEST"

# Update manifest
NEW_ENTRY=$(jq -n \
    --arg id "$RUN_ID" \
    --arg commit "$COMMIT_SHA" \
    --arg timestamp "$TIMESTAMP" \
    --arg profile "$PROFILE" \
    --arg fixture "$FIXTURE" \
    --arg status "$STATUS" \
    --argjson summary "$SUMMARY" \
    --arg report "history/${RUN_ID}.json" \
    '{id: $id, commit: $commit, timestamp: $timestamp, profile: $profile, fixture: $fixture, status: $status, summary: $summary, report: $report}')

# Upsert (remove existing if same id), prepend, slice to 50
tmp_manifest=$(mktemp)
jq --argjson new_entry "$NEW_ENTRY" --arg id "$RUN_ID" \
    '.runs = ([$new_entry] + [.runs[] | select(.id != $id)])[:50]' \
    "$MANIFEST_FILE" > "$tmp_manifest"
mv "$tmp_manifest" "$MANIFEST_FILE"

# Clean up orphaned reports
expected_files=$(jq -r '.runs[].report' "$MANIFEST_FILE" | tr -d '\r')
for file in "$HISTORY_DIR/history"/*.json; do
    filename=$(basename "$file")
    if [[ "$filename" == "manifest.json" ]]; then
        continue
    fi
    is_expected=0
    for expected in $expected_files; do
        if [[ "$expected" == "history/$filename" ]]; then
            is_expected=1
            break
        fi
    done
    if [[ $is_expected -eq 0 ]]; then
        rm -f "$file"
    fi
done
