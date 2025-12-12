# Deployments

Learn how to create, manage, and configure database deployments with Guepard CLI.

## What are Deployments?

A **deployment** is a database instance managed by Guepard. It's your primary workspace for database version control, similar to how a Git repository is your workspace for code.

### Deployment Components

Each deployment consists of:

- **Database Engine**: PostgreSQL, MySQL, or MongoDB
- **Version**: Specific database version (e.g., PostgreSQL 16, MySQL 8.0)
- **Infrastructure**: Region, cloud provider, and performance profile
- **Access**: Connection details and authentication
- **State Management**: Snapshots, branches, and version history

## Creating Deployments

### Interactive Deployment (Recommended)

The easiest way to create a deployment is using interactive mode:

```bash
guepard deploy --interactive
```

This guides you through each configuration step:

```
🐆 Welcome to Interactive Deployment! 🚀
💡 Let's create your database deployment step by step.

1️⃣ Step 1: Choose Database Provider
Available options: PostgreSQL, MySQL, MongoDB
🔧 Database Provider [PostgreSQL]: 

2️⃣ Step 2: Choose Database Version
🔧 Database Version [16]: 

3️⃣ Step 3: Choose Region
Available options: us-west, us-east, eu-west, asia-pacific
🌍 Region [us-west]: 

4️⃣ Step 4: Choose Deployment Type
Available options: REPOSITORY, F2
🏗️ Deployment Type [REPOSITORY]: 

5️⃣ Step 5: Choose Datacenter
Available options: aws, gcp, azure
🏢 Datacenter [aws]: 

6️⃣ Step 6: Repository Name
📁 Repository Name [my-database]: 

7️⃣ Step 7: Database Password
🔐 Database Password: 

8️⃣ Step 8: Database Username
👤 Database Username [guepard]: 

9️⃣ Step 9: Performance Profile
Available options: gp.g1.xsmall, gp.g1.small, gp.g1.medium, gp.g1.large
⚡ Performance Profile [gp.g1.xsmall]: 
```

### Command-Line Deployment

For automation or advanced users, use command-line flags:

```bash
guepard deploy \
  --database-provider PostgreSQL \
  --database-version 16 \
  --region us-west \
  --instance-type REPOSITORY \
  --datacenter aws \
  --repository-name my-app-db \
  --database-password secure_password_123 \
  --user guepard \
  --performance-profile gp.g1.small
```

**Create deployment with node ID:**
```bash
guepard deploy \
  --database-provider PostgreSQL \
  --database-version 17 \
  --region us-west-aws \
  --instance-type REPOSITORY \
  --datacenter us-west-aws \
  --repository-name db-new-api \
  --database-password guepard \
  --node-id <node_id>
```

**Or using short flags:**
```bash
guepard deploy -p PostgreSQL -v 17 -r us-west-aws -i REPOSITORY -d us-west-aws -n db-new-api -w guepard -s <node_id>
```

**Get deployment details as JSON:**
```bash
guepard deploy --deployment-id <id> --json
```

### Deployment Parameters

| Parameter | Short | Description | Required | Options |
|-----------|-------|-------------|----------|---------|
| `--database-provider` | `-p` | Database type | Yes | PostgreSQL, MySQL, MongoDB |
| `--database-version` | `-v` | Database version | Yes | 16, 15, 14 (PostgreSQL)<br>8.0, 5.7 (MySQL)<br>7.0, 6.0 (MongoDB) |
| `--region` | `-r` | Geographic region | Yes | us-west, us-east, eu-west, asia-pacific |
| `--instance-type` | `-i` | Deployment type | Yes | REPOSITORY (versioning)<br>F2 (compute) |
| `--datacenter` | `-d` | Cloud provider | Yes | aws, gcp, azure |
| `--repository-name` | `-n` | Repository name | Yes | Alphanumeric, hyphens allowed |
| `--database-password` | `-w` | Database password | Yes | Strong password recommended |
| `--user` | `-u` | Database username | No | Default: guepard |
| `--performance-profile` | `-f` | Performance tier | No | gp.g1.xsmall, gp.g1.small, gp.g1.medium, gp.g1.large |
| `--node-id` | `-s` | Node ID for deployment | No | Optional node identifier |
| `--json` | | Output results as JSON | No | Machine-readable output |

