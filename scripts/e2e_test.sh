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
    echo "1)  [Config]  Show API settings"
    echo "2)  [Auth]    Login with Token"
    echo "3)  [Deploy]  Get Deployment Details (-x)"
    echo "4)  [List]    List All Deployments"
    echo "5)  [List]    List Performance Profiles"
    echo "6)  [Branch]  List Branches for Target"
    echo "7)  [Commit]  List Commits for Target"
    echo "8)  [Log]     View Recent Logs"
    echo "9)  [Log]     Follow Logs (JSON NDJSON)"
    echo "10) [Compute] Check Compute Status"
    echo "11) [Usage]   Show Account Usage"
    echo "12) [Deploy]  Create New Deployment (F2)"
    echo "q)  Quit"
    
    read -p "👉 Select a command: " choice

    case $choice in
        1) run_cmd "$BINARY config --show --json" "Config displayed" ;;
        2) run_cmd "$BINARY login --code '$TOKEN' --json" "Login successful" ;;
        3) run_cmd "$BINARY deploy -x '$DEPLOY_ID' --json" "Details retrieved" ;;
        4) run_cmd "$BINARY list deployments --json" "Deployments listed" ;;
        5) run_cmd "$BINARY list performance --json" "Performance profiles listed" ;;
        6) run_cmd "$BINARY list branches -x '$DEPLOY_ID' --json" "Branches listed" ;;
        7) run_cmd "$BINARY list commits -x '$DEPLOY_ID' --json" "Commits listed" ;;
        8) run_cmd "$BINARY log -x '$DEPLOY_ID' -n 10 --json" "Logs retrieved" ;;
        9) echo "👀 Press Ctrl+C to stop following..."; $BINARY log -x "$DEPLOY_ID" --follow --json ;;
        10) run_cmd "$BINARY compute status -x '$DEPLOY_ID' --json" "Compute status retrieved" ;;
        11) run_cmd "$BINARY usage --json" "Usage statistics retrieved" ;;
        12) 
            REPO_NAME="e2e-test-$(date +%s)"
            NODE_ID="e1b33620-ea91-437f-9b8e-6334040a7423"
            run_cmd "$BINARY deploy -p PostgreSQL -v 16 -r us-west -i F2 -d aws -n $REPO_NAME -w Pass123! -f gp.g1.xsmall -s $NODE_ID --json" "New deployment created"
            ;;
        q|quit|exit) echo "👋 Goodbye!"; exit 0 ;;
        *) echo "❌ Invalid option" ;;
    esac
done
