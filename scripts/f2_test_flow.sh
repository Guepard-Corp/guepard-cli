#!/bin/bash
# F2-only test flow: deploy F2, commit twice, clone, commit again and clone,
# 2 commits per clone, branch from last snapshot of clone, purge one clone, purge F2.
# Edit the variables below, then run after: guepard login
set -e

# --- Edit these ---
NODE_ID="bca2a856-aec2-443c-b46d-f1ee890eaa44"
PASSWORD="postgres"
REPO_NAME=""   # leave empty to use f2-test-<timestamp>
BINARY="cargo run --"   # use "guepard" if using installed CLI
SLEEP_SEC=5   # pause between steps (0 to disable)

TIMESTAMP=$(date +%s)
[ -z "$REPO_NAME" ] && REPO_NAME="f2-test-$TIMESTAMP"
[ -z "$NODE_ID" ] && { echo "Set NODE_ID in the script"; exit 1; }
[ -z "$PASSWORD" ] && { echo "Set PASSWORD in the script"; exit 1; }

echo "🐆 Guepard F2 test flow"
echo "   BINARY=$BINARY  REPO_NAME=$REPO_NAME  NODE_ID=$NODE_ID"
echo "-----------------------------------"

# 1. Deploy database F2
echo -e "\n▶ 1. Deploy F2"
OUT=$($BINARY deploy -p PostgreSQL -v 16 -r us-west -i F2 -d aws -n "$REPO_NAME" -w "$PASSWORD" -f gp.g1.xsmall -s "$NODE_ID" --json)
F2_ID=$(echo "$OUT" | jq -r '.id // .deployment_id // .deployment.id // empty')
[ -z "$F2_ID" ] && { echo "Failed to get F2_ID"; echo "$OUT"; exit 1; }
echo "   F2_ID=$F2_ID"
[ "$SLEEP_SEC" -gt 0 ] && sleep "$SLEEP_SEC"

# 2. Get branch ID
echo -e "\n▶ 2. Get branch ID"
OUT=$($BINARY list branches -x "$F2_ID" --json)
BRANCH_ID=$(echo "$OUT" | jq -r '.[0].id // .[0].branch_id // empty')
[ -z "$BRANCH_ID" ] && { echo "Failed to get BRANCH_ID"; echo "$OUT"; exit 1; }
echo "   BRANCH_ID=$BRANCH_ID"
[ "$SLEEP_SEC" -gt 0 ] && sleep "$SLEEP_SEC"

# 3. Commit twice
echo -e "\n▶ 3. Commit twice"
$BINARY commit -m "First snapshot" -x "$F2_ID" -b "$BRANCH_ID" --json >/dev/null
$BINARY commit -m "Second snapshot" -x "$F2_ID" -b "$BRANCH_ID" --json >/dev/null
echo "   done"
[ "$SLEEP_SEC" -gt 0 ] && sleep "$SLEEP_SEC"

# 4. Clone (first clone)
echo -e "\n▶ 4. First clone"
SNAP1_ID=$($BINARY list commits -x "$F2_ID" --json | jq -r '.[0].id // empty')
[ -z "$SNAP1_ID" ] && { echo "Failed to get SNAP1_ID"; exit 1; }
OUT=$($BINARY clone -x "$F2_ID" -s "$SNAP1_ID" --json)
CLONE1_ID=$(echo "$OUT" | jq -r '.id // .deployment_id // .deployment.id // empty')
[ -z "$CLONE1_ID" ] && { echo "Failed to get CLONE1_ID"; echo "$OUT"; exit 1; }
echo "   CLONE1_ID=$CLONE1_ID"
$BINARY compute start -x "$CLONE1_ID" --json >/dev/null || true
[ "$SLEEP_SEC" -gt 0 ] && sleep "$SLEEP_SEC"

