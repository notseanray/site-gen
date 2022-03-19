#!/bin/bash
touch result
curl \
 -H "Accept: application/vnd.github.v3+json" \
 "https://api.github.com/users/notseanray/repos?per_page=100" | tee -a > result
mkdir backups
cp ../content/projects.sml ./backups 
name=$(date)
mv ./backups/projects.sml ./backups/${name//" "/"-"}
python gen.py && cp projects.sml ../content/
