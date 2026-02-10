#!/bin/bash

echo "Sending a GET request to /echo"
curl -X GET http://0.0.0.0:3000/echo

echo "\nSending a POST request to /echo with some data"
curl -X POST -H "Content-Type: application/json" -d '{"message": "Hello from test script"}' http://0.0.0.0:3000/echo
