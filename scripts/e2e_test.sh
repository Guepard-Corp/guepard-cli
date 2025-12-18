#!/bin/bash
set -e

# --- Configuration ---
API_URL=${PUBLIC_API:-"https://api.guepard.run"}
TOKEN=${GUEPARD_TOKEN:-"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiI1YzVkZWU0OS1kNzRmLTQwZTEtOTFlYi0wNWQ5OTNlMDFiNjUiLCJhdWQiOiJhdXRoZW50aWNhdGVkIiwicm9sZSI6ImF1dGhlbnRpY2F0ZWQiLCJzY29wZXMiOlsiYWRtaW4iXSwiZXhwIjoxNzY4NjY0NzgzLCJ0b2tlbl9pZCI6ImIxYjRjNmIzLTdkNWYtNGM3NC04OTgwLTU4MDNkYTJlMDI2OCIsImlhdCI6MTc2NjA3Mjc4NX0.BDswpd0OWB2lIZhmOqPM8r9mN9_YfTDxVSrVX0WsjKI"}
DEPLOY_ID=${DEPLOY_ID:-"abbfa037-3af6-4758-b724-88147fb4b6bd"}
BRANCH_ID=${BRANCH_ID:-""}
SNAP_ID=${SNAP_ID:-""}
TIMESTAMP=$(date +%s)
BINARY="cargo run --"

echo "🐆 Guepard CLI Interactive E2E Tool"
echo "API: $API_URL"
echo "Target ID: $DEPLOY_ID"
if [ -n "$BRANCH_ID" ]; then echo "Branch ID: $BRANCH_ID"; fi
if [ -n "$SNAP_ID" ]; then echo "Snapshot ID: $SNAP_ID"; fi
echo "-----------------------------------"

# --- CLI Argument Parsing ---
NON_INTERACTIVE=false
SINGLE_TEST=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --non-interactive|-y)
            NON_INTERACTIVE=true
            shift
            ;;
        --test|-t)
            SINGLE_TEST="$2"
            NON_INTERACTIVE=true
            shift 2
            ;;
        --deploy-id|-x)
            DEPLOY_ID="$2"
            shift 2
            ;;
        --branch-id|-b)
            BRANCH_ID="$2"
            shift 2
            ;;
        --snap-id|-s)
            SNAP_ID="$2"
            shift 2
            ;;
        *)
            shift
            ;;
    esac
done

# Helper for JSON display and validation
run_cmd() {
    local cmd="$1"
    local msg="$2"
    local update_var="$3"
    echo -e "\n🚀 Running: $cmd"
    
    local output
    # Run the command and capture output
    output=$(eval "$cmd")
    local exit_code=$?
    
    if [ $exit_code -ne 0 ]; then
        echo -e "❌ Command failed with exit code $exit_code"
        if [ "$NON_INTERACTIVE" = true ]; then
            exit $exit_code
        fi
        return $exit_code
    fi

    echo "📄 Result:"
    echo "$output" | jq . || echo "$output"
    echo -e "✅ $msg"
    
    # Update global variable if requested
    if [ -n "$update_var" ]; then
        case $update_var in
            DEPLOY_ID)
                DEPLOY_ID=$(echo "$output" | jq -r 'if type == "array" then .[0].id // .[0].deployment_id else .id // .deployment_id // .deployment.id end' | grep -v null || echo "")
                [ -n "$DEPLOY_ID" ] && echo "📌 Cached DEPLOY_ID: $DEPLOY_ID"
                ;;
            SNAP_ID)
                SNAP_ID=$(echo "$output" | jq -r 'if type == "array" then .[0].id // .[0].snapshot_id // .[0].commit_id else .id // .snapshot_id // .commit_id end' | grep -v null || echo "")
                [ -n "$SNAP_ID" ] && echo "📌 Cached SNAP_ID: $SNAP_ID"
                ;;
            BRANCH_ID)
                BRANCH_ID=$(echo "$output" | jq -r 'if type == "array" then .[0].id // .[0].branch_id else .id // .branch_id end' | grep -v null || echo "")
                [ -n "$BRANCH_ID" ] && echo "📌 Cached BRANCH_ID: $BRANCH_ID"
                ;;
            CLONE_ID)
                CLONE_ID=$(echo "$output" | jq -r 'if type == "array" then .[0].id // .[0].deployment_id else .id // .deployment_id // .deployment.id end' | grep -v null || echo "")
                [ -n "$CLONE_ID" ] && echo "📌 Cached CLONE_ID: $CLONE_ID"
                ;;
        esac
    fi

    if [ "$NON_INTERACTIVE" = false ]; then
        read -p "Press Enter to continue..."
    fi
}

