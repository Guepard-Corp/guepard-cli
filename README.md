Here's the complete updated `README.md` content with the login commands (`link` and `login`) added under a new section titled **🔐 Authentication Commands**, consistent with the existing style:

---

# 🐆 Guepard CLI Documentation

`gprd` is a command-line tool to manage your Guepard environment. This CLI allows you to efficiently handle deployments, branches, bookmarks, compute instances, and usage quotas for your data management tasks.

---

## 🚀 Installation

1. **Clone the repository:**

   ```bash
   git clone <repository-url>
   ```

2. **Navigate to the project directory:**

   ```bash
   cd guepard-cli
   ```

3. **Build the project using Cargo:**

   ```bash
   cargo build --release
   ```

4. **Run the CLI tool:**

   ```bash
   ./target/release/gprd -h
   ```

---

## 📖 Usage Guide

### General CLI Structure

```bash
gprd <SUBCOMMAND>
```

🔝 Run `gprd -h` for a full list of subcommands and options.

---

## 🔐 Authentication Commands

### 🔗 Get Login Link

Generates a login link containing a session ID.

```bash
gprd link
```

This command initiates the login process by returning a URL with a `session_id`. Open this URL in your browser to continue authentication.

---

### 🔑 Complete Login

Completes the login flow by submitting the verification code.

```bash
gprd login
```

After opening the login link and authenticating in the browser, you'll receive a 6-digit code. Run `gprd login` and enter the code when prompted. A confirmation message will appear upon successful login.

---

### 🗝️🔓 Logout

Deletes stored credentials from the user's device, effectively logging out of the CLI.

```bash
gprd logout
```

This command ensures that all locally stored authentication tokens are removed, preventing unauthorized access. Use this command when switching accounts or securing your environment.



---

## 📦 Deployment Management Commands

### ➕ Create Deployment

Creates a new deployment.

```bash
gprd deploy create 
  -p <DATABASE_PROVIDER> 
  -v <DATABASE_VERSION> 
  -r <REGION> 
  -d <DATACENTER> 
  -i <INSTANCE_TYPE> 
  -t <DEPLOYMENT_TYPE> 
  -n <REPOSITORY_NAME> 
  -u <DATABASE_USERNAME> 
  -w <DATABASE_PASSWORD> 
  [--performance-profile-id <PERFORMANCE_PROFILE_ID>] 
  [--node-id <NODE_ID>]
```

**Options:**

* `-p, --database-provider`        : Database provider (e.g., PostgreSQL, MySQL)
* `-v, --database-version`         : Database version (e.g., 17)
* `-r, --region`                   : Region for the deployment (e.g., us-west-aws)
* `-d, --datacenter`               : Datacenter for the deployment (e.g., us-west-aws)
* `-i, --instance-type`            : Instance type (e.g., free)
* `-t, --deployment-type`          : Deployment type (e.g., REPOSITORY)
* `-n, --repository-name`          : Name of the repository
* `-u, --database-username`        : Database username
* `-w, --database-password`        : Database password
* `--performance-profile-id`       : Performance profile ID (optional)
* `--node-id`                      : Node ID (optional)

**Output:**  
Displays the deployment ID, name, status, repository name, database provider, region, and username.

**Example:**

✅ Created deployment [18cdfa3d-7614-4784-963a-f91011efe81a] 'fierce-ocean-vq0l4t' (Status: INIT) with repo [db-no-node], provider [PostgreSQL], region [global], username [guepard]

---

### ✏️ Update Deployment

Updates an existing deployment’s repository name.

```bash
gprd deploy update 
  -x <DEPLOYMENT_ID> 
  -n <REPO_NAME>
```

**Options:**

* `-x, --deployment-id`   : ID of the deployment to update
* `-n, --repository-name` : New repository name

---

### 📋 List Deployments

Lists all deployments.

```bash
gprd deploy list
```

---

### 🔍 Get Deployment

Fetches details of a specific deployment.

```bash
gprd deploy get -x <DEPLOYMENT_ID>
```

**Options:**

* `-x, --deployment-id` : ID of the deployment to fetch

---

## 🌿 Branch Management Commands

### 🌱 Create Branch

Creates a new branch from a snapshot in a deployment.

```bash
gprd branch create 
  -x <DEPLOYMENT_ID> 
  -b <BRANCH_ID> 
  -s <SNAPSHOT_ID> 
  [--discard-changes] 
  [--checkout] 
  [--ephemeral]
```

**Options:**

* `-x, --deployment-id`   : Deployment ID
* `-b, --branch-id`       : Source branch ID (used as branch name)
* `-s, --snapshot-id`     : Snapshot ID
* `--discard-changes`     : Discard changes in the source branch (default: false)
* `--checkout`            : Checkout the branch after creation (default: false)
* `--ephemeral`           : Create an ephemeral branch (default: false)

**Output:**  
Confirms the branch creation with branch ID, name, status, snapshot ID, and deployment ID.

**Example:**

