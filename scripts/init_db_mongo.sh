#!/bin/bash
set -e

DB_USERNAME=zero2prod
DB_PASSWORD=password

if [[ -z "${SKIP_DOCKER}" ]]; then
    docker run -d --name mongodb -p 27017:27017 -e MONGO_INITDB_ROOT_USERNAME=$DB_USERNAME -e MONGO_INITDB_ROOT_PASSWORD=$DB_PASSWORD mongo:latest

    echo "Waiting for MongoDB to start..."
    sleep 7
fi

docker exec -i mongodb mongosh --username $DB_USERNAME --password $DB_PASSWORD <<EOF
use newsletter 
db.createCollection("subscriptions")
EOF