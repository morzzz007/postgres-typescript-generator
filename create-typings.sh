#!/bin/bash

cp /exchange/psql-typings.toml /app/ 2>/dev/null || echo "NO EXTRA TYPINGS"
/app/postgres-typescript-generator
cp /app/types.d.ts /exchange/