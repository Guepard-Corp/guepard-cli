# Guepard CLI Tutorial

This tutorial walks you through the complete workflow of creating deployments, managing commits, and cloning databases using the Guepard CLI.

## Table of Contents

1. [Creating a Deployment](#1-creating-a-deployment)
2. [Viewing Deployment Details](#2-viewing-deployment-details)
3. [Creating Commits](#3-creating-commits)
4. [Cloning a Deployment](#4-cloning-a-deployment)
5. [Viewing Clone Details](#5-viewing-clone-details)

---

## 1. Creating a Deployment

Create a new F2 deployment with PostgreSQL.

### Command

```bash
guepard deploy -p PostgreSQL -v 16 -r us-west -i F2 -d aws -w testpassword123 -n newTestF2 -f gp.g1.xsmall -s e1b33620-ea91-437f-9b8e-6334040a7423
```

### Parameters

- `-p PostgreSQL`: Database provider
- `-v 16`: PostgreSQL version
- `-r us-west`: Region
- `-i F2`: Instance type (F2)
- `-d aws`: Datacenter provider
- `-w testpassword123`: Database password
- `-n newTestF2`: Repository name
- `-f gp.g1.xsmall`: Performance profile/flavor
- `-s e1b33620-ea91-437f-9b8e-6334040a7423`: Snapshot ID (optional)

### Expected Output

```
✅ Deployment created successfully!

📋 Deployment Details
  ID: 41799326-15a1-430d-9621-1e3de50ababb
  Name: ancient-moon-enulal
  Type: F2
  Repository: newTestF2
  Provider: PostgreSQL
  Version: 16
  Status: INIT
  FQDN: ancient-moon-enulal.us-west-aws.db.guepard.run
  Region: global
  Datacenter: us-west-aws
  Created: 2025-12-12T23:08:54.522138+00:00

🔗 Database Connection
  Host: ancient-moon-enulal.us-west-aws.db.guepard.run
  Port: 25545
  Database: newTestF2
  Username: guepard
  Password: testpassword123

💡 Ready-to-use Connection URI:
postgresql://guepard:testpassword123@ancient-moon-enulal.us-west-aws.db.guepard.run:25545/newTestF2

📝 Connect with psql:
  $ psql 'postgresql://guepard:testpassword123@ancient-moon-enulal.us-west-aws.db.guepard.run:25545/newTestF2'

ℹ️ Connect with any PostgreSQL client using the URI above

💡 Use 'guepard deploy -x 41799326-15a1-430d-9621-1e3de50ababb' to get more details
```

### What Happens

- A new deployment is created with the specified configuration
- You receive connection details immediately
- The deployment starts in `INIT` status and will transition to `CREATED` when ready
- Save the deployment ID for future operations

---

## 2. Viewing Deployment Details

Retrieve detailed information about your deployment, including checkout and compute information.

### Command

```bash
guepard deploy -x 41799326-15a1-430d-9621-1e3de50ababb
```

### Parameters

- `-x 41799326-15a1-430d-9621-1e3de50ababb`: Deployment ID from the previous step

### Expected Output

```
📋 Deployment Details
  ID: 41799326-15a1-430d-9621-1e3de50ababb
  Name: ancient-moon-enulal
  Type: F2
  Repository: newTestF2
  Provider: PostgreSQL
  Version: 16
  Status: CREATED
  FQDN: ancient-moon-enulal.us-west-aws.db.guepard.run
  Region: global
  Datacenter: us-west-aws
  Created: 2025-12-12T23:08:54.522138+00:00

📍 Checkout Information
  Branch: main
  Branch ID: 0e0932af-1ea9-4e13-a107-9c028fcc862a
  Snapshot: daaf1750
  Comment: Initial Bookmark
  Snapshot ID: daaf1750-637c-4a08-be97-fba31a5ade20

🖥️ Compute Information
  Compute Name: ancient-moon-enulal
  FQDN: ancient-moon-enulal.us-west-aws.db.guepard.run
  Port: 25545
  Connection String: postgresql://guepard:testpassword123@ancient-moon-enulal.us-west-aws.db.guepard.run:25545/postgres?sslmode=require

🔗 Database Connection
  Host: ancient-moon-enulal.us-west-aws.db.guepard.run
  Port: 25545
  Database: newTestF2
  Username: guepard
  Password: testpassword123

💡 Ready-to-use Connection URI:
postgresql://guepard:testpassword123@ancient-moon-enulal.us-west-aws.db.guepard.run:25545/newTestF2

📝 Connect with psql:
  $ psql 'postgresql://guepard:testpassword123@ancient-moon-enulal.us-west-aws.db.guepard.run:25545/newTestF2'

ℹ️ Connect with any PostgreSQL client using the URI above
```

### What Happens

- Displays complete deployment information
- Shows checkout information (branch and snapshot details)
- Provides compute information including connection strings
- Status should now be `CREATED` (was `INIT` during creation)
- Note the Branch ID for creating commits

---

## 3. Creating Commits

Create snapshots of your database state by committing changes.

### Command 1: First Commit

```bash
guepard commit -m 'Test commit for deployment snapshot' -x 41799326-15a1-430d-9621-1e3de50ababb -b 0e0932af-1ea9-4e13-a107-9c028fcc862a
```

### Parameters

- `-m 'Test commit for deployment snapshot'`: Commit message
- `-x 41799326-15a1-430d-9621-1e3de50ababb`: Deployment ID
- `-b 0e0932af-1ea9-4e13-a107-9c028fcc862a`: Branch ID (from deployment details)

### Expected Output

```
✅ Created commit successfully!
╭──────────────────────────────────────┬─────────────────────────────────────┬────────┬──────────────────────────────────╮
│ Commit ID                            │ Message                             │ Status │ Created                          │
├──────────────────────────────────────┼─────────────────────────────────────┼────────┼──────────────────────────────────┤
│ 6ea6b7a7-e8d2-4483-9900-bc065faf298c │ Test commit for deployment snapshot │ INIT   │ 2025-12-12T21:34:44.849294+00:00 │
╰──────────────────────────────────────┴─────────────────────────────────────┴────────┴──────────────────────────────────╯
```

### Command 2: Second Commit

```bash
guepard commit -m 'commit 2' -x 41799326-15a1-430d-9621-1e3de50ababb -b 0e0932af-1ea9-4e13-a107-9c028fcc862a
```

### Expected Output

```
✅ Created commit successfully!
╭──────────────────────────────────────┬──────────┬────────┬─────────────────────────────────╮
│ Commit ID                            │ Message  │ Status │ Created                         │
├──────────────────────────────────────┼──────────┼────────┼─────────────────────────────────┤
│ 1c67aefa-f4d5-4669-9ad2-622ca799b739 │ commit 2 │ INIT   │ 2025-12-12T21:35:32.38499+00:00 │
╰──────────────────────────────────────┴──────────┴────────┴─────────────────────────────────╯
```

### What Happens

- Creates a snapshot of the current database state
- Each commit creates a new snapshot that can be used for cloning or checkout
- Commits are tracked with unique IDs and timestamps
- Use these snapshot IDs when cloning to specific database states

---

## 4. Cloning a Deployment

Create a clone of your deployment from a specific snapshot.

### Command

```bash
guepard clone -x 41799326-15a1-430d-9621-1e3de50ababb -s 5752578a-a6ff-4be4-81fd-2ac6b65c6e03 -f gp.g1.small
```

### Parameters

- `-x 41799326-15a1-430d-9621-1e3de50ababb`: Parent deployment ID
- `-s 5752578a-a6ff-4be4-81fd-2ac6b65c6e03`: Snapshot ID to clone from
- `-f gp.g1.small`: Performance profile/flavor for the clone

### Expected Output

```
✅ Clone created successfully!

📋 Clone Details
  ID: f66ad3a9-7877-4e04-95ab-c5996538d3e6
  Name: silent-sky-audn9o
  Deployment: clone-newTestF2
  Provider: PostgreSQL
  Version: 16
  Status: INIT
  FQDN: silent-sky-audn9o.us-west-aws.db.guepard.run
  Created: 2025-12-12T23:15:14.289709+00:00
  Deployment Parent: 41799326-15a1-430d-9621-1e3de50ababb
  Snapshot Parent: 5752578a-a6ff-4be4-81fd-2ac6b65c6e03

  Branch: main
  Branch ID: 3f77ea28-11bd-4087-88e9-e170a2208f21
  Snapshot: 43613486
  Comment: commit 2
  Snapshot ID: 43613486-d2b1-48f9-b30e-ebea2f24b277

🔗 Database Connection
  Host: silent-sky-audn9o.us-west-aws.db.guepard.run
  Port: 20980
  Database: clone-newTestF2
  Username: N/A
  Password: N/A
```

### What Happens

- Creates a new clone deployment from the specified snapshot
- The clone has its own unique ID and connection details
- Repository name is prefixed with `clone-` (e.g., `clone-newTestF2`)
- Initially, credentials may show as `N/A` until the clone is fully provisioned
- The clone maintains a reference to its parent deployment and snapshot

---

## 5. Viewing Clone Details

Retrieve detailed information about your clone, similar to viewing deployment details.

### Command

```bash
guepard deploy -x f66ad3a9-7877-4e04-95ab-c5996538d3e6
```

### Parameters

- `-x f66ad3a9-7877-4e04-95ab-c5996538d3e6`: Clone ID from the previous step

### Expected Output

```
📋 Clone Details
  ID: f66ad3a9-7877-4e04-95ab-c5996538d3e6
  Name: silent-sky-audn9o
  Type: Clone
  Repository: clone-newTestF2
  Provider: PostgreSQL
  Version: 16
  Status: INIT
  FQDN: silent-sky-audn9o.us-west-aws.db.guepard.run
  Region: global
  Datacenter: us-west-aws
  Created: 2025-12-12T23:15:14.289709+00:00
  Deployment Parent: 41799326-15a1-430d-9621-1e3de50ababb
  Snapshot Parent: 5752578a-a6ff-4be4-81fd-2ac6b65c6e03

📍 Checkout Information
  Branch: main
  Branch ID: 3f77ea28-11bd-4087-88e9-e170a2208f21
  Snapshot: 43613486
  Comment: commit 2
  Snapshot ID: 43613486-d2b1-48f9-b30e-ebea2f24b277

🖥️ Compute Information
  Compute Name: happy-star-eprols
  FQDN: happy-star-eprols.us-west-aws.db.guepard.run
  Port: 20980
  Connection String: postgresql://guepard:testpassword123@happy-star-eprols.us-west-aws.db.guepard.run:20980/postgres?sslmode=require

🔗 Database Connection
  Host: silent-sky-audn9o.us-west-aws.db.guepard.run
  Port: 20980
  Database: clone-newTestF2
  Username: guepard
  Password: testpassword123

💡 Ready-to-use Connection URI:
postgresql://guepard:testpassword123@silent-sky-audn9o.us-west-aws.db.guepard.run:20980/clone-newTestF2

📝 Connect with psql:
  $ psql 'postgresql://guepard:testpassword123@silent-sky-audn9o.us-west-aws.db.guepard.run:20980/clone-newTestF2'

ℹ️ Connect with any PostgreSQL client using the URI above
```

### What Happens

- Displays complete clone information
- Shows parent deployment and snapshot references
- Provides full connection details including credentials
- The clone has its own compute instance with separate connection details
- You can now connect to the clone independently from the parent deployment

---

## Summary

This tutorial covered the complete workflow:

1. **Create a deployment** - Set up a new database deployment
2. **View deployment details** - Get comprehensive information including branch IDs
3. **Create commits** - Save snapshots of your database state
4. **Clone a deployment** - Create independent copies from specific snapshots
5. **View clone details** - Access connection information for your clones

### Key Takeaways

- Save deployment IDs and branch IDs for subsequent operations
- Commits create snapshots that enable point-in-time cloning
- Clones are independent deployments with their own connection details
- Use `guepard deploy -x <ID>` to view details for both deployments and clones
- Connection URIs are provided for easy integration with PostgreSQL clients