## Deployment Types

### REPOSITORY Deployments

**REPOSITORY** deployments are designed for version control and development:

- **Full version control**: Complete snapshot and branching capabilities
- **Development focused**: Optimized for frequent changes and experimentation
- **Branch management**: Create and manage multiple branches
- **Snapshot history**: Complete audit trail of all changes

**Use cases:**
- Application development
- Feature development
- Testing and QA
- Development environments

**Example:**
```bash
guepard deploy \
  --database-provider PostgreSQL \
  --database-version 16 \
  --region us-west \
  --instance-type REPOSITORY \
  --datacenter aws \
  --repository-name dev-database \
  --database-password dev_password
```

### F2 Deployments

**F2** deployments are optimized for compute and production workloads:

- **High performance**: Optimized for production workloads
- **Compute focused**: Designed for running applications
- **Limited versioning**: Basic snapshot capabilities
- **Production ready**: Built for stability and performance

**Use cases:**
- Production applications
- High-traffic systems
- Compute-intensive workloads
- Production environments

**Example:**
```bash
guepard deploy \
  --database-provider PostgreSQL \
  --database-version 16 \
  --region us-west \
  --instance-type F2 \
  --datacenter aws \
  --repository-name prod-database \
  --database-password prod_password \
  --performance-profile gp.g1.large
```

## Performance Profiles

Choose the right performance profile for your workload:

### gp.g1.xsmall
- **Use case**: Development, testing, small applications
- **Resources**: 1 vCPU, 2GB RAM
- **Storage**: 20GB SSD
- **Cost**: Most economical

### gp.g1.small
- **Use case**: Small production applications, staging
- **Resources**: 2 vCPU, 4GB RAM
- **Storage**: 50GB SSD
- **Cost**: Balanced performance and cost

### gp.g1.medium
- **Use case**: Medium production applications
- **Resources**: 4 vCPU, 8GB RAM
- **Storage**: 100GB SSD
- **Cost**: Good performance for most applications

### gp.g1.large
- **Use case**: Large production applications, high-traffic systems
- **Resources**: 8 vCPU, 16GB RAM
- **Storage**: 200GB SSD
- **Cost**: High performance for demanding workloads

## Regions and Datacenters

### Available Regions

- **us-west**: West Coast United States
- **us-east**: East Coast United States
- **eu-west**: Western Europe
- **asia-pacific**: Asia Pacific region

### Cloud Providers

- **AWS**: Amazon Web Services
- **GCP**: Google Cloud Platform
- **Azure**: Microsoft Azure

### Choosing the Right Region

Consider these factors when selecting a region:

- **Latency**: Choose the region closest to your users
- **Compliance**: Some data must stay in specific regions
- **Availability**: Check regional availability for your chosen database
- **Cost**: Pricing may vary by region

## Managing Deployments

### View Deployment Details

```bash
guepard deploy --deployment-id <deployment_id>
```

**Output:**
```
📋 Deployment Details
  🆔 ID: 12345678-1234-1234-1234-123456789abc
  📝 Name: my-app-db
  📁 Repository: my-app-db
  🗄️ Provider: PostgreSQL
  🔢 Version: 16
  ✅ Status: active
  🌐 FQDN: my-app-db.guepard.run
  🌍 Region: us-west
  🏢 Datacenter: aws
  📅 Created: 2025-01-08T10:30:00Z

🔗 Database Connection
  🏠 Host: my-app-db.guepard.run
  🔌 Port: 5432
  🗃️ Database: my-app-db
  👤 Username: guepard
  🔐 Password: secure_password_123

💡 Ready-to-use Connection URI:
postgresql://guepard:secure_password_123@my-app-db.guepard.run:5432/my-app-db
```

### List All Deployments

```bash
guepard list deployments
```

**Output:**
```
✅ Found 3 deployments
┌─────────────────────────────────────────────────────────────────┐
│ ID            │ Name      │ Repository │ Provider │ Version │ Status │ FQDN              │
├─────────────────────────────────────────────────────────────────┤
│ 12345678...   │ my-app-db │ my-app-db  │ PostgreSQL│ 16     │ active │ my-app-db.guepard.run │
│ 87654321...   │ test-db   │ test-db    │ MySQL    │ 8.0     │ active │ test-db.guepard.run   │
│ 11223344...   │ prod-db   │ prod-db    │ PostgreSQL│ 16     │ active │ prod-db.guepard.run   │
└─────────────────────────────────────────────────────────────────┘
```

