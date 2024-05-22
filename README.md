# Rust exam project

Scenario 3: Simple Banking System

cph-cm370@cphbusiness.dk

# Backend:
To run the project create a database in postgresql

Then create a .env file and add the DATABASE_URL

Install diesel with postgresql: "cargo install diesel_cli --no-default-features --features postgres"

The run: "diesel setup" and "diesel migration generate create_tables" and add the sql code to the down.sql and up.sql files

The use: "diesel migration run" to create the tables in the database

To run the project backend: "cargo run"

# Frontend:

Cd into: web_ui folder

Run: "npm install" and then "npm start"