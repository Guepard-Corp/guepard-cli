#!/bin/bash
set -e

# --- Configuration ---
API_URL=${PUBLIC_API:-"https://api.guepard.run"}
TOKEN=${GUEPARD_TOKEN:-"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiI1YzVkZWU0OS1kNzRmLTQwZTEtOTFlYi0wNWQ5OTNlMDFiNjUiLCJhdWQiOiJhdXRoZW50aWNhdGVkIiwicm9sZSI6ImF1dGhlbnRpY2F0ZWQiLCJzY29wZXMiOlsiYWRtaW4iXSwiZXhwIjoxNzY4NjY0NzgzLCJ0b2tlbl9pZCI6ImIxYjRjNmIzLTdkNWYtNGM3NC04OTgwLTU4MDNkYTJlMDI2OCIsImlhdCI6MTc2NjA3Mjc4NX0.BDswpd0OWB2lIZhmOqPM8r9mN9_YfTDxVSrVX0WsjKI"}
DEPLOY_ID="abbfa037-3af6-4758-b724-88147fb4b6bd"
TIMESTAMP=$(date +%s)
BINARY="cargo run --"

echo "🐆 Guepard CLI Interactive E2E Tool"
echo "API: $API_URL"
echo "Target ID: $DEPLOY_ID"
echo "-----------------------------------"

# Helper for JSON display and validation
run_cmd() {
    local cmd="$1"
    local msg="$2"
    echo -e "\n🚀 Running: $cmd"
    
    local output
    # Run the command and capture output
    output=$(eval "$cmd")
    local exit_code=$?
    
    if [ $exit_code -ne 0 ]; then
        echo -e "❌ Command failed with exit code $exit_code"
        return $exit_code
    fi

    echo "📄 Result:"
    echo "$output" | jq . || echo "$output"
    echo -e "✅ $msg"
}

