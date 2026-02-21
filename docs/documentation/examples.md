# Real-World Examples

Explore practical examples and use cases for Guepard CLI in real-world scenarios.

## Table of Contents

- [E-commerce Application](#e-commerce-application)
- [SaaS Multi-tenant System](#saas-multi-tenant-system)
- [Development Team Workflow](#development-team-workflow)
- [Production Deployment Pipeline](#production-deployment-pipeline)
- [Database Migration Management](#database-migration-management)
- [Testing and QA Workflow](#testing-and-qa-workflow)
- [Disaster Recovery](#disaster-recovery)
- [Microservices Architecture](#microservices-architecture)

## E-commerce Application

### Scenario
Building an e-commerce platform with user management, product catalog, orders, and payments.

### Initial Setup

```bash
# 1. Create production database
guepard deploy \
  --database-provider PostgreSQL \
  --database-version 16 \
  --region us-west \
  --instance-type REPOSITORY \
  --datacenter aws \
  --repository-name ecommerce-prod \
  --database-password secure_password_123

# Or with node ID
guepard deploy \
  --database-provider PostgreSQL \
  --database-version 17 \
  --region us-west-aws \
  --instance-type REPOSITORY \
  --datacenter us-west-aws \
  --repository-name ecommerce-prod \
  --database-password secure_password_123 \
  --node-id <node_id>

# Or using short flags
guepard deploy -p PostgreSQL -v 17 -r us-west-aws -i REPOSITORY -d us-west-aws -n ecommerce-prod -w secure_password_123 -s <node_id>

# Output: Deployment ID: prod-12345678-1234-1234-1234-123456789abc
```

### Development Workflow

```bash
# 2. Create initial schema snapshot
guepard commit \
  --message "Initial e-commerce schema: users, products, orders" \
  --deployment-id prod-12345678-1234-1234-1234-123456789abc \
  --branch-id main-branch-id

# 3. Create feature branch for user authentication
guepard branch \
  --deployment-id prod-12345678-1234-1234-1234-123456789abc \
  --snapshot-id initial-schema-snapshot-id \
  feature-auth \
  --checkout \
  --ephemeral

# 4. Develop authentication features (via your app)
# Add tables: user_sessions, password_resets, etc.

# 5. Commit authentication changes
guepard commit \
  --message "Add user authentication: sessions, password reset" \
  --deployment-id prod-12345678-1234-1234-1234-123456789abc \
  --branch-id feature-auth-branch-id

# 6. Create branch for payment integration
guepard branch \
  --deployment-id prod-12345678-1234-1234-1234-123456789abc \
  --snapshot-id auth-snapshot-id \
  feature-payments \
  --checkout \
  --ephemeral

# 7. Add payment tables and commit
guepard commit \
  --message "Add payment processing: transactions, refunds" \
  --deployment-id prod-12345678-1234-1234-1234-123456789abc \
  --branch-id feature-payments-branch-id
```

### Production Deployment

```bash
# 8. Merge features to main (conceptually)
guepard checkout \
  --deployment-id prod-12345678-1234-1234-1234-123456789abc \
  --branch-id main-branch-id

# 9. Create production snapshot
guepard commit \
  --message "Release v1.0: Complete e-commerce platform" \
  --deployment-id prod-12345678-1234-1234-1234-123456789abc \
  --branch-id main-branch-id

# 10. Start production compute
guepard compute start --deployment-id prod-12345678-1234-1234-1234-123456789abc
```

## SaaS Multi-tenant System

### Scenario
Building a SaaS application with tenant isolation and data segregation.

### Multi-tenant Setup

```bash
# 1. Create main SaaS database
guepard deploy \
  --database-provider PostgreSQL \
  --database-version 16 \
  --region us-east \
  --instance-type REPOSITORY \
  --datacenter aws \
  --repository-name saas-platform \
  --database-password saas_secure_pass

# Output: Deployment ID: saas-87654321-4321-4321-4321-210987654321
```

### Tenant Management Workflow

```bash
# 2. Create base tenant schema
guepard commit \
  --message "Base tenant schema: tenants, users, subscriptions" \
  --deployment-id saas-87654321-4321-4321-4321-210987654321 \
  --branch-id main-branch-id

# 3. Create branch for tenant A
guepard branch \
  --deployment-id saas-87654321-4321-4321-4321-210987654321 \
  --snapshot-id base-tenant-schema-id \
  tenant-acme-corp \
  --checkout

# 4. Add tenant-specific data
guepard commit \
  --message "Tenant ACME Corp: custom fields, workflows" \
  --deployment-id saas-87654321-4321-4321-4321-210987654321 \
  --branch-id tenant-acme-corp-branch-id

# 5. Create branch for tenant B
guepard branch \
  --deployment-id saas-87654321-4321-4321-4321-210987654321 \
  --snapshot-id base-tenant-schema-id \
  tenant-tech-startup \
  --checkout

# 6. Add different tenant configuration
guepard commit \
  --message "Tenant TechStartup: simplified schema, integrations" \
  --deployment-id saas-87654321-4321-4321-4321-210987654321 \
  --branch-id tenant-tech-startup-branch-id
```

### Tenant Onboarding Automation

```bash
#!/bin/bash
# tenant-onboard.sh

TENANT_NAME=$1
DEPLOYMENT_ID="saas-87654321-4321-4321-4321-210987654321"
BASE_SNAPSHOT_ID="base-tenant-schema-id"

# Create new tenant branch
guepard branch \
  --deployment-id $DEPLOYMENT_ID \
  --snapshot-id $BASE_SNAPSHOT_ID \
  "tenant-$TENANT_NAME" \
  --checkout

# Add tenant-specific configuration
guepard commit \
  --message "Onboard tenant $TENANT_NAME: initial setup" \
  --deployment-id $DEPLOYMENT_ID \
  --branch-id "tenant-$TENANT_NAME-branch-id"

echo "✅ Tenant $TENANT_NAME onboarded successfully!"
```

## Development Team Workflow

### Scenario
A development team working on a web application with multiple developers and environments.

### Team Setup

```bash
# 1. Lead developer creates main deployment
guepard deploy \
  --database-provider PostgreSQL \
  --database-version 16 \
  --region us-west \
  --instance-type REPOSITORY \
  --datacenter aws \
  --repository-name team-project \
  --database-password team_password_2025

# Output: Deployment ID: team-11111111-2222-3333-4444-555555555555
```

### Developer Workflow

```bash
# 2. Create development branch
guepard branch \
  --deployment-id team-11111111-2222-3333-4444-555555555555 \
  --snapshot-id initial-schema-id \
  develop \
  --checkout

# 3. Developer Alice works on user profiles
guepard branch \
  --deployment-id team-11111111-2222-3333-4444-555555555555 \
  --snapshot-id develop-snapshot-id \
  feature/user-profiles-alice \
  --checkout \
  --ephemeral

# 4. Alice commits her work
guepard commit \
  --message "Add user profile management: avatars, preferences" \
  --deployment-id team-11111111-2222-3333-4444-555555555555 \
  --branch-id feature/user-profiles-alice-branch-id

# 5. Developer Bob works on notifications
guepard branch \
  --deployment-id team-11111111-2222-3333-4444-555555555555 \
  --snapshot-id develop-snapshot-id \
  feature/notifications-bob \
  --checkout \
  --ephemeral

# 6. Bob commits his work
guepard commit \
  --message "Add notification system: email, push, in-app" \
  --deployment-id team-11111111-2222-3333-4444-555555555555 \
  --branch-id feature/notifications-bob-branch-id
```

### Code Review and Integration

```bash
# 7. Review Alice's changes
guepard checkout \
  --deployment-id team-11111111-2222-3333-4444-555555555555 \
  --branch-id feature/user-profiles-alice-branch-id

# Test Alice's changes, then merge to develop
guepard checkout \
  --deployment-id team-11111111-2222-3333-4444-555555555555 \
  --branch-id develop-branch-id

guepard commit \
  --message "Merge user profiles feature from Alice" \
  --deployment-id team-11111111-2222-3333-4444-555555555555 \
  --branch-id develop-branch-id

# 8. Review Bob's changes
guepard checkout \
  --deployment-id team-11111111-2222-3333-4444-555555555555 \
  --branch-id feature/notifications-bob-branch-id

# Test Bob's changes, then merge to develop
guepard checkout \
  --deployment-id team-11111111-2222-3333-4444-555555555555 \
  --branch-id develop-branch-id

guepard commit \
  --message "Merge notifications feature from Bob" \
  --deployment-id team-11111111-2222-3333-4444-555555555555 \
  --branch-id develop-branch-id
```

## Production Deployment Pipeline

### Scenario
Automated deployment pipeline for a production application with staging and production environments.

### Pipeline Setup

```bash
# 1. Create staging deployment
guepard deploy \
  --database-provider PostgreSQL \
  --database-version 16 \
  --region us-west \
  --instance-type REPOSITORY \
  --datacenter aws \
  --repository-name app-staging \
  --database-password staging_pass_123

# Output: Deployment ID: staging-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee

# 2. Create production deployment
guepard deploy \
  --database-provider PostgreSQL \
  --database-version 16 \
  --region us-west \
  --instance-type REPOSITORY \
  --datacenter aws \
  --repository-name app-production \
  --database-password prod_secure_pass_456

# Output: Deployment ID: prod-bbbbbbbb-cccc-dddd-eeee-ffffffffffff
```

### Automated Deployment Script

```bash
#!/bin/bash
# deploy.sh

STAGING_DEPLOYMENT_ID="staging-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee"
PROD_DEPLOYMENT_ID="prod-bbbbbbbb-cccc-dddd-eeee-ffffffffffff"
VERSION=$1

echo "🚀 Starting deployment pipeline for version $VERSION"

# 1. Deploy to staging
echo "📋 Deploying to staging..."
guepard checkout \
  --deployment-id $STAGING_DEPLOYMENT_ID \
  --branch-id develop-branch-id

guepard commit \
  --message "Deploy v$VERSION to staging" \
  --deployment-id $STAGING_DEPLOYMENT_ID \
  --branch-id develop-branch-id

# 2. Run staging tests (simulated)
echo "🧪 Running staging tests..."
sleep 5
echo "✅ Staging tests passed!"

# 3. Deploy to production
echo "🏭 Deploying to production..."
guepard checkout \
  --deployment-id $PROD_DEPLOYMENT_ID \
  --branch-id main-branch-id

guepard commit \
  --message "Release v$VERSION to production" \
  --deployment-id $PROD_DEPLOYMENT_ID \
  --branch-id main-branch-id

# 4. Start production compute
echo "⚡ Starting production compute..."
guepard compute start --deployment-id $PROD_DEPLOYMENT_ID

echo "✅ Deployment pipeline completed successfully!"
```

### Rollback Procedure

```bash
#!/bin/bash
# rollback.sh

PROD_DEPLOYMENT_ID="prod-bbbbbbbb-cccc-dddd-eeee-ffffffffffff"
PREVIOUS_SNAPSHOT_ID=$1

echo "🔄 Rolling back to snapshot: $PREVIOUS_SNAPSHOT_ID"

# 1. Stop production compute
guepard compute stop --deployment-id $PROD_DEPLOYMENT_ID

# 2. Checkout previous snapshot
guepard checkout \
  --deployment-id $PROD_DEPLOYMENT_ID \
  --snapshot-id $PREVIOUS_SNAPSHOT_ID

# 3. Create rollback commit
guepard commit \
  --message "Rollback to previous stable version" \
  --deployment-id $PROD_DEPLOYMENT_ID \
  --branch-id main-branch-id

# 4. Restart production compute
guepard compute start --deployment-id $PROD_DEPLOYMENT_ID

echo "✅ Rollback completed successfully!"
```

## Database Migration Management

### Scenario
Managing database schema migrations for a growing application.

### Migration Workflow

```bash
# 1. Create migration branch
guepard branch \
  --deployment-id app-12345678-1234-1234-1234-123456789abc \
  --snapshot-id current-schema-id \
  migration/add-user-roles \
  --checkout \
  --ephemeral

# 2. Apply migration (via your migration tool)
# This would typically involve running SQL scripts or using a migration framework

# 3. Commit migration
guepard commit \
  --message "Migration: Add user roles and permissions system" \
  --deployment-id app-12345678-1234-1234-1234-123456789abc \
  --branch-id migration/add-user-roles-branch-id

# 4. Test migration
guepard checkout \
  --deployment-id app-12345678-1234-1234-1234-123456789abc \
  --branch-id migration/add-user-roles-branch-id

# Run tests, then merge to main
guepard checkout \
  --deployment-id app-12345678-1234-1234-1234-123456789abc \
  --branch-id main-branch-id

guepard commit \
  --message "Apply migration: user roles system" \
  --deployment-id app-12345678-1234-1234-1234-123456789abc \
  --branch-id main-branch-id
```

### Migration Rollback

```bash
# 1. Create rollback branch from previous snapshot
guepard branch \
  --deployment-id app-12345678-1234-1234-1234-123456789abc \
  --snapshot-id pre-migration-snapshot-id \
  rollback/user-roles \
  --checkout \
  --ephemeral

# 2. Apply rollback changes
guepard commit \
  --message "Rollback: Remove user roles system" \
  --deployment-id app-12345678-1234-1234-1234-123456789abc \
  --branch-id rollback/user-roles-branch-id

# 3. Merge rollback to main
guepard checkout \
  --deployment-id app-12345678-1234-1234-1234-123456789abc \
  --branch-id main-branch-id

guepard commit \
  --message "Apply rollback: user roles system" \
  --deployment-id app-12345678-1234-1234-1234-123456789abc \
  --branch-id main-branch-id
```

## Testing and QA Workflow

### Scenario
Comprehensive testing workflow with multiple test environments.

### Test Environment Setup

```bash
# 1. Create test environments
guepard deploy \
  --database-provider PostgreSQL \
  --database-version 16 \
  --region us-west \
  --instance-type REPOSITORY \
  --datacenter aws \
  --repository-name app-unit-tests \
  --database-password test_pass_123

guepard deploy \
  --database-provider PostgreSQL \
  --database-version 16 \
  --region us-west \
  --instance-type REPOSITORY \
  --datacenter aws \
  --repository-name app-integration-tests \
  --database-password test_pass_456

guepard deploy \
  --database-provider PostgreSQL \
  --database-version 16 \
  --region us-west \
  --instance-type REPOSITORY \
  --datacenter aws \
  --repository-name app-e2e-tests \
  --database-password test_pass_789
```

### Test Execution Workflow

```bash
#!/bin/bash
# run-tests.sh

UNIT_DEPLOYMENT_ID="unit-11111111-2222-3333-4444-555555555555"
INTEGRATION_DEPLOYMENT_ID="integration-22222222-3333-4444-5555-666666666666"
E2E_DEPLOYMENT_ID="e2e-33333333-4444-5555-6666-777777777777"

echo "🧪 Starting comprehensive test suite..."

# 1. Unit Tests
echo "🔬 Running unit tests..."
guepard checkout \
  --deployment-id $UNIT_DEPLOYMENT_ID \
  --branch-id test-branch-id

guepard commit \
  --message "Unit tests: test data setup" \
  --deployment-id $UNIT_DEPLOYMENT_ID \
  --branch-id test-branch-id

# Run unit tests (simulated)
echo "✅ Unit tests passed!"

# 2. Integration Tests
echo "🔗 Running integration tests..."
guepard checkout \
  --deployment-id $INTEGRATION_DEPLOYMENT_ID \
  --branch-id test-branch-id

guepard commit \
  --message "Integration tests: full system test" \
  --deployment-id $INTEGRATION_DEPLOYMENT_ID \
  --branch-id test-branch-id

# Run integration tests (simulated)
echo "✅ Integration tests passed!"

# 3. End-to-End Tests
echo "🎯 Running end-to-end tests..."
guepard checkout \
  --deployment-id $E2E_DEPLOYMENT_ID \
  --branch-id test-branch-id

guepard commit \
  --message "E2E tests: complete user journey" \
  --deployment-id $E2E_DEPLOYMENT_ID \
  --branch-id test-branch-id

# Run E2E tests (simulated)
echo "✅ End-to-end tests passed!"

echo "🎉 All tests completed successfully!"
```

## Disaster Recovery

### Scenario
Implementing disaster recovery procedures for critical applications.

### Backup and Recovery Setup

```bash
# 1. Create backup deployment
guepard deploy \
  --database-provider PostgreSQL \
  --database-version 16 \
  --region us-east \
  --instance-type REPOSITORY \
  --datacenter aws \
  --repository-name app-backup \
  --database-password backup_secure_pass

# Output: Deployment ID: backup-44444444-5555-6666-7777-888888888888
```

### Disaster Recovery Procedure

```bash
#!/bin/bash
# disaster-recovery.sh

PROD_DEPLOYMENT_ID="prod-12345678-1234-1234-1234-123456789abc"
BACKUP_DEPLOYMENT_ID="backup-44444444-5555-6666-7777-888888888888"
LATEST_SNAPSHOT_ID=$1

echo "🚨 Starting disaster recovery procedure..."

# 1. Stop production compute
echo "⏹️ Stopping production compute..."
guepard compute stop --deployment-id $PROD_DEPLOYMENT_ID

# 2. Create emergency snapshot from backup
echo "📸 Creating emergency snapshot..."
guepard checkout \
  --deployment-id $BACKUP_DEPLOYMENT_ID \
  --snapshot-id $LATEST_SNAPSHOT_ID

guepard commit \
  --message "Emergency recovery snapshot" \
  --deployment-id $BACKUP_DEPLOYMENT_ID \
  --branch-id emergency-branch-id

# 3. Restore to production
echo "🔄 Restoring production from backup..."
guepard checkout \
  --deployment-id $PROD_DEPLOYMENT_ID \
  --snapshot-id $LATEST_SNAPSHOT_ID

guepard commit \
  --message "Disaster recovery: restored from backup" \
  --deployment-id $PROD_DEPLOYMENT_ID \
  --branch-id main-branch-id

# 4. Restart production compute
echo "⚡ Restarting production compute..."
guepard compute start --deployment-id $PROD_DEPLOYMENT_ID

echo "✅ Disaster recovery completed successfully!"
```

## Microservices Architecture

### Scenario
Managing multiple microservices with separate databases.

### Microservices Setup

```bash
# 1. User Service Database
guepard deploy \
  --database-provider PostgreSQL \
  --database-version 16 \
  --region us-west \
  --instance-type REPOSITORY \
  --datacenter aws \
  --repository-name user-service \
  --database-password user_service_pass

# Output: Deployment ID: user-55555555-6666-7777-8888-999999999999

# 2. Order Service Database
guepard deploy \
  --database-provider PostgreSQL \
  --database-version 16 \
  --region us-west \
  --instance-type REPOSITORY \
  --datacenter aws \
  --repository-name order-service \
  --database-password order_service_pass

# Output: Deployment ID: order-66666666-7777-8888-9999-aaaaaaaaaaaa

# 3. Payment Service Database
guepard deploy \
  --database-provider PostgreSQL \
  --database-version 16 \
  --region us-west \
  --instance-type REPOSITORY \
  --datacenter aws \
  --repository-name payment-service \
  --database-password payment_service_pass

# Output: Deployment ID: payment-77777777-8888-9999-aaaa-bbbbbbbbbbbb
```

### Microservice Deployment Script

```bash
#!/bin/bash
# deploy-microservices.sh

USER_DEPLOYMENT_ID="user-55555555-6666-7777-8888-999999999999"
ORDER_DEPLOYMENT_ID="order-66666666-7777-8888-9999-aaaaaaaaaaaa"
PAYMENT_DEPLOYMENT_ID="payment-77777777-8888-9999-aaaa-bbbbbbbbbbbb"
VERSION=$1

echo "🚀 Deploying microservices version $VERSION..."

# Deploy User Service
echo "👤 Deploying User Service..."
guepard commit \
  --message "Deploy User Service v$VERSION" \
  --deployment-id $USER_DEPLOYMENT_ID \
  --branch-id main-branch-id

guepard compute start --deployment-id $USER_DEPLOYMENT_ID

# Deploy Order Service
echo "📦 Deploying Order Service..."
guepard commit \
  --message "Deploy Order Service v$VERSION" \
  --deployment-id $ORDER_DEPLOYMENT_ID \
  --branch-id main-branch-id

guepard compute start --deployment-id $ORDER_DEPLOYMENT_ID

# Deploy Payment Service
echo "💳 Deploying Payment Service..."
guepard commit \
  --message "Deploy Payment Service v$VERSION" \
  --deployment-id $PAYMENT_DEPLOYMENT_ID \
  --branch-id main-branch-id

guepard compute start --deployment-id $PAYMENT_DEPLOYMENT_ID

echo "✅ All microservices deployed successfully!"
```

## Best Practices Summary

### General Guidelines
1. **Use descriptive commit messages** that explain what changed
2. **Create branches for experiments** using ephemeral branches
3. **Test changes in isolation** before merging to main
4. **Monitor usage regularly** with `guepard usage`
5. **Keep snapshots frequent** to enable easy rollbacks

### Team Collaboration
1. **Coordinate branch usage** to avoid conflicts
2. **Use consistent naming conventions** for branches
3. **Document deployment procedures** for your team
4. **Implement automated testing** in your pipeline
5. **Plan for rollbacks** before deploying

### Production Considerations
1. **Always test in staging** before production
2. **Keep production snapshots** for disaster recovery
3. **Monitor compute resources** and performance
4. **Implement monitoring and alerting** for your deployments
5. **Have rollback procedures** ready and tested

---

*These examples show the power and flexibility of Guepard CLI in real-world scenarios. Adapt them to your specific needs and workflows! 🐆*
