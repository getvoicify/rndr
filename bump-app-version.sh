#!/bin/bash

# This script will update the version of a Tauri app's tauri.conf.json

# Function to extract version components
get_version_components() {
  IFS='.' read -ra components <<< "$1"
  major=${components[0]}
  minor=${components[1]}
  patch=${components[2]}
}

compare_versions() {
  if [[ $1 -gt $2 ]]; then
      echo "$EXISTING_VERSION is greater than $NEW_VERSION"
      exit 1;
    elif [[ $1 -lt $2 ]]; then
      echo "$EXISTING_VERSION is less than $NEW_VERSION"
    else
      echo "$EXISTING_VERSION is equal to $NEW_VERSION"
  fi
}

# Exit if any command fails
set -e

# Check if the user provided a new version
if [ $# -eq 0 ]; then
  echo "No new version provided"
  exit 1
fi

# Get the new version number
NEW_VERSION=$1

# Get the path to the tauri.conf.json
TAURI_CONF="$(pwd)/src-tauri/tauri.conf.json"

# Get the existing version from tauri.conf.json
EXISTING_VERSION=$(jq -r .package.version "$TAURI_CONF")

get_version_components "$EXISTING_VERSION"
major1=$major
minor1=$minor
patch1=$patch

get_version_components "$NEW_VERSION"
major2=$major
minor2=$minor
patch2=$patch

compare_versions "$major1" "$major2"
compare_versions "$minor1" "$minor2"
compare_versions "$patch1" "$patch2"

echo "Updating $TAURI_CONF"
echo "Updating version from $EXISTING_VERSION to $NEW_VERSION"

# Update the existing version with the new version
#sed -i "s/\"version\": \"$EXISTING_VERSION\",/\"version\": \"$NEW_VERSION\",/" "$TAURI_CONF" || echo "Failed to update version in $TAURI_CONF" && exit 1

node ./version-patcher.cjs "$NEW_VERSION"

# Output the new version
echo "Tauri app version successfully updated to $NEW_VERSION"