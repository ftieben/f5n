# f5n
A combination of a Skill-Management System and a personal CRM

## Tech-Stack
The Idea of this project is to play around with a bunch of different technologies and have it as a long time "Portfolio" Project

The current iteration will look like:

### Svelte (TypeScript, Tailwind)
WebUI Serving Layer

### clap (Rust)
CLI Interface to Interact with the APIs

### actix (Rust)
REST API 

### ScyllaDB
Database

## Folder Structure
- ui 
- cli
- services
  - skill
  - crm
  - admin

## Build and Run
We will focus first of having the local setup up and running in an easy way. Therefor everything will be done with docker and docker-compose

### Build
  docker-compose build

### Run
  docker-compose up -d
