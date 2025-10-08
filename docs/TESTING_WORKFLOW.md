# Guepard CLI Complete Testing Workflow

This document provides a comprehensive testing workflow for all Guepard CLI commands. Follow this workflow to test every command and feature systematically.

## Prerequisites

1. **Authentication Setup**
   ```bash
   # Test login process
   guepard login
   
   # Verify authentication
   guepard usage
   ```

2. **Test Environment Variables**
   ```bash
   # Set test variables (replace with actual values)
   export TEST_DEPLOYMENT_ID="your-test-deployment-id"
   export TEST_BRANCH_ID="your-test-branch-id"
   export TEST_SNAPSHOT_ID="your-test-snapshot-id"
   export TEST_REPO_NAME="test-repo-$(date +%s)"
   ```

## Complete Command Testing Workflow

### Phase 1: Authentication & Basic Commands

#### 1.1 Login Testing
```bash
# Test interactive login
guepard login

# Test direct code login
guepard login -c "your-verification-code"

# Test logout
guepard logout

# Re-login for subsequent tests
guepard login
```

#### 1.2 Usage Command
```bash
# Test usage information
guepard usage
```

### Phase 2: Deployment Management

#### 2.1 List Deployments
```bash
# List all deployments
guepard list deployments

# List with custom columns
guepard list deployments -c "id,name,status,provider"

# List with specific columns
guepard list deployments -c "id,name,repository,status,fqdn"
```

#### 2.2 Create Deployment
```bash
# Create PostgreSQL deployment
guepard deploy -p PostgreSQL -v 16 -r us-west -d aws -n $TEST_REPO_NAME -w "testpass123" -u guepard

# Create MySQL deployment
guepard deploy -p MySQL -v 8.0 -r us-east -d gcp -n "mysql-test-$(date +%s)" -w "testpass123"

# Create MongoDB deployment
guepard deploy -p MongoDB -v 7.0 -r eu-west -d azure -n "mongodb-test-$(date +%s)" -w "testpass123"

# Interactive deployment
guepard deploy -I

# Create with performance profile
guepard deploy -p PostgreSQL -v 16 -f gp.g1.small -n "perf-test-$(date +%s)" -w "testpass123"
```

#### 2.3 Get Deployment Details
```bash
# Get specific deployment (replace with actual ID)
guepard deploy -x $TEST_DEPLOYMENT_ID

# Test with non-existent deployment
guepard deploy -x "non-existent-id"
```

#### 2.4 Update Deployment
```bash
# Update repository name
guepard deploy -x $TEST_DEPLOYMENT_ID -n "updated-repo-name"

# Update with confirmation
guepard deploy -x $TEST_DEPLOYMENT_ID -n "updated-repo-name" -y
```

### Phase 3: Branch Management

#### 3.1 List Branches
```bash
# List branches for deployment
guepard branch -x $TEST_DEPLOYMENT_ID

# List branches without deployment ID (should show helpful message)
guepard branch

# List with branch name (should show helpful message)
guepard branch "feature-branch"
```

#### 3.2 Create Branches
```bash
# Create branch from snapshot
guepard branch -x $TEST_DEPLOYMENT_ID -s $TEST_SNAPSHOT_ID -n "feature-branch-$(date +%s)" -k -e

# Create ephemeral branch
guepard branch -x $TEST_DEPLOYMENT_ID -s $TEST_SNAPSHOT_ID -n "ephemeral-branch" -e

# Create branch with checkout
guepard branch -x $TEST_DEPLOYMENT_ID -s $TEST_SNAPSHOT_ID -n "checkout-branch" -k

# Create branch from source branch
guepard branch -x $TEST_DEPLOYMENT_ID -s $TEST_SNAPSHOT_ID -n "from-source" -b $TEST_BRANCH_ID
```

### Phase 4: Snapshot Management (Commit)

#### 4.1 Create Snapshots
```bash
# Create snapshot with message
guepard commit -m "Initial snapshot" -x $TEST_DEPLOYMENT_ID -b $TEST_BRANCH_ID

# Create snapshot with descriptive message
guepard commit -m "Feature implementation complete" -x $TEST_DEPLOYMENT_ID -b $TEST_BRANCH_ID

# Create snapshot with special characters
guepard commit -m "Fix: Handle edge cases & improve performance" -x $TEST_DEPLOYMENT_ID -b $TEST_BRANCH_ID
```