execute_test() {
    local choice=$1
    case $choice in
        1) run_cmd "$BINARY config --show --json" "Config displayed" ;;
        2) run_cmd "$BINARY config --api-url 'http://localhost:3000' --json" "API URL updated" ;;
        3) run_cmd "$BINARY login --code '$TOKEN' --json" "Login successful" ;;
        4) run_cmd "$BINARY logout --json" "Logout successful" ;;
        
        5) run_cmd "$BINARY list deployments --json" "Deployments listed" ;;
        6) run_cmd "$BINARY deploy -x '$DEPLOY_ID' --json" "Details retrieved" ;;
        7) 
            REPO_NAME="e2e-f2-$TIMESTAMP"
            NODE_ID="e1b33620-ea91-437f-9b8e-6334040a7423"
            run_cmd "$BINARY deploy -p PostgreSQL -v 16 -r us-west -i F2 -d aws -n $REPO_NAME -w Pass123! -f gp.g1.xsmall -s $NODE_ID --json" "F2 deployment created" DEPLOY_ID
            ;;
        8) 
            REPO_NAME="e2e-repo-$TIMESTAMP"
            NODE_ID="e1b33620-ea91-437f-9b8e-6334040a7423"
            run_cmd "$BINARY deploy -p PostgreSQL -v 16 -r us-west -i REPOSITORY -d aws -n $REPO_NAME -w Pass123! -f gp.g1.xsmall -s $NODE_ID --json" "REPOSITORY deployment created" DEPLOY_ID
            ;;
        9) 
            if [ "$NON_INTERACTIVE" = true ]; then
                echo "⏭️ Skipping interactive wizard in non-interactive mode"
            else
                echo "🪄 Launching interactive wizard..."; $BINARY deploy --interactive
            fi
            ;;
        10) run_cmd "$BINARY deploy -x '$DEPLOY_ID' -n 'updated-name-$TIMESTAMP' --json" "Deployment updated" ;;
        11) 
            if [ "$NON_INTERACTIVE" = true ]; then
                echo "⏭️ Skipping deletion in non-interactive mode for safety"
            else
                echo "⚠️ This will delete $DEPLOY_ID"; run_cmd "$BINARY deploy -x '$DEPLOY_ID' --yes --json" "Deployment deleted"
            fi
            ;;

        12) run_cmd "$BINARY list branches -x '$DEPLOY_ID' --json" "Branches listed" ;;
        13) run_cmd "$BINARY list commits -x '$DEPLOY_ID' --json" "Commits listed" ;;
        14) 
            local use_snap=${SNAP_ID}
            if [ -z "$use_snap" ]; then
                use_snap=$(cargo run -- list commits -x "$DEPLOY_ID" --json | jq -r '.[0].id')
            fi
            run_cmd "$BINARY branch 'dev-$TIMESTAMP' -x '$DEPLOY_ID' -s '$use_snap' --json" "Branch created" BRANCH_ID
            ;;
        15) 
            local use_snap=${SNAP_ID}
            if [ -z "$use_snap" ]; then
                use_snap=$(cargo run -- list commits -x "$DEPLOY_ID" --json | jq -r '.[0].id')
            fi
            run_cmd "$BINARY branch 'temp-$TIMESTAMP' -x '$DEPLOY_ID' -s '$use_snap' --ephemeral --checkout --json" "Ephemeral branch created and checked out" BRANCH_ID
            ;;
        16)
            local use_snap=${SNAP_ID}
            if [ -z "$use_snap" ]; then
                use_snap=$(cargo run -- list commits -x "$DEPLOY_ID" --json | jq -r '.[0].id')
            fi
            run_cmd "$BINARY branch 'clean-$TIMESTAMP' -x '$DEPLOY_ID' -s '$use_snap' -d true --json" "Branch created with discard_changes" BRANCH_ID
            ;;
        17) 
            local use_branch=${BRANCH_ID}
            if [ -z "$use_branch" ]; then
                use_branch=$(cargo run -- list branches -x "$DEPLOY_ID" --json | jq -r '.[0].id')
            fi
            run_cmd "$BINARY commit -m 'E2E Test Snapshot' -x '$DEPLOY_ID' -b '$use_branch' --json" "Commit created" SNAP_ID
            ;;
        18) 
            local use_branch=${BRANCH_ID}
            if [ -z "$use_branch" ]; then
                use_branch=$(cargo run -- list branches -x "$DEPLOY_ID" --json | jq -r '.[0].id')
            fi
            run_cmd "$BINARY checkout -x '$DEPLOY_ID' -c '$use_branch' --json" "Branch checked out"
            ;;
        19) 
            local use_snap=${SNAP_ID}
            if [ -z "$use_snap" ]; then
                use_snap=$(cargo run -- list commits -x "$DEPLOY_ID" --json | jq -r '.[0].id')
            fi
            run_cmd "$BINARY checkout -x '$DEPLOY_ID' -s '$use_snap' --json" "Snapshot restored"
            ;;
        20)
            local use_branch=${BRANCH_ID}
            if [ -z "$use_branch" ]; then
                use_branch=$(cargo run -- list branches -x "$DEPLOY_ID" --json | jq -r '.[0].id')
            fi
            run_cmd "$BINARY checkout -x '$DEPLOY_ID' -c '$use_branch' -d true --json" "Checkout with discard_changes"
            ;;

        21) run_cmd "$BINARY log -x '$DEPLOY_ID' -n 20 --json" "Logs retrieved" ;;
        22) run_cmd "$BINARY log -x '$DEPLOY_ID' --stdout-only --json" "Stdout logs retrieved" ;;
        23) run_cmd "$BINARY log -x '$DEPLOY_ID' --stderr-only --json" "Stderr logs retrieved" ;;
        24) run_cmd "$BINARY log -x '$DEPLOY_ID' --timestamps --json" "Logs with timestamps retrieved" ;;
        25) run_cmd "$BINARY log -x '$DEPLOY_ID' --since '2025-01-01' --json" "Filtered logs retrieved" ;;
        26) 
            if [ "$NON_INTERACTIVE" = true ]; then
                echo "⏭️ Skipping log follow in non-interactive mode"
            else
                echo "👀 Press Ctrl+C to stop following..."; $BINARY log -x "$DEPLOY_ID" --follow --json
            fi
            ;;

        27) run_cmd "$BINARY compute status -x '$DEPLOY_ID' --json" "Compute status retrieved" ;;
        28) run_cmd "$BINARY compute list -x '$DEPLOY_ID' --json" "Compute details retrieved" ;;
        29) run_cmd "$BINARY compute start -x '$DEPLOY_ID' --json" "Compute started" ;;
        30) run_cmd "$BINARY compute stop -x '$DEPLOY_ID' --json" "Compute stopped" ;;
        31) run_cmd "$BINARY compute restart -x '$DEPLOY_ID' --json" "Compute restarted" ;;
        32) run_cmd "$BINARY compute logs -x '$DEPLOY_ID' --json" "Compute logs retrieved" ;;

        33) 
            local use_snap=${SNAP_ID}
            if [ -z "$use_snap" ]; then
                use_snap=$(cargo run -- list commits -x "$DEPLOY_ID" --json | jq -r '.[0].id')
            fi
            run_cmd "$BINARY clone -x '$DEPLOY_ID' -s '$use_snap' --json" "Clone created" CLONE_ID
            ;;
        34)
            local use_snap=${SNAP_ID}
            if [ -z "$use_snap" ]; then
                use_snap=$(cargo run -- list commits -x "$DEPLOY_ID" --json | jq -r '.[0].id')
            fi
            run_cmd "$BINARY clone -x '$DEPLOY_ID' -s '$use_snap' -n 'custom-clone-$TIMESTAMP' -f gp.g1.small --json" "Custom clone created" CLONE_ID
            ;;
        35) run_cmd "$BINARY clone list -x '$DEPLOY_ID' --json" "Clones listed" ;;

        36) run_cmd "$BINARY usage --json" "Usage statistics retrieved (JSON)" ;;
        37) run_cmd "$BINARY usage" "Usage statistics retrieved (Table)" ;;
        38) run_cmd "$BINARY list performance --json" "Performance profiles listed" ;;
        39) run_cmd "$BINARY list deployments --columns id,name,status --json" "Custom columns listed" ;;
        40) run_cmd "$BINARY list commits -x '$DEPLOY_ID' --all --json" "All commits listed" ;;
        41) run_cmd "$BINARY list commits -x '$DEPLOY_ID' --graph" "Git graph displayed" ;;
        42) run_dynamic_workflow ;;
        
        q|quit|exit) echo "👋 Goodbye!"; exit 0 ;;
        *) echo "❌ Invalid option: $choice" ;;
    esac
}

