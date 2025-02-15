#!/bin/sh

# Script is using jwt.sh project to create JWT
# URL: https://github.com/gasconleon/jwt.sh

JWT_SH_URL="https://raw.githubusercontent.com/gasconleon/jwt.sh/refs/heads/master/jwt.sh"
PRIVATE_KEY_FILE="jwtRS256.key"
ALGORITHM="RS256"
ISSUER=test

CURRENT_TIMESTAMP=$(date +%s)
JWT_EXP_TIMESTAMP=$((CURRENT_TIMESTAMP + 3600))

echo "Enter client_alias_id of some user ( choose from groups_clients table ): "
read caid

echo "Enter group_id of your choosen user: "
read gid

curl -sSL $JWT_SH_URL | bash -s -- \
-a $ALGORITHM \
-P $PRIVATE_KEY_FILE \
-H "{\"alg\": \"$ALGORITHM\", \"typ\": \"JWT\",\"iss\": \"$ISSUER\"}" \
-p "{\"caid\": $caid, \"gid\": $gid, \"iss\": \"$ISSUER\", \"exp\": $JWT_EXP_TIMESTAMP, \"sub\": \"\"}"