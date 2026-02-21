# Workflow Guides and Best Practices

Learn proven workflows and best practices for using Guepard CLI effectively in different development scenarios.

## Table of Contents

- [Development Workflows](#development-workflows)
- [Team Collaboration](#team-collaboration)
- [CI/CD Integration](#cicd-integration)
- [Production Management](#production-management)
- [Database Migration Strategies](#database-migration-strategies)
- [Testing Strategies](#testing-strategies)
- [Monitoring and Maintenance](#monitoring-and-maintenance)
- [Best Practices Summary](#best-practices-summary)

## Development Workflows

### Feature Development Workflow

This is the recommended workflow for developing new features:

```bash
# 1. Start from main branch
guepard checkout --deployment-id <deployment_id> --branch-id main-branch-id

# 2. Create feature branch
guepard branch \
  --deployment-id <deployment_id> \
  --snapshot-id latest-main-snapshot-id \
  feature/user-authentication \
  --checkout \
  --ephemeral

# 3. Develop feature (make database changes via your application)
# Add tables, modify schema, etc.

# 4. Create intermediate commits
guepard commit \
  --message "Add user table and basic authentication" \
  --deployment-id <deployment_id> \
  --branch-id feature/user-authentication-branch-id

# 5. Continue development
guepard commit \
  --message "Add password reset functionality" \
  --deployment-id <deployment_id> \
  --branch-id feature/user-authentication-branch-id

# 6. Finalize feature
guepard commit \
  --message "Complete user authentication feature" \
  --deployment-id <deployment_id> \
  --branch-id feature/user-authentication-branch-id

# 7. Merge back to main (conceptually)
guepard checkout --deployment-id <deployment_id> --branch-id main-branch-id
guepard commit \
  --message "Merge user authentication feature" \
  --deployment-id <deployment_id> \
  --branch-id main-branch-id
```

### Hotfix Workflow

For urgent fixes in production:

```bash
# 1. Create hotfix branch from production snapshot
guepard branch \
  --deployment-id <deployment_id> \
  --snapshot-id production-snapshot-id \
  hotfix/critical-bug-fix \
  --checkout \
  --ephemeral

# 2. Apply fix
guepard commit \
  --message "Fix critical security vulnerability" \
  --deployment-id <deployment_id> \
  --branch-id hotfix/critical-bug-fix-branch-id

# 3. Deploy to production immediately
guepard checkout --deployment-id <deployment_id> --branch-id main-branch-id
guepard commit \
  --message "Deploy hotfix: critical security fix" \
  --deployment-id <deployment_id> \
  --branch-id main-branch-id

# 4. Merge back to develop
guepard checkout --deployment-id <deployment_id> --branch-id develop-branch-id
guepard commit \
  --message "Merge hotfix to develop" \
  --deployment-id <deployment_id> \
  --branch-id develop-branch-id
```

### Experimental Development

For testing new ideas without affecting main development:

```bash
# 1. Create experimental branch
guepard branch \
  --deployment-id <deployment_id> \
  --snapshot-id latest-snapshot-id \
  experiment/new-architecture \
  --checkout \
  --ephemeral

# 2. Try different approaches
guepard commit \
  --message "Experiment: try microservices approach" \
  --deployment-id <deployment_id> \
  --branch-id experiment/new-architecture-branch-id

# 3. If successful, merge to feature branch
# If not successful, simply discard the branch
```

## Team Collaboration

### Branch Naming Conventions

Establish consistent naming conventions for your team:

```bash
# Feature branches
feature/user-authentication
feature/payment-integration
feature/admin-dashboard

# Bug fix branches
bugfix/login-error
bugfix/payment-timeout
bugfix/data-validation

# Hotfix branches
hotfix/security-patch
hotfix/critical-bug

# Release branches
release/v1.2.0
release/v2.0.0-beta

# Experimental branches
experiment/new-database-design
experiment/performance-optimization
```

### Code Review Workflow

```bash
# 1. Developer creates feature branch
guepard branch \
  --deployment-id <deployment_id> \
  --snapshot-id latest-snapshot-id \
  feature/new-feature \
  --checkout \
  --ephemeral

# 2. Developer commits changes
guepard commit \
  --message "Implement new feature with tests" \
  --deployment-id <deployment_id> \
  --branch-id feature/new-feature-branch-id

# 3. Reviewer checks out the branch
guepard checkout \
  --deployment-id <deployment_id> \
  --branch-id feature/new-feature-branch-id

# 4. Reviewer tests the changes
# Run tests, check functionality, etc.

# 5. If approved, merge to develop
guepard checkout --deployment-id <deployment_id> --branch-id develop-branch-id
guepard commit \
  --message "Merge feature/new-feature (approved by reviewer)" \
  --deployment-id <deployment_id> \
  --branch-id develop-branch-id

# 6. If not approved, provide feedback
# Developer makes changes and repeats the process
```

### Conflict Resolution

When multiple developers work on the same deployment:

```bash
# 1. Check current state
guepard list branches --deployment-id <deployment_id>

# 2. Identify conflicts
# Look for overlapping changes in the same areas

# 3. Coordinate resolution
# Use ephemeral branches to test conflict resolution

guepard branch \
  --deployment-id <deployment_id> \
  --snapshot-id conflict-resolution-base-id \
  resolve/feature-conflict \
  --checkout \
  --ephemeral

# 4. Apply resolution
guepard commit \
  --message "Resolve conflict between feature A and B" \
  --deployment-id <deployment_id> \
  --branch-id resolve/feature-conflict-branch-id

# 5. Test resolution
# Verify that both features work together

# 6. Merge resolution
guepard checkout --deployment-id <deployment_id> --branch-id develop-branch-id
guepard commit \
  --message "Apply conflict resolution" \
  --deployment-id <deployment_id> \
  --branch-id develop-branch-id
```

## CI/CD Integration

### GitHub Actions Workflow

```yaml
name: Database Deployment Pipeline

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Guepard CLI
        run: |
          wget https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.27.17/guepard-cli-0.27.17-linux-amd64.tar.gz
          tar -xzf guepard-cli-0.27.17-linux-amd64.tar.gz
          sudo mv guepard /usr/local/bin/
      
      - name: Authenticate with Guepard
        run: guepard login --code ${{ secrets.GUEPARD_TOKEN }}
        env:
          GUEPARD_TOKEN: ${{ secrets.GUEPARD_TOKEN }}
      
      - name: Run Database Tests
        run: |
          # Create test branch
          guepard branch \
            --deployment-id ${{ secrets.TEST_DEPLOYMENT_ID }} \
            --snapshot-id ${{ secrets.TEST_SNAPSHOT_ID }} \
            test/${{ github.sha }} \
            --checkout \
            --ephemeral
          
          # Run your database tests here
          # This could be unit tests, integration tests, etc.
          
          # Commit test results
          guepard commit \
            --message "Test run for ${{ github.sha }}" \
            --deployment-id ${{ secrets.TEST_DEPLOYMENT_ID }} \
            --branch-id test/${{ github.sha }}-branch-id

  deploy-staging:
    needs: test
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/develop'
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Guepard CLI
        run: |
          wget https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.27.17/guepard-cli-0.27.17-linux-amd64.tar.gz
          tar -xzf guepard-cli-0.27.17-linux-amd64.tar.gz
          sudo mv guepard /usr/local/bin/
      
      - name: Deploy to Staging
        run: |
          guepard login --code ${{ secrets.GUEPARD_TOKEN }}
          guepard checkout \
            --deployment-id ${{ secrets.STAGING_DEPLOYMENT_ID }} \
            --branch-id develop-branch-id
          guepard commit \
            --message "Deploy to staging: ${{ github.sha }}" \
            --deployment-id ${{ secrets.STAGING_DEPLOYMENT_ID }} \
            --branch-id develop-branch-id
          guepard compute start --deployment-id ${{ secrets.STAGING_DEPLOYMENT_ID }}

  deploy-production:
    needs: test
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Guepard CLI
        run: |
          wget https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.27.17/guepard-cli-0.27.17-linux-amd64.tar.gz
          tar -xzf guepard-cli-0.27.17-linux-amd64.tar.gz
          sudo mv guepard /usr/local/bin/
      
      - name: Deploy to Production
        run: |
          guepard login --code ${{ secrets.GUEPARD_TOKEN }}
          guepard checkout \
            --deployment-id ${{ secrets.PROD_DEPLOYMENT_ID }} \
            --branch-id main-branch-id
          guepard commit \
            --message "Deploy to production: ${{ github.sha }}" \
            --deployment-id ${{ secrets.PROD_DEPLOYMENT_ID }} \
            --branch-id main-branch-id
          guepard compute start --deployment-id ${{ secrets.PROD_DEPLOYMENT_ID }}
```

### Jenkins Pipeline

```groovy
pipeline {
    agent any
    
    environment {
        GUEPARD_TOKEN = credentials('guepard-token')
        STAGING_DEPLOYMENT_ID = 'staging-12345678-1234-1234-1234-123456789abc'
        PROD_DEPLOYMENT_ID = 'prod-87654321-4321-4321-4321-210987654321'
    }
    
    stages {
        stage('Setup') {
            steps {
                sh '''
                    wget https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.27.17/guepard-cli-0.27.17-linux-amd64.tar.gz
                    tar -xzf guepard-cli-0.27.17-linux-amd64.tar.gz
                    sudo mv guepard /usr/local/bin/
                '''
            }
        }
        
        stage('Test') {
            steps {
                sh '''
                    guepard login --code $GUEPARD_TOKEN
                    guepard branch \
                      --deployment-id $STAGING_DEPLOYMENT_ID \
                      --snapshot-id test-base-snapshot-id \
                      test/$BUILD_NUMBER \
                      --checkout \
                      --ephemeral
                    # Run tests here
                    guepard commit \
                      --message "Test run for build $BUILD_NUMBER" \
                      --deployment-id $STAGING_DEPLOYMENT_ID \
                      --branch-id test/$BUILD_NUMBER-branch-id
                '''
            }
        }
        
        stage('Deploy Staging') {
            when {
                branch 'develop'
            }
            steps {
                sh '''
                    guepard checkout \
                      --deployment-id $STAGING_DEPLOYMENT_ID \
                      --branch-id develop-branch-id
                    guepard commit \
                      --message "Deploy to staging: build $BUILD_NUMBER" \
                      --deployment-id $STAGING_DEPLOYMENT_ID \
                      --branch-id develop-branch-id
                    guepard compute start --deployment-id $STAGING_DEPLOYMENT_ID
                '''
            }
        }
        
        stage('Deploy Production') {
            when {
                branch 'main'
            }
            steps {
                sh '''
                    guepard checkout \
                      --deployment-id $PROD_DEPLOYMENT_ID \
                      --branch-id main-branch-id
                    guepard commit \
                      --message "Deploy to production: build $BUILD_NUMBER" \
                      --deployment-id $PROD_DEPLOYMENT_ID \
                      --branch-id main-branch-id
                    guepard compute start --deployment-id $PROD_DEPLOYMENT_ID
                '''
            }
        }
    }
}
```

## Production Management

### Blue-Green Deployment

```bash
#!/bin/bash
# blue-green-deploy.sh

BLUE_DEPLOYMENT_ID="blue-11111111-2222-3333-4444-555555555555"
GREEN_DEPLOYMENT_ID="green-22222222-3333-4444-5555-666666666666"
CURRENT_COLOR=$1
NEW_VERSION=$2

if [ "$CURRENT_COLOR" = "blue" ]; then
    ACTIVE_DEPLOYMENT_ID=$BLUE_DEPLOYMENT_ID
    INACTIVE_DEPLOYMENT_ID=$GREEN_DEPLOYMENT_ID
    NEW_COLOR="green"
else
    ACTIVE_DEPLOYMENT_ID=$GREEN_DEPLOYMENT_ID
    INACTIVE_DEPLOYMENT_ID=$BLUE_DEPLOYMENT_ID
    NEW_COLOR="blue"
fi

echo "🔄 Starting blue-green deployment..."
echo "Current: $CURRENT_COLOR ($ACTIVE_DEPLOYMENT_ID)"
echo "Target: $NEW_COLOR ($INACTIVE_DEPLOYMENT_ID)"

# 1. Deploy to inactive environment
echo "📦 Deploying to $NEW_COLOR environment..."
guepard checkout \
  --deployment-id $INACTIVE_DEPLOYMENT_ID \
  --branch-id main-branch-id

guepard commit \
  --message "Deploy v$NEW_VERSION to $NEW_COLOR" \
  --deployment-id $INACTIVE_DEPLOYMENT_ID \
  --branch-id main-branch-id

# 2. Start inactive environment
echo "⚡ Starting $NEW_COLOR environment..."
guepard compute start --deployment-id $INACTIVE_DEPLOYMENT_ID

# 3. Run health checks
echo "🏥 Running health checks..."
sleep 30
# Add your health check logic here

# 4. Switch traffic (conceptually)
echo "🔄 Switching traffic to $NEW_COLOR..."
# This would typically involve updating load balancer configuration

# 5. Stop old environment
echo "⏹️ Stopping $CURRENT_COLOR environment..."
guepard compute stop --deployment-id $ACTIVE_DEPLOYMENT_ID

echo "✅ Blue-green deployment completed!"
echo "Active environment: $NEW_COLOR"
```

### Canary Deployment

```bash
#!/bin/bash
# canary-deploy.sh

PROD_DEPLOYMENT_ID="prod-12345678-1234-1234-1234-123456789abc"
CANARY_DEPLOYMENT_ID="canary-87654321-4321-4321-4321-210987654321"
NEW_VERSION=$1
CANARY_PERCENTAGE=$2

echo "🎯 Starting canary deployment for version $NEW_VERSION"
echo "Canary percentage: $CANARY_PERCENTAGE%"

# 1. Deploy to canary environment
echo "📦 Deploying to canary environment..."
guepard checkout \
  --deployment-id $CANARY_DEPLOYMENT_ID \
  --branch-id main-branch-id

guepard commit \
  --message "Deploy v$NEW_VERSION to canary" \
  --deployment-id $CANARY_DEPLOYMENT_ID \
  --branch-id main-branch-id

guepard compute start --deployment-id $CANARY_DEPLOYMENT_ID

# 2. Monitor canary performance
echo "📊 Monitoring canary performance..."
# Add monitoring logic here
sleep 300  # Monitor for 5 minutes

# 3. Check metrics
echo "📈 Checking canary metrics..."
# Add metric checking logic here

# 4. Decide whether to promote
read -p "Promote canary to production? (y/n): " promote

if [ "$promote" = "y" ]; then
    echo "🚀 Promoting canary to production..."
    
    # Deploy to production
    guepard checkout \
      --deployment-id $PROD_DEPLOYMENT_ID \
      --branch-id main-branch-id
    
    guepard commit \
      --message "Promote v$NEW_VERSION from canary to production" \
      --deployment-id $PROD_DEPLOYMENT_ID \
      --branch-id main-branch-id
    
    guepard compute start --deployment-id $PROD_DEPLOYMENT_ID
    
    echo "✅ Canary promoted to production!"
else
    echo "🔄 Rolling back canary..."
    guepard compute stop --deployment-id $CANARY_DEPLOYMENT_ID
    echo "❌ Canary deployment rolled back"
fi
```

## Database Migration Strategies

### Forward-Only Migrations

```bash
#!/bin/bash
# forward-migration.sh

DEPLOYMENT_ID=$1
MIGRATION_NAME=$2

echo "🔄 Running forward migration: $MIGRATION_NAME"

# 1. Create migration branch
guepard branch \
  --deployment-id $DEPLOYMENT_ID \
  --snapshot-id latest-snapshot-id \
  migration/$MIGRATION_NAME \
  --checkout \
  --ephemeral

# 2. Apply migration
# This would typically involve running SQL scripts or using a migration framework
echo "📝 Applying migration: $MIGRATION_NAME"
# Add your migration logic here

# 3. Commit migration
guepard commit \
  --message "Migration: $MIGRATION_NAME" \
  --deployment-id $DEPLOYMENT_ID \
  --branch-id migration/$MIGRATION_NAME-branch-id

# 4. Test migration
echo "🧪 Testing migration..."
# Add test logic here

# 5. Merge to main
guepard checkout --deployment-id $DEPLOYMENT_ID --branch-id main-branch-id
guepard commit \
  --message "Apply migration: $MIGRATION_NAME" \
  --deployment-id $DEPLOYMENT_ID \
  --branch-id main-branch-id

echo "✅ Migration completed successfully!"
```

### Reversible Migrations

```bash
#!/bin/bash
# reversible-migration.sh

DEPLOYMENT_ID=$1
MIGRATION_NAME=$2
ACTION=$3  # "forward" or "backward"

echo "🔄 Running $ACTION migration: $MIGRATION_NAME"

if [ "$ACTION" = "forward" ]; then
    # Forward migration
    guepard branch \
      --deployment-id $DEPLOYMENT_ID \
      --snapshot-id latest-snapshot-id \
      migration/$MIGRATION_NAME-forward \
      --checkout \
      --ephemeral
    
    # Apply forward migration
    echo "📝 Applying forward migration: $MIGRATION_NAME"
    # Add forward migration logic here
    
    guepard commit \
      --message "Forward migration: $MIGRATION_NAME" \
      --deployment-id $DEPLOYMENT_ID \
      --branch-id migration/$MIGRATION_NAME-forward-branch-id
    
else
    # Backward migration
    guepard branch \
      --deployment-id $DEPLOYMENT_ID \
      --snapshot-id pre-migration-snapshot-id \
      migration/$MIGRATION_NAME-backward \
      --checkout \
      --ephemeral
    
    # Apply backward migration
    echo "📝 Applying backward migration: $MIGRATION_NAME"
    # Add backward migration logic here
    
    guepard commit \
      --message "Backward migration: $MIGRATION_NAME" \
      --deployment-id $DEPLOYMENT_ID \
      --branch-id migration/$MIGRATION_NAME-backward-branch-id
fi

echo "✅ $ACTION migration completed!"
```

## Testing Strategies

### Test Environment Management

```bash
#!/bin/bash
# setup-test-environments.sh

echo "🧪 Setting up test environments..."

# 1. Unit Test Environment
guepard deploy \
  --database-provider PostgreSQL \
  --database-version 16 \
  --region us-west \
  --instance-type REPOSITORY \
  --datacenter aws \
  --repository-name unit-tests \
  --database-password unit_test_pass

UNIT_DEPLOYMENT_ID="unit-11111111-2222-3333-4444-555555555555"

# 2. Integration Test Environment
guepard deploy \
  --database-provider PostgreSQL \
  --database-version 16 \
  --region us-west \
  --instance-type REPOSITORY \
  --datacenter aws \
  --repository-name integration-tests \
  --database-password integration_test_pass

INTEGRATION_DEPLOYMENT_ID="integration-22222222-3333-4444-5555-666666666666"

# 3. E2E Test Environment
guepard deploy \
  --database-provider PostgreSQL \
  --database-version 16 \
  --region us-west \
  --instance-type REPOSITORY \
  --datacenter aws \
  --repository-name e2e-tests \
  --database-password e2e_test_pass

E2E_DEPLOYMENT_ID="e2e-33333333-4444-5555-6666-777777777777"

echo "✅ Test environments created:"
echo "Unit Tests: $UNIT_DEPLOYMENT_ID"
echo "Integration Tests: $INTEGRATION_DEPLOYMENT_ID"
echo "E2E Tests: $E2E_DEPLOYMENT_ID"
```

### Automated Testing Pipeline

```bash
#!/bin/bash
# run-test-suite.sh

UNIT_DEPLOYMENT_ID="unit-11111111-2222-3333-4444-555555555555"
INTEGRATION_DEPLOYMENT_ID="integration-22222222-3333-4444-5555-666666666666"
E2E_DEPLOYMENT_ID="e2e-33333333-4444-5555-6666-777777777777"
TEST_BRANCH="test/$BUILD_NUMBER"

echo "🧪 Running comprehensive test suite..."

# 1. Unit Tests
echo "🔬 Running unit tests..."
guepard branch \
  --deployment-id $UNIT_DEPLOYMENT_ID \
  --snapshot-id unit-test-base-snapshot-id \
  $TEST_BRANCH \
  --checkout \
  --ephemeral

# Run unit tests
# Add your unit test execution logic here

guepard commit \
  --message "Unit tests for build $BUILD_NUMBER" \
  --deployment-id $UNIT_DEPLOYMENT_ID \
  --branch-id $TEST_BRANCH-branch-id

echo "✅ Unit tests passed!"

# 2. Integration Tests
echo "🔗 Running integration tests..."
guepard branch \
  --deployment-id $INTEGRATION_DEPLOYMENT_ID \
  --snapshot-id integration-test-base-snapshot-id \
  $TEST_BRANCH \
  --checkout \
  --ephemeral

# Run integration tests
# Add your integration test execution logic here

guepard commit \
  --message "Integration tests for build $BUILD_NUMBER" \
  --deployment-id $INTEGRATION_DEPLOYMENT_ID \
  --branch-id $TEST_BRANCH-branch-id

echo "✅ Integration tests passed!"

# 3. End-to-End Tests
echo "🎯 Running end-to-end tests..."
guepard branch \
  --deployment-id $E2E_DEPLOYMENT_ID \
  --snapshot-id e2e-test-base-snapshot-id \
  $TEST_BRANCH \
  --checkout \
  --ephemeral

# Run E2E tests
# Add your E2E test execution logic here

guepard commit \
  --message "E2E tests for build $BUILD_NUMBER" \
  --deployment-id $E2E_DEPLOYMENT_ID \
  --branch-id $TEST_BRANCH-branch-id

echo "✅ End-to-end tests passed!"

echo "🎉 All tests completed successfully!"
```

## Monitoring and Maintenance

### Health Monitoring

```bash
#!/bin/bash
# health-check.sh

DEPLOYMENT_ID=$1

echo "🏥 Running health check for deployment: $DEPLOYMENT_ID"

# 1. Check compute status
echo "📊 Checking compute status..."
guepard compute status --deployment-id $DEPLOYMENT_ID

# 2. Check recent logs
echo "📋 Checking recent logs..."
guepard log --deployment-id $DEPLOYMENT_ID --lines 20

# 3. Check usage
echo "📈 Checking usage..."
guepard usage

# 4. Check for errors in logs
echo "🔍 Checking for errors..."
guepard log --deployment-id $DEPLOYMENT_ID --stderr-only --lines 50

echo "✅ Health check completed!"
```

### Maintenance Window

```bash
#!/bin/bash
# maintenance-window.sh

DEPLOYMENT_ID=$1
MAINTENANCE_TYPE=$2  # "backup", "update", "cleanup"

echo "🔧 Starting maintenance window: $MAINTENANCE_TYPE"

# 1. Create maintenance snapshot
guepard commit \
  --message "Pre-maintenance snapshot: $MAINTENANCE_TYPE" \
  --deployment-id $DEPLOYMENT_ID \
  --branch-id main-branch-id

# 2. Stop compute
echo "⏹️ Stopping compute for maintenance..."
guepard compute stop --deployment-id $DEPLOYMENT_ID

# 3. Perform maintenance
case $MAINTENANCE_TYPE in
    "backup")
        echo "💾 Performing backup maintenance..."
        # Add backup logic here
        ;;
    "update")
        echo "🔄 Performing update maintenance..."
        # Add update logic here
        ;;
    "cleanup")
        echo "🧹 Performing cleanup maintenance..."
        # Add cleanup logic here
        ;;
esac

# 4. Restart compute
echo "⚡ Restarting compute after maintenance..."
guepard compute start --deployment-id $DEPLOYMENT_ID

# 5. Verify health
echo "🏥 Verifying system health..."
sleep 30
guepard compute status --deployment-id $DEPLOYMENT_ID

echo "✅ Maintenance window completed!"
```

## Best Practices Summary

### Development Best Practices

1. **Use descriptive commit messages**
   ```bash
   # Good
   guepard commit -m "Add user authentication with JWT tokens"
   
   # Bad
   guepard commit -m "fix"
   ```

2. **Create branches for all changes**
   ```bash
   # Always create a branch for new work
   guepard branch -x <deployment_id> -s <snapshot_id> feature/new-feature -k -e
   ```

3. **Test changes in isolation**
   ```bash
   # Use ephemeral branches for experiments
   guepard branch -x <deployment_id> -s <snapshot_id> experiment/new-idea -e
   ```

4. **Keep commits atomic**
   ```bash
   # Each commit should represent one logical change
   guepard commit -m "Add user table"
   guepard commit -m "Add user authentication"
   guepard commit -m "Add user permissions"
   ```

### Team Collaboration Best Practices

1. **Establish naming conventions**
   ```bash
   feature/user-authentication
   bugfix/login-error
   hotfix/security-patch
   release/v1.2.0
   ```

2. **Coordinate branch usage**
   ```bash
   # Check what branches exist before creating new ones
   guepard branch -x <deployment_id>
   ```

3. **Document deployment procedures**
   ```bash
   # Create scripts for common operations
   ./deploy-staging.sh
   ./deploy-production.sh
   ```

4. **Implement code review process**
   ```bash
   # Review changes before merging
   guepard checkout -x <deployment_id> -c <feature_branch_id>
   # Test and review, then merge
   ```

### Production Best Practices

1. **Always test in staging first**
   ```bash
   # Deploy to staging
   guepard checkout -x <staging_deployment_id> -c <branch_id>
   guepard commit -m "Deploy to staging" -x <staging_deployment_id> -b <branch_id>
   
   # Test thoroughly, then deploy to production
   guepard checkout -x <prod_deployment_id> -c <branch_id>
   guepard commit -m "Deploy to production" -x <prod_deployment_id> -b <branch_id>
   ```

2. **Keep production snapshots**
   ```bash
   # Create snapshots before major changes
   guepard commit -m "Pre-deployment snapshot" -x <prod_deployment_id> -b <branch_id>
   ```

3. **Monitor compute resources**
   ```bash
   # Check status regularly
   guepard compute status -x <deployment_id>
   guepard usage
   ```

4. **Have rollback procedures ready**
   ```bash
   # Know how to rollback quickly
   ./rollback.sh <deployment_id> <previous_snapshot_id>
   ```

### Security Best Practices

1. **Use strong passwords**
   ```bash
   # Generate secure passwords for deployments
   guepard deploy -p PostgreSQL -v 16 -r us-west -d aws -n myapp -w $(openssl rand -base64 32)
   ```

2. **Rotate credentials regularly**
   ```bash
   # Update passwords periodically
   guepard deploy -x <deployment_id> -w $(openssl rand -base64 32)
   ```

3. **Monitor access logs**
   ```bash
   # Check logs for suspicious activity
   guepard log -x <deployment_id> --since "2025-01-01" --stderr-only
   ```

4. **Use environment variables for CI/CD**
   ```bash
   # Never hardcode tokens in scripts
   export GUEPARD_TOKEN="your-token"
   guepard login --code $GUEPARD_TOKEN
   ```

---

*These workflows and best practices will help you use Guepard CLI effectively in any development scenario. Adapt them to your specific needs and team requirements! 🐆*
