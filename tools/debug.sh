#!/bin/bash

# shellcheck source=tools/config.sh
. "$(dirname "$0")"/config.sh

cd "$SRC_DIR"

SESSION=debug-os
GDB_PATH=gdb
tmux new-session -d -s "$SESSION" "make clean && make debug"
GDB_PANE=$(tmux split-window -h -t "$SESSION" -P -F '#{pane_id}')
tmux send-keys -t "$GDB_PANE" "$GDB_PATH -iex \"add-auto-load-safe-path $SRC_DIR/.gdbinit\" $BUILD_DIR/kernel.elf" C-m
tmux attach-session -t "$SESSION"
cd -

