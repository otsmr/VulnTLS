#!/bin/bash

cd src/

echo "##########################"
echo "#  SERVER CERTIFICATE    #"
echo "##########################"
openssl ecparam -out server.key -name prime256v1 -genkey
openssl req -new -key server.key -x509 -nodes -days 365 -out server.cert

echo "##########################"
echo "#  CLIENT CERTIFICATE    #"
echo "##########################"

echo " --- (1) Create CA keys --- \n"
openssl ecparam -out ca.key -name prime256v1 -genkey
echo " --- (1.2) Create CA cert --- \n"
openssl req -new -key ca.key -x509 -nodes -days 365 -out ca.cert

cd ..

echo " --- (2) Create Client keys --- \n"
openssl ecparam -out client.key -name prime256v1 -genkey
echo " --- (2.2) Create Client cert --- \n"
openssl req -new -key client.key -out client.cert

echo " --- (3) Sign client cert ---\n"
openssl x509 -req -in client.cert -days 365 -CA src/ca.cert -CAkey src/ca.key -CAcreateserial -out client.signed.cert