# 5. Commit again, then clone again
echo -e "\n▶ 5. Commit again + second clone"
$BINARY commit -m "Third snapshot" -x "$F2_ID" -b "$BRANCH_ID" --json >/dev/null
SNAP2_ID=$($BINARY list commits -x "$F2_ID" --json | jq -r '.[0].id // empty')
[ -z "$SNAP2_ID" ] && { echo "Failed to get SNAP2_ID"; exit 1; }
OUT=$($BINARY clone -x "$F2_ID" -s "$SNAP2_ID" --json)
CLONE2_ID=$(echo "$OUT" | jq -r '.id // .deployment_id // .deployment.id // empty')
[ -z "$CLONE2_ID" ] && { echo "Failed to get CLONE2_ID"; echo "$OUT"; exit 1; }
echo "   CLONE2_ID=$CLONE2_ID"
$BINARY compute start -x "$CLONE2_ID" --json >/dev/null || true
[ "$SLEEP_SEC" -gt 0 ] && sleep "$SLEEP_SEC"

# 6. Create 2 commits in each clone
echo -e "\n▶ 6. Two commits per clone"
CLONE1_BRANCH_ID=$($BINARY list branches -x "$CLONE1_ID" --json | jq -r '.[0].id // empty')
[ -z "$CLONE1_BRANCH_ID" ] && { echo "Failed to get CLONE1_BRANCH_ID"; exit 1; }
$BINARY commit -m "Clone1 commit 1" -x "$CLONE1_ID" -b "$CLONE1_BRANCH_ID" --json >/dev/null
$BINARY commit -m "Clone1 commit 2" -x "$CLONE1_ID" -b "$CLONE1_BRANCH_ID" --json >/dev/null
CLONE2_BRANCH_ID=$($BINARY list branches -x "$CLONE2_ID" --json | jq -r '.[0].id // empty')
[ -z "$CLONE2_BRANCH_ID" ] && { echo "Failed to get CLONE2_BRANCH_ID"; exit 1; }
$BINARY commit -m "Clone2 commit 1" -x "$CLONE2_ID" -b "$CLONE2_BRANCH_ID" --json >/dev/null
$BINARY commit -m "Clone2 commit 2" -x "$CLONE2_ID" -b "$CLONE2_BRANCH_ID" --json >/dev/null
echo "   done"
[ "$SLEEP_SEC" -gt 0 ] && sleep "$SLEEP_SEC"

# 7. Create branch from last snapshot of last clone (fails on F2 in CLI; continue anyway)
echo -e "\n▶ 7. Branch from last snapshot of clone (may fail on F2)"
LAST_SNAP=$($BINARY list commits -x "$CLONE2_ID" --json | jq -r '.[0].id // empty')
if [ -n "$LAST_SNAP" ]; then
  $BINARY branch -x "$CLONE2_ID" -s "$LAST_SNAP" "feature-from-clone-$TIMESTAMP" --json || true
else
  echo "   (no snapshot, skip)"
fi
[ "$SLEEP_SEC" -gt 0 ] && sleep "$SLEEP_SEC"

# 8. Purge one clone (commented out)
# echo -e "\n▶ 8. Purge one clone (CLONE1_ID)"
# $BINARY deploy -x "$CLONE1_ID" --purge --yes --json >/dev/null || true
# echo "   done"

# 9. Purge the F2 (commented out)
# echo -e "\n▶ 9. Purge F2"
# $BINARY deploy -x "$F2_ID" --purge --yes --json >/dev/null || true
# echo "   done"

echo -e "\n-----------------------------------"
echo "✅ F2 test flow finished."
echo "-----------------------------------"
echo "  F2 ($F2_ID)"
echo "    branch:    $BRANCH_ID"
echo "    snapshots:"
$BINARY list commits -x "$F2_ID" --json 2>/dev/null | jq -r '.[].id' | while read -r s; do echo "      $s"; done
echo ""
echo "  Clone 1 ($CLONE1_ID)"
echo "    branch:    $CLONE1_BRANCH_ID"
echo "    snapshots:"
$BINARY list commits -x "$CLONE1_ID" --json 2>/dev/null | jq -r '.[].id' | while read -r s; do echo "      $s"; done
echo ""
echo "  Clone 2 ($CLONE2_ID)"
echo "    branch:    $CLONE2_BRANCH_ID"
echo "    snapshots:"
$BINARY list commits -x "$CLONE2_ID" --json 2>/dev/null | jq -r '.[].id' | while read -r s; do echo "      $s"; done
echo "-----------------------------------"
