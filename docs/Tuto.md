cargo run -- deploy -p PostgreSQL -v 16 -r us-west -i F2 -d aws -w testpassword123 -n F2Demo -f gp.g1.xsmall -s e1b33620-ea91-437f-9b8e-6334040a7423
   Compiling guepard-cli v0.27.22 (/Users/mghassen/Workspace/GPRD/guepard-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.43s
     Running `target/debug/guepard deploy -p PostgreSQL -v 16 -r us-west -i F2 -d aws -w testpassword123 -n F2Demo -f gp.g1.xsmall -s e1b33620-ea91-437f-9b8e-6334040a7423`
✅ Deployment created successfully!

📋 Deployment Details
  ID: 27a4aea7-880a-4116-a91c-a468962b5eda
  Name: ancient-cloud-4xyi1g
  Repository: F2Demo
  Provider: PostgreSQL
  Version: 16
  Status: INIT
  FQDN: ancient-cloud-4xyi1g.us-west-aws.db.guepard.run
  Region: global
  Datacenter: us-west-aws
  Created: 2025-12-12T21:19:20.582519+00:00

🔗 Database Connection
  Host: ancient-cloud-4xyi1g.us-west-aws.db.guepard.run
  Port: 20067
  Database: F2Demo
  Username: guepard
  Password: testpassword123

💡 Ready-to-use Connection URI:
postgresql://guepard:testpassword123@ancient-cloud-4xyi1g.us-west-aws.db.guepard.run:20067/F2Demo

📝 Connect with psql:
  $ psql 'postgresql://guepard:testpassword123@ancient-cloud-4xyi1g.us-west-aws.db.guepard.run:20067/F2Demo'

ℹ️ Connect with any PostgreSQL client using the URI above

💡 Use 'guepard deploy -x 27a4aea7-880a-4116-a91c-a468962b5eda' to get more details




-------