run_dynamic_workflow() {
    echo -e "\n🔄 Starting Dynamic Workflow Test..."
    
    # 1. Get current branch and latest snapshot
    echo "📍 Step 1: Getting latest data from $DEPLOY_ID"
    local branches_json=$(cargo run -- list branches -x "$DEPLOY_ID" --json)
    local branch_id=$(echo "$branches_json" | jq -r '.[0].id')
    local branch_name=$(echo "$branches_json" | jq -r '.[0].name')
    
    local commits_json=$(cargo run -- list commits -x "$DEPLOY_ID" --json)
    local last_snap_id=$(echo "$commits_json" | jq -r '.[0].id')
    
    echo "   Using Branch: $branch_name ($branch_id)"
    echo "   Using Snapshot: $last_snap_id"

    # 2. Create a new commit
    echo -e "\n📸 Step 2: Creating a new commit..."
    local commit_msg="E2E Workflow Commit $(date +%s)"
    local commit_json=$(cargo run -- commit -m "$commit_msg" -x "$DEPLOY_ID" -b "$branch_id" --json)
    local new_snap_id=$(echo "$commit_json" | jq -r '.id')
    echo "   Created Commit: $new_snap_id"

    # 3. Checkout the new commit
    echo -e "\n🔄 Step 3: Checking out the new commit..."
    run_cmd "$BINARY checkout -x '$DEPLOY_ID' -s '$new_snap_id' --json" "Snapshot checked out"

    # 4. Create a clone from the new commit
    echo -e "\n🎭 Step 4: Creating a clone from $new_snap_id..."
    local clone_name="workflow-clone-$(date +%s)"
    local clone_json=$(cargo run -- clone -x "$DEPLOY_ID" -s "$new_snap_id" -n "$clone_name" --json)
    local clone_id=$(echo "$clone_json" | jq -r '.deployment.id')
    echo "   Created Clone: $clone_id"

    # 5. List branches in the clone
    echo -e "\n🌿 Step 5: Listing branches in the clone..."
    local clone_branches_json=$(cargo run -- list branches -x "$clone_id" --json)
    local clone_branch_id=$(echo "$clone_branches_json" | jq -r '.[0].id')
    echo "   Clone Main Branch: $clone_branch_id"

    # 6. Create a branch in the clone
    echo -e "\n🌿 Step 6: Creating a branch in the clone..."
    local new_branch_name="feature-workflow-$(date +%s)"
    run_cmd "$BINARY branch '$new_branch_name' -x '$clone_id' -s '$new_snap_id' --json" "Branch created in clone"

    # 7. Create a commit in the clone
    echo -e "\n📸 Step 7: Creating a commit in the clone..."
    run_cmd "$BINARY commit -m 'Commit in clone' -x '$clone_id' -b '$clone_branch_id' --json" "Commit created in clone"

    echo -e "\n✅ Dynamic Workflow Test Completed Successfully!"
}

