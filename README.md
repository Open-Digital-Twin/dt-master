# DT Master
![Rust](https://github.com/Open-Digital-Twin/dt-instance/workflows/Rust/badge.svg)

test2


## Commands

### DB

#### Run DB
mkdir /var/local/cassandra
docker run --name cassandradb -v /var/local/cassandra:/var/lib/cassandra -p 9042:9042 -d cassandra:latest

#### Remove DB
docker stop cassandradb && docker rm $_

### Start
docker ps -a
docker start ID

### Bash in container
docker exec -it ID /bin/bash