✅ Created branch [ca395480-ee92-441f-9773-560f768b96f7] 'from main branch 01' (INIT) from snapshot [5452c8a5-ca29-4c4e-91f9-8513209ae32e] in deployment [15c6cb1d-9ab2-440f-9e91-10b51306637f]

---

### 🌿 List Branches

Lists all non-ephemeral branches for a deployment.

```bash
gprd branch list -x <DEPLOYMENT_ID>
```

**Options:**

* `-x, --deployment-id` : Deployment ID

**Output:**  
Displays a table with branch ID, name, status, and snapshot ID. Ephemeral branches are excluded.

**Example:**

✅ Retrieved 1 non-ephemeral branches:
```
┌──────────────────────────────────────┬──────────────┬─────────┬──────────────────────────────────────┐
│ Branch ID                            │ Name         │ Status  │ Snapshot ID                          │
├──────────────────────────────────────┼──────────────┼─────────┼──────────────────────────────────────┤
│ 599cff72-247e-4baf-bac9-73e86d4d86b4 │ main         │ CREATED │ 69049b72-9ef0-4e2e-9ff3-4b94adeac994 │
└──────────────────────────────────────┴──────────────┴─────────┴──────────────────────────────────────┘
```

---

### ✅ Checkout Branch

Checks out an existing branch.

```bash
gprd branch checkout 
  -x <DEPLOYMENT_ID> 
  -c <CLONE_ID>
```

---

## 🔖 Bookmark Management Commands

### 📋 List All Bookmarks

Lists all bookmarks for a deployment.

```bash
gprd bookmark list-all -x <DEPLOYMENT_ID>
```

---

### 📋 List Bookmarks

Lists bookmarks for a specific branch in a deployment.

```bash
gprd bookmark list 
  -x <DEPLOYMENT_ID> 
  -b <BRANCH_ID>
```

---

### ➕ Create Bookmark

Creates a new bookmark for a clone.

```bash
gprd bookmark create 
  -x <DEPLOYMENT_ID> 
  -b <BRANCH_ID> 
  -m <COMMENT>
```

---

### ✅ Checkout Bookmark

Checks out a bookmark as a new branch.

```bash
gprd bookmark checkout 
  -x <DEPLOYMENT_ID> 
  -c <CLONE_ID> 
  -s <SNAPSHOT_ID> 
  -d <DISCARD_CHANGES> 
  [-k] [-e]
```

---

## 👤 Compute Management Commands

### 📋 List Compute

Lists compute details for a deployment.

```bash
gprd compute list -x <DEPLOYMENT_ID>
```

**Options:**

* `-x, --deployment-id` : Deployment ID

**Output:**  
Displays the compute name, branch ID, and connection string.

**Example:**

✅ Compute 'sad-sound-n0j1kq', Branch: [599cff72-247e-4baf-bac9-73e86d4d86b4], Connection String : [postgresql://guepard:guepard@sad-sound-n0j1kq.us-west-aws.db.dev.guepard.run:22885/postgres?sslmode=require]

### ▶️ Start Compute

Starts a compute instance for a deployment.

```bash
gprd compute start -x <DEPLOYMENT_ID>
```

**Options:**

* `-x, --deployment-id` : Deployment ID

**Output:**  
Confirms that the job compute  for the deployment is received by the nomad server.

**Example:**

✅ Started compute for deployment [15c6cb1d-9ab2-440f-9e91-10b51306637f]

---

### 🛑 Stop Compute

Stops a compute instance for a deployment.

```bash
gprd compute stop -x <DEPLOYMENT_ID>
```

**Options:**

* `-x, --deployment-id` : Deployment ID

**Output:**  
Confirms that the compute instance for the deployment has stopped.

**Example:**

✅ Stopped compute for deployment [15c6cb1d-9ab2-440f-9e91-10b51306637f]


---

### 🔎 Get Compute Status

Checks the health status of a compute instance.

```bash
gprd compute status 
  -x <DEPLOYMENT_ID> 
```

---

## 📊 Usage Command

### 📈 Show Usage

Displays quota and usage details.

```bash
gprd usage
```

---



## ⚙️ Environment Variables

The CLI requires the following environment variables for API connectivity. You can create a `.env` file in the project root.

### Required Variables:

```dotenv
PUBLIC_API=<API_URL>      # Base URL of the Guepard API (e.g., https://api.guepard.io)
```

### Example `.env` file:

```dotenv
PUBLIC_API=https://api.guepard.io/v1
API_TOKEN=your-api-token-here
```

These variables are loaded automatically via dotenv if present.

---

## 🎨 Output Formatting

* ✅ **Success**: Green "✅" prefix with cyan IDs (e.g., `[deployment_id]`)
* ℹ️ **Info**: Blue "ℹ️" prefix for empty results
* ⚠️ **Warnings**: Yellow "⚠️" prefix (e.g., unhealthy compute)
* ❌ **Errors**: Red "❌" prefix (requires error handling files for full details)

---

