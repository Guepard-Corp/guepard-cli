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



cargo run -- deploy -x 27a4aea7-880a-4116-a91c-a468962b5eda             ✔ │ 9s  │ 22:28:22  
   Compiling guepard-cli v0.27.22 (/Users/mghassen/Workspace/GPRD/guepard-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.10s
     Running `target/debug/guepard deploy -x 27a4aea7-880a-4116-a91c-a468962b5eda`
📋 Deployment Details
  ID: 27a4aea7-880a-4116-a91c-a468962b5eda
  Name: ancient-cloud-4xyi1g
  Repository: F2Demo
  Provider: PostgreSQL
  Version: 16
  Status: CREATED
  FQDN: ancient-cloud-4xyi1g.us-west-aws.db.guepard.run
  Region: global
  Datacenter: us-west-aws
  Created: 2025-12-12T21:19:20.582519+00:00
  Branch: main
  Branch ID: 30c79841-72a6-4ead-a8f9-2d4c68182f4a
  Snapshot: c54f12f8
  Comment: Initial Bookmark
  Snapshot ID: c54f12f8-e480-488c-b96a-89f1d1299603

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


-----

│  ~/Wo/GPRD/guepard-cli │   main  cargo run -- commit -m "Test commit for deployment snapshot" -x 27a4aea7-880a-4116-a91c-a468962b5eda -b 30c79841-72a6-4ead-a8f9-2d4c68182f4a
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/guepard commit -m 'Test commit for deployment snapshot' -x 27a4aea7-880a-4116-a91c-a468962b5eda -b 30c79841-72a6-4ead-a8f9-2d4c68182f4a`
✅ Created commit successfully!
╭──────────────────────────────────────┬─────────────────────────────────────┬────────┬──────────────────────────────────╮
│ Commit ID                            │ Message                             │ Status │ Created                          │
├──────────────────────────────────────┼─────────────────────────────────────┼────────┼──────────────────────────────────┤
│ 6ea6b7a7-e8d2-4483-9900-bc065faf298c │ Test commit for deployment snapshot │ INIT   │ 2025-12-12T21:34:44.849294+00:00 │
╰──────────────────────────────────────┴─────────────────────────────────────┴────────┴──────────────────────────────────╯
  │  ~/Wo/GPRD/guepard-cli │   main !1  cargo run -- commit -m "commit 2" -x 27a4aea7-880a-4116-a91c-a468962b5eda -b 30c79841-72a6-4ead-a8f9-2d4c68182f4a        
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/guepard commit -m 'commit 2' -x 27a4aea7-880a-4116-a91c-a468962b5eda -b 30c79841-72a6-4ead-a8f9-2d4c68182f4a`
✅ Created commit successfully!
╭──────────────────────────────────────┬──────────┬────────┬─────────────────────────────────╮
│ Commit ID                            │ Message  │ Status │ Created                         │
├──────────────────────────────────────┼──────────┼────────┼─────────────────────────────────┤
│ 1c67aefa-f4d5-4669-9ad2-622ca799b739 │ commit 2 │ INIT   │ 2025-12-12T21:35:32.38499+00:00 │
╰──────────────────────────────────────┴──────────┴────────┴─────────────────────────────────╯

-------

cargo run -- deploy -x 27a4aea7-880a-4116-a91c-a468962b5eda             ✔ │ 9s  │ 22:36:07  
   Compiling guepard-cli v0.27.23 (/Users/mghassen/Workspace/GPRD/guepard-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.28s
     Running `target/debug/guepard deploy -x 27a4aea7-880a-4116-a91c-a468962b5eda`
📋 Deployment Details
  ID: 27a4aea7-880a-4116-a91c-a468962b5eda
  Name: ancient-cloud-4xyi1g
  Repository: F2Demo
  Provider: PostgreSQL
  Version: 16
  Status: CREATED
  FQDN: ancient-cloud-4xyi1g.us-west-aws.db.guepard.run
  Region: global
  Datacenter: us-west-aws
  Created: 2025-12-12T21:19:20.582519+00:00
  Branch: main
  Branch ID: 30c79841-72a6-4ead-a8f9-2d4c68182f4a
  Snapshot: 1c67aefa
  Comment: commit 2
  Snapshot ID: 1c67aefa-f4d5-4669-9ad2-622ca799b739

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

-------