### Update Deployment

```bash
guepard deploy \
  --deployment-id <deployment_id> \
  --repository-name new-name
```

### Delete Deployment

```bash
guepard deploy \
  --deployment-id <deployment_id> \
  --yes
```

**Note**: This action cannot be undone. All data, snapshots, and branches will be permanently deleted.

## Connection Management

### Database Connection

Once your deployment is created, you can connect using:

**Connection URI:**
```
postgresql://guepard:password@my-app-db.guepard.run:5432/my-app-db
```

**Individual parameters:**
- **Host**: `my-app-db.guepard.run`
- **Port**: `5432` (PostgreSQL), `3306` (MySQL), `27017` (MongoDB)
- **Database**: `my-app-db`
- **Username**: `guepard` (or custom)
- **Password**: Your chosen password

### Connecting with psql

```bash
psql 'postgresql://guepard:password@my-app-db.guepard.run:5432/my-app-db'
```

### Connecting with MySQL Client

```bash
mysql -h my-app-db.guepard.run -P 3306 -u guepard -p my-app-db
```

### Connecting with MongoDB Client

```bash
mongosh 'mongodb://guepard:password@my-app-db.guepard.run:27017/my-app-db'
```

## Compute Management

### Start Compute

```bash
guepard compute start --deployment-id <deployment_id>
```

### Stop Compute

```bash
guepard compute stop --deployment-id <deployment_id>
```

### Check Compute Status

```bash
guepard compute status --deployment-id <deployment_id>
```

### View Compute Details

```bash
guepard compute list --deployment-id <deployment_id>
```

## Best Practices

### Deployment Naming

Use descriptive, consistent naming:

```bash
# Good naming
guepard deploy -n "myapp-dev"
guepard deploy -n "myapp-staging"
guepard deploy -n "myapp-prod"

# Bad naming
guepard deploy -n "db1"
guepard deploy -n "test"
guepard deploy -n "database"
```

### Password Security

Use strong, unique passwords:

```bash
# Generate secure password
openssl rand -base64 32

# Use in deployment
guepard deploy -w $(openssl rand -base64 32) -n myapp-db
```

### Environment Separation

Create separate deployments for different environments:

```bash
# Development
guepard deploy -n "myapp-dev" -i REPOSITORY -f gp.g1.xsmall

# Staging
guepard deploy -n "myapp-staging" -i REPOSITORY -f gp.g1.small

# Production
guepard deploy -n "myapp-prod" -i F2 -f gp.g1.large
```

### Resource Planning

Choose appropriate performance profiles:

- **Development**: gp.g1.xsmall
- **Testing**: gp.g1.small
- **Staging**: gp.g1.medium
- **Production**: gp.g1.large

### Monitoring

Regularly monitor your deployments:

```bash
# Check status
guepard compute status -x <deployment_id>

# View logs
guepard log -x <deployment_id>

# Check usage
guepard usage
```

## Troubleshooting Deployments

### Common Issues

**Deployment creation fails:**
```bash
# Check authentication
guepard login

# Verify parameters
guepard deploy --help

# Use interactive mode
guepard deploy --interactive
```

**Cannot connect to database:**
```bash
# Check compute status
guepard compute status -x <deployment_id>

# Start compute if stopped
guepard compute start -x <deployment_id>

# Verify connection details
guepard deploy -x <deployment_id>
```

**Performance issues:**
```bash
# Check current profile
guepard deploy -x <deployment_id>

# Upgrade performance profile
guepard deploy -x <deployment_id> -f gp.g1.large
```

### Getting Help

If you encounter issues:

1. **Check logs**: `guepard log -x <deployment_id>`
2. **Verify status**: `guepard compute status -x <deployment_id>`
3. **Review documentation**: Check this guide and other documentation
4. **Community support**: Join our [Discord](https://discord.gg/NYsNzQGvZT)
5. **Report issues**: Create an issue on [GitHub](https://github.com/Guepard-Corp/guepard-cli/issues)

---

*Now that you understand deployments, learn about [Branches and Snapshots](branches-snapshots.md) to start using version control! 🐆*
