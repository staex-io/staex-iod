#!/bin/sh

for file in /migrations/*.sql; do
  npx sql-formatter -l sqlite --fix "${file}"
done
