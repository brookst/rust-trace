#!/bin/bash

# find relevent directory paths
GIT_TOPLEVEL=$(git rev-parse --show-toplevel)
HOOK_DIR=$GIT_TOPLEVEL/.git/hooks
SCRIPT_DIR=$GIT_TOPLEVEL/scripts

# list scripts (but not this one)
SCRIPTS=$(ls $SCRIPT_DIR | grep -v $(basename $0))
for script in $SCRIPTS; do
    # link script into hooks directory of this git repo
    ln -sv ../../${SCRIPT_DIR#$GIT_TOPLEVEL/}/$script $HOOK_DIR
done