# Main Menu Loop
while true; do
    echo -e "\n--- Available Test Commands ---"
    echo "⚙️  CONFIG & AUTH"
    echo "1)  [Config]  Show Configuration"
    echo "2)  [Config]  Set API URL (Localhost)"
    echo "3)  [Auth]    Login with Token"
    echo "4)  [Auth]    Logout"
    echo ""
    echo "🚀 DEPLOYMENT"
    echo "5)  [Deploy]  List All Deployments"
    echo "6)  [Deploy]  Get Details (-x)"
    echo "7)  [Deploy]  Create New (F2)"
    echo "8)  [Deploy]  Create New (REPOSITORY)"
    echo "9)  [Deploy]  Interactive Wizard"
    echo "10) [Deploy]  Update Name"
    echo "11) [Deploy]  Delete Deployment"
    echo ""
    echo "🌿 VERSION CONTROL (VC)"
    echo "12) [VC]      List Branches"
    echo "13) [VC]      List Commits (Snapshots)"
    echo "14) [VC]      Create Branch (Regular)"
    echo "15) [VC]      Create Branch (Ephemeral + Checkout)"
    echo "16) [VC]      Create Branch (with Discard Changes)"
    echo "17) [VC]      Create Commit (Snapshot)"
    echo "18) [VC]      Checkout Branch"
    echo "19) [VC]      Checkout Snapshot"
    echo "20) [VC]      Checkout (with Discard Changes)"
    echo ""
    echo "📋 LOGS"
    echo "21) [Logs]    View Recent (20 lines)"
    echo "22) [Logs]    View Stdout Only"
    echo "23) [Logs]    View Stderr Only"
    echo "24) [Logs]    View with Timestamps"
    echo "25) [Logs]    View Since '2025-01-01'"
    echo "26) [Logs]    Follow Logs (Real-time)"
    echo ""
    echo "💻 COMPUTE"
    echo "27) [Compute] Status"
    echo "28) [Compute] List Details"
    echo "29) [Compute] Start"
    echo "30) [Compute] Stop"
    echo "31) [Compute] Restart"
    echo "32) [Compute] Compute Logs"
    echo ""
    echo "🎭 CLONES"
    echo "33) [Clone]   Create Clone (Latest Snap)"
    echo "34) [Clone]   Create Clone (Custom Name & Perf)"
    echo "35) [Clone]   List Clones"
    echo ""
    echo "📊 META & LISTING"
    echo "36) [Meta]    Show Usage (JSON)"
    echo "37) [Meta]    Show Usage (Table)"
    echo "38) [List]    List Performance Profiles"
    echo "39) [List]    List Deployments (Custom Columns: id,name,status)"
    echo "40) [List]    List Commits (Including Auto Snaps)"
    echo "41) [List]    List Commits (Git Graph Table)"
    echo "q)  Quit"
    
    read -p "👉 Select a command: " choice

    case $choice in
        1) run_cmd "$BINARY config --show --json" "Config displayed" ;;
        2) run_cmd "$BINARY config --api-url 'http://localhost:3000' --json" "API URL updated" ;;
        3) run_cmd "$BINARY login --code '$TOKEN' --json" "Login successful" ;;
        4) run_cmd "$BINARY logout --json" "Logout successful" ;;
        
        5) run_cmd "$BINARY list deployments --json" "Deployments listed" ;;
        6) run_cmd "$BINARY deploy -x '$DEPLOY_ID' --json" "Details retrieved" ;;
        7) 
            REPO_NAME="e2e-f2-$(date +%s)"
            NODE_ID="e1b33620-ea91-437f-9b8e-6334040a7423"
            run_cmd "$BINARY deploy -p PostgreSQL -v 16 -r us-west -i F2 -d aws -n $REPO_NAME -w Pass123! -f gp.g1.xsmall -s $NODE_ID --json" "F2 deployment created"
            ;;
        8) 
            REPO_NAME="e2e-repo-$(date +%s)"
            NODE_ID="e1b33620-ea91-437f-9b8e-6334040a7423"
            run_cmd "$BINARY deploy -p PostgreSQL -v 16 -r us-west -i REPOSITORY -d aws -n $REPO_NAME -w Pass123! -f gp.g1.xsmall -s $NODE_ID --json" "REPOSITORY deployment created"
            ;;
        9) echo "🪄 Launching interactive wizard..."; $BINARY deploy --interactive ;;
        10) run_cmd "$BINARY deploy -x '$DEPLOY_ID' -n 'updated-name-$(date +%s)' --json" "Deployment updated" ;;
        11) echo "⚠️ This will delete $DEPLOY_ID"; run_cmd "$BINARY deploy -x '$DEPLOY_ID' --yes --json" "Deployment deleted" ;;

        12) run_cmd "$BINARY list branches -x '$DEPLOY_ID' --json" "Branches listed" ;;
        13) run_cmd "$BINARY list commits -x '$DEPLOY_ID' --json" "Commits listed" ;;
        14) 
            SNAP_ID=$(cargo run -- list commits -x "$DEPLOY_ID" --json | jq -r '.[0].id')
            run_cmd "$BINARY branch 'dev-$(date +%s)' -x '$DEPLOY_ID' -s '$SNAP_ID' --json" "Branch created"
            ;;
        15) 
            SNAP_ID=$(cargo run -- list commits -x "$DEPLOY_ID" --json | jq -r '.[0].id')
            run_cmd "$BINARY branch 'temp-$(date +%s)' -x '$DEPLOY_ID' -s '$SNAP_ID' --ephemeral --checkout --json" "Ephemeral branch created and checked out"
            ;;
        16)
            SNAP_ID=$(cargo run -- list commits -x "$DEPLOY_ID" --json | jq -r '.[0].id')
            run_cmd "$BINARY branch 'clean-$(date +%s)' -x '$DEPLOY_ID' -s '$SNAP_ID' -d true --json" "Branch created with discard_changes"
            ;;
        17) 
            BRANCH_ID=$(cargo run -- list branches -x "$DEPLOY_ID" --json | jq -r '.[0].id')
            run_cmd "$BINARY commit -m 'E2E Test Snapshot' -x '$DEPLOY_ID' -b '$BRANCH_ID' --json" "Commit created"
            ;;
        18) 
            BRANCH_ID=$(cargo run -- list branches -x "$DEPLOY_ID" --json | jq -r '.[0].id')
            run_cmd "$BINARY checkout -x '$DEPLOY_ID' -c '$BRANCH_ID' --json" "Branch checked out"
            ;;
        19) 
            SNAP_ID=$(cargo run -- list commits -x "$DEPLOY_ID" --json | jq -r '.[0].id')
            run_cmd "$BINARY checkout -x '$DEPLOY_ID' -s '$SNAP_ID' --json" "Snapshot restored"
            ;;
        20)
            BRANCH_ID=$(cargo run -- list branches -x "$DEPLOY_ID" --json | jq -r '.[0].id')
            run_cmd "$BINARY checkout -x '$DEPLOY_ID' -c '$BRANCH_ID' -d true --json" "Checkout with discard_changes"
            ;;

        21) run_cmd "$BINARY log -x '$DEPLOY_ID' -n 20 --json" "Logs retrieved" ;;
        22) run_cmd "$BINARY log -x '$DEPLOY_ID' --stdout-only --json" "Stdout logs retrieved" ;;
        23) run_cmd "$BINARY log -x '$DEPLOY_ID' --stderr-only --json" "Stderr logs retrieved" ;;
        24) run_cmd "$BINARY log -x '$DEPLOY_ID' --timestamps --json" "Logs with timestamps retrieved" ;;
        25) run_cmd "$BINARY log -x '$DEPLOY_ID' --since '2025-01-01' --json" "Filtered logs retrieved" ;;
        26) echo "👀 Press Ctrl+C to stop following..."; $BINARY log -x "$DEPLOY_ID" --follow --json ;;

        27) run_cmd "$BINARY compute status -x '$DEPLOY_ID' --json" "Compute status retrieved" ;;
        28) run_cmd "$BINARY compute list -x '$DEPLOY_ID' --json" "Compute details retrieved" ;;
        29) run_cmd "$BINARY compute start -x '$DEPLOY_ID' --json" "Compute started" ;;
        30) run_cmd "$BINARY compute stop -x '$DEPLOY_ID' --json" "Compute stopped" ;;
        31) run_cmd "$BINARY compute restart -x '$DEPLOY_ID' --json" "Compute restarted" ;;
        32) run_cmd "$BINARY compute logs -x '$DEPLOY_ID' --json" "Compute logs retrieved" ;;

        33) 
            SNAP_ID=$(cargo run -- list commits -x "$DEPLOY_ID" --json | jq -r '.[0].id')
            run_cmd "$BINARY clone -x '$DEPLOY_ID' -s '$SNAP_ID' --json" "Clone created"
            ;;
        34)
            SNAP_ID=$(cargo run -- list commits -x "$DEPLOY_ID" --json | jq -r '.[0].id')
            run_cmd "$BINARY clone -x '$DEPLOY_ID' -s '$SNAP_ID' -n 'custom-clone-$(date +%s)' -f gp.g1.small --json" "Custom clone created"
            ;;
        35) run_cmd "$BINARY clone list -x '$DEPLOY_ID' --json" "Clones listed" ;;

        36) run_cmd "$BINARY usage --json" "Usage statistics retrieved (JSON)" ;;
        37) $BINARY usage ;;
        38) run_cmd "$BINARY list performance --json" "Performance profiles listed" ;;
        39) run_cmd "$BINARY list deployments --columns id,name,status --json" "Custom columns listed" ;;
        40) run_cmd "$BINARY list commits -x '$DEPLOY_ID' --all --json" "All commits listed" ;;
        41) $BINARY list commits -x "$DEPLOY_ID" --graph ;;
        
        q|quit|exit) echo "👋 Goodbye!"; exit 0 ;;
        *) echo "❌ Invalid option" ;;
    esac
done
