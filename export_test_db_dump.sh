#/bin/bash

PGPASSWORD=postgres pg_dump -Fc --no-acl --no-owner -h localhost -U postgres postgres > basiliq_test_db.dump
