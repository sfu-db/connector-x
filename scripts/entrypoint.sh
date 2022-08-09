#!/bin/sh

echo "Waiting for db..."

while ! nc -z pgDB $1; do
  echo $1
  sleep 0.1
done

echo "DB opened for connections"