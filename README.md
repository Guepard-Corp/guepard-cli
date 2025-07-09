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
  -p <DB_PROVIDER> 
  -v <DB_VERSION> 
  -r <REGION> 
  -i <INSTANCE_TYPE> 
  -d <DATACENTER> 
  -n <REPO_NAME> 
  -w <DB_PASSWORD>
```

**Options:**

* `-p, --database-provider` : Database provider (e.g., PostgreSQL, MySQL)
* `-v, --database-version`  : Database version
* `-r, --region`            : Region for deployment
* `-i, --instance-type`     : Instance type
* `-d, --datacenter`        : Datacenter location
* `-n, --repository-name`   : Repository name
* `-w, --database-password` : Database password

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

### ➕ Create Branch

Creates a new branch from a snapshot.

```bash
gprd branch create 
  -x <DEPLOYMENT_ID> 
  -c <CLONE_ID> 
  -s <SNAPSHOT_ID> 
  -d <DISCARD_CHANGES> 
  [-k] [-e]
```

**Options:**

* `-x, --deployment-id` : Deployment ID
* `-c, --clone-id`      : Clone ID to branch from
* `-s, --snapshot-id`   : Snapshot ID to base the branch on
* `-d, --discard-changes` : Changes to discard (e.g., "true")
* `-k, --checkout`      : Check out the branch after creation (optional)
* `-e, --ephemeral`     : Mark the branch as ephemeral (optional)

---

### 📋 List Branches

Lists all non-ephemeral branches for a deployment.

```bash
gprd branch list -x <DEPLOYMENT_ID>
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

Lists bookmarks for a specific clone in a deployment.

```bash
gprd bookmark list 
  -x <DEPLOYMENT_ID> 
  -c <CLONE_ID>
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

Fetches details of a compute instance.

```bash
gprd compute list 
  -x <DEPLOYMENT_ID> 
  -c <COMPUTE_ID>
```

---

### ▶️ Start Compute

Starts a compute instance.

```bash
gprd compute start 
  -x <DEPLOYMENT_ID> 
  -c <COMPUTE_ID>
```

---

### ⏹️ Stop Compute

Stops a compute instance.

```bash
gprd compute stop 
  -x <DEPLOYMENT_ID> 
  -c <COMPUTE_ID>
```

---

### 📜 Get Compute Logs

Fetches logs for a compute instance.

```bash
gprd compute logs 
  -x <DEPLOYMENT_ID> 
  -c <COMPUTE_ID>
```

---

### 🔎 Get Compute Status

Checks the health status of a compute instance.

```bash
gprd compute status 
  -x <DEPLOYMENT_ID> 
  -c <COMPUTE_ID>
```

---

## 📊 Usage Command

### 📈 Show Usage

Displays quota and usage details.

```bash
gprd usage
```

---

## 👀 Show Commands

### 🌿 Show Branches

Lists branches with the active one marked.

```bash
gprd show branches -x <DEPLOYMENT_ID>
```

---

### 🔖 Show Bookmarks

Lists bookmarks with the active one marked.

```bash
gprd show bookmarks -x <DEPLOYMENT_ID>
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

