#!/bin/bash
# Find large files in git history

cd /mnt/f/Node/hivellm/transmutation

echo "Finding large files in git history (>10MB)..."
echo ""

git rev-list --objects --all | \
git cat-file --batch-check='%(objecttype) %(objectname) %(objectsize) %(rest)' | \
sed -n 's/^blob //p' | \
sort --numeric-sort --key=2 -r | \
head -20 | \
awk '{
    size=$2
    name=$3
    for(i=4;i<=NF;i++) name=name" "$i
    
    if(size > 1048576) {
        printf "%6.2f MB - %s\n", size/1048576, name
    }
}'