### Phase 5: Checkout Operations

#### 5.1 Checkout Branches
```bash
# Checkout to branch
guepard checkout "branch-name" -x $TEST_DEPLOYMENT_ID -c $TEST_BRANCH_ID

# Checkout to snapshot
guepard checkout "snapshot-hash" -x $TEST_DEPLOYMENT_ID -s $TEST_SNAPSHOT_ID

# Checkout with discard changes
guepard checkout "branch-name" -x $TEST_DEPLOYMENT_ID -c $TEST_BRANCH_ID -d "true"

# Checkout without deployment ID (should show helpful message)
guepard checkout "branch-name"
```

### Phase 6: Compute Management

#### 6.1 Compute Status
```bash
# Get compute status
guepard compute status -x $TEST_DEPLOYMENT_ID

# Get compute info (default action)
guepard compute -x $TEST_DEPLOYMENT_ID
```

#### 6.2 Compute Operations
```bash
# Start compute
guepard compute start -x $TEST_DEPLOYMENT_ID

# Stop compute
guepard compute stop -x $TEST_DEPLOYMENT_ID

# Restart compute
guepard compute restart -x $TEST_DEPLOYMENT_ID

# List compute details
guepard compute list -x $TEST_DEPLOYMENT_ID
```

### Phase 7: Logging & History

#### 7.1 View Logs
```bash
# View logs with default settings
guepard log -x $TEST_DEPLOYMENT_ID

# View limited number of lines
guepard log -x $TEST_DEPLOYMENT_ID -n 10

# Follow logs in real-time
guepard log -x $TEST_DEPLOYMENT_ID -f

# View only stdout logs
guepard log -x $TEST_DEPLOYMENT_ID --stdout-only

# View only stderr logs
guepard log -x $TEST_DEPLOYMENT_ID --stderr-only

# View logs with timestamps
guepard log -x $TEST_DEPLOYMENT_ID -t

# Filter logs by date
guepard log -x $TEST_DEPLOYMENT_ID --since "2024-01-01"
guepard log -x $TEST_DEPLOYMENT_ID --until "2024-12-31"
guepard log -x $TEST_DEPLOYMENT_ID --since "2024-01-01 10:00:00" --until "2024-01-01 18:00:00"
```

### Phase 8: List Operations

#### 8.1 List Deployments
```bash
# List all deployments
guepard list deployments

# List with custom columns
guepard list deployments -c "id,name,status"

# List with all available columns
guepard list deployments -c "id,name,repository,provider,version,status,fqdn,region,datacenter,created"
```

#### 8.2 List Branches
```bash
# List branches for deployment
guepard list branches -x $TEST_DEPLOYMENT_ID

# List branches without deployment ID (should fail gracefully)
guepard list branches
```

#### 8.3 List Commits/Snapshots
```bash
# List commits for deployment
guepard list commits -x $TEST_DEPLOYMENT_ID

# List commits with git graph
guepard list commits -x $TEST_DEPLOYMENT_ID -g

# List all commits including AUTO SNAPs
guepard list commits -x $TEST_DEPLOYMENT_ID -a

# List commits with graph and all
guepard list commits -x $TEST_DEPLOYMENT_ID -g -a
```

### Phase 9: Error Handling & Edge Cases

#### 9.1 Invalid Parameters
```bash
# Test with invalid deployment ID
guepard deploy -x "invalid-id"
guepard branch -x "invalid-id"
guepard commit -m "test" -x "invalid-id" -b "invalid-branch"
guepard compute status -x "invalid-id"

# Test with missing required parameters
guepard commit -x $TEST_DEPLOYMENT_ID  # Missing message and branch
guepard branch -x $TEST_DEPLOYMENT_ID  # Missing snapshot and name for creation

# Test with invalid database provider
guepard deploy -p "InvalidDB" -n "test" -w "pass"
```

#### 9.2 Network & Authentication Errors
```bash
# Test with invalid credentials
guepard logout
guepard usage  # Should fail

# Test with expired token (if applicable)
guepard usage
```

### Phase 10: Performance & Stress Testing

#### 10.1 Multiple Operations
```bash
# Create multiple deployments rapidly
for i in {1..5}; do
  guepard deploy -p PostgreSQL -v 16 -n "stress-test-$i" -w "testpass123" -y
done

# Create multiple branches
for i in {1..3}; do
  guepard branch -x $TEST_DEPLOYMENT_ID -s $TEST_SNAPSHOT_ID -n "stress-branch-$i"
done

# Create multiple snapshots
for i in {1..3}; do
  guepard commit -m "Stress test snapshot $i" -x $TEST_DEPLOYMENT_ID -b $TEST_BRANCH_ID
done
```

#### 10.2 Large Data Operations
```bash
# List with large number of deployments
guepard list deployments

# List with large number of branches
guepard list branches -x $TEST_DEPLOYMENT_ID

# List with large number of commits
guepard list commits -x $TEST_DEPLOYMENT_ID -a
```

## Test Data Cleanup

### Cleanup Commands
```bash
# List all test deployments
guepard list deployments | grep "test\|stress"

# Note: Manual cleanup required for deployments
# Use the Guepard web interface or API to delete test deployments
```

## Expected Results Summary

### Successful Operations Should Return:
- **Deployments**: Table with ID, name, repository, provider, version, status, FQDN
- **Branches**: Table with ID, name, status, snapshot, environment, ephemeral status
- **Commits**: List of snapshots with messages and timestamps
- **Compute**: Status information with instance details
- **Logs**: Formatted log output with timestamps
- **Usage**: Account usage statistics

### Error Cases Should Return:
- **Exit Code 1**: General errors
- **Exit Code 2**: Deployment errors
- **Exit Code 3**: Branch errors
- **Exit Code 4**: Bookmark/snapshot errors
- **Exit Code 5**: Compute errors
- **Exit Code 6**: Usage errors
- **Exit Code 7**: Login errors

## Automated Testing Script

Create a test script to automate this workflow:

```bash
#!/bin/bash
# save as test-workflow.sh

set -e

echo "🧪 Starting Guepard CLI Testing Workflow"

# Test authentication
echo "Testing authentication..."
guepard login
guepard usage

# Test basic commands
echo "Testing basic commands..."
guepard list deployments

# Create test deployment
echo "Creating test deployment..."
DEPLOYMENT_ID=$(guepard deploy -p PostgreSQL -v 16 -n "test-$(date +%s)" -w "testpass123" -y | grep -o 'ID: [a-f0-9-]*' | cut -d' ' -f2)

# Test deployment operations
echo "Testing deployment operations..."
guepard deploy -x $DEPLOYMENT_ID

# Test branch operations
echo "Testing branch operations..."
guepard branch -x $DEPLOYMENT_ID

# Create test branch
BRANCH_ID=$(guepard branch -x $DEPLOYMENT_ID -s $(guepard list commits -x $DEPLOYMENT_ID | head -2 | tail -1 | awk '{print $1}') -n "test-branch" | grep -o 'ID: [a-f0-9-]*' | cut -d' ' -f2)

# Test commit operations
echo "Testing commit operations..."
guepard commit -m "Test snapshot" -x $DEPLOYMENT_ID -b $BRANCH_ID

# Test compute operations
echo "Testing compute operations..."
guepard compute status -x $DEPLOYMENT_ID

# Test log operations
echo "Testing log operations..."
guepard log -x $DEPLOYMENT_ID -n 5

echo "✅ Testing workflow completed successfully!"
echo "Test deployment ID: $DEPLOYMENT_ID"
echo "Test branch ID: $BRANCH_ID"
```

## Notes

1. **Replace Placeholders**: Update `$TEST_DEPLOYMENT_ID`, `$TEST_BRANCH_ID`, and `$TEST_SNAPSHOT_ID` with actual values from your test environment.

2. **Rate Limiting**: Be mindful of API rate limits when running stress tests.

3. **Cleanup**: Always clean up test resources to avoid unnecessary costs.

4. **Environment**: Run tests in a dedicated test environment, not production.

5. **Documentation**: Update this workflow as new commands or features are added.

6. **CI/CD Integration**: This workflow can be integrated into CI/CD pipelines for automated testing.

This comprehensive testing workflow ensures all Guepard CLI commands and features are thoroughly tested across various scenarios and edge cases.