# --- Non-Interactive Execution ---
if [ "$NON_INTERACTIVE" = true ]; then
    if [ -n "$SINGLE_TEST" ]; then
        execute_test "$SINGLE_TEST"
    else
        echo "🏃 Running full E2E workflow..."
        
        # 1. Login
        execute_test 3
        
        # 2. List & Meta
        execute_test 1  # Config
        execute_test 38 # Performance
        execute_test 36 # Usage
        
        # 3. Create a REPOSITORY deployment for testing
        echo -e "\n📦 Step 3: Creating a REPOSITORY deployment..."
        REPO_NAME="e2e-repo-$TIMESTAMP"
        NODE_ID="e1b33620-ea91-437f-9b8e-6334040a7423"
        CREATE_OUT=$($BINARY deploy -p PostgreSQL -v 16 -r us-west -i REPOSITORY -d aws -n "$REPO_NAME" -w Pass123! -f gp.g1.xsmall -s "$NODE_ID" --json)
        DEPLOY_ID=$(echo "$CREATE_OUT" | jq -r '.deployment.id')
        echo "✅ Created Deployment: $DEPLOY_ID"
        
        # 4. Deployment Details
        execute_test 6
        
        # 5. VC Operations on Repo
        echo -e "\n🌿 Step 5: Version Control Operations..."
        # List initial branches (should have main)
        execute_test 12
        BRANCH_ID=$(cargo run -- list branches -x "$DEPLOY_ID" --json | jq -r '.[0].id')
        
        # Create Commit 1
        run_cmd "$BINARY commit -m 'E2E Commit 1' -x '$DEPLOY_ID' -b '$BRANCH_ID' --json" "Commit 1 created"
        SNAP_1=$(cargo run -- list commits -x "$DEPLOY_ID" --json | jq -r '.[0].id')
        
        # Create Commit 2
        run_cmd "$BINARY commit -m 'E2E Commit 2' -x '$DEPLOY_ID' -b '$BRANCH_ID' --json" "Commit 2 created"
        SNAP_2=$(cargo run -- list commits -x "$DEPLOY_ID" --json | jq -r '.[0].id')
        
        # Checkout to SNAP_1
        run_cmd "$BINARY checkout -x '$DEPLOY_ID' -s '$SNAP_1' --json" "Restored to Snapshot 1"
        
        # 6. Clones
        echo -e "\n🎭 Step 6: Clone Operations..."
        CLONE_OUT=$($BINARY clone -x "$DEPLOY_ID" -s "$SNAP_2" -n "e2e-clone-$TIMESTAMP" --json)
        CLONE_ID=$(echo "$CLONE_OUT" | jq -r '.deployment.id')
        echo "✅ Created Clone: $CLONE_ID"
        
        # VC on Clone
        run_cmd "$BINARY list branches -x '$CLONE_ID' --json" "Branches in clone listed"
        CLONE_BRANCH_ID=$(cargo run -- list branches -x "$CLONE_ID" --json | jq -r '.[0].id')
        
        run_cmd "$BINARY branch 'feat-clone-$TIMESTAMP' -x '$CLONE_ID' -s '$SNAP_2' --json" "New branch in clone"
        
        # 7. Compute
        echo -e "\n💻 Step 7: Compute Operations..."
        execute_test 27 # Status
        execute_test 28 # Details
        
        # 8. Logs
        echo -e "\n📋 Step 8: Log Operations..."
        execute_test 21 # Recent logs
        
        echo -e "\n🏁 Full E2E workflow completed successfully!"
    fi
    exit 0
fi

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
    echo "42) [Flow]    Run Dynamic Workflow (Commit->Checkout->Clone->Branch...)"
    echo "q)  Quit"
    
    read -p "👉 Select a command: " choice
    execute_test "$choice"
done
