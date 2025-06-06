#!/bin/bash

# Claude Profile Management Script
# This script manages system prompts for Claude by applying profiles from claude-bank
# or resetting the current profile.

set -e

SYSTEM_PROMPT_LOCATION="$HOME/.claude/CLAUDE.md"
CLAUDE_BANK_DIR="$HOME/Documents/prompts/claude-bank"

show_usage() {
    echo "Usage: $0 <command> [arguments]"
    echo ""
    echo "Commands:"
    echo "  apply <path>    Apply a prompt from claude-bank (e.g., 'backend/plan')"
    echo "  reset           Reset/clear the current Claude profile"
    echo ""
    echo "Examples:"
    echo "  $0 apply backend/plan"
    echo "  $0 apply frontend/exec"
    echo "  $0 reset"
}

apply_profile() {
    local profile_path="$1"
    
    if [[ -z "$profile_path" ]]; then
        echo "Error: Profile path is required for apply command" >&2
        show_usage
        exit 1
    fi
    
    local source_file="${CLAUDE_BANK_DIR}/${profile_path}.md"
    
    if [[ ! -f "$source_file" ]]; then
        echo "Error: Profile '$profile_path' not found at $source_file" >&2
        echo "Available profiles:"
        find "$CLAUDE_BANK_DIR" -name "*.md" -type f | sed "s|$CLAUDE_BANK_DIR/||" | sed 's/\.md$//' | sort
        exit 1
    fi
    
    # Create .claude directory if it doesn't exist
    mkdir -p "$(dirname "$SYSTEM_PROMPT_LOCATION")"
    
    # Copy the profile to CLAUDE.md
    if cp "$source_file" "$SYSTEM_PROMPT_LOCATION"; then
        echo "Successfully applied profile '$profile_path' to $SYSTEM_PROMPT_LOCATION"
    else
        echo "Error: Failed to apply profile '$profile_path'" >&2
        exit 1
    fi
}

reset_profile() {
    if [[ -f "$SYSTEM_PROMPT_LOCATION" ]]; then
        if rm "$SYSTEM_PROMPT_LOCATION"; then
            echo "Successfully reset Claude profile (removed $SYSTEM_PROMPT_LOCATION)"
        else
            echo "Error: Failed to remove $SYSTEM_PROMPT_LOCATION" >&2
            exit 1
        fi
    else
        echo "No Claude profile found at $SYSTEM_PROMPT_LOCATION (already reset)"
    fi
}

# Main script logic
if [[ $# -eq 0 ]]; then
    echo "Error: No command specified" >&2
    show_usage
    exit 1
fi

case "$1" in
    apply)
        apply_profile "$2"
        ;;
    reset)
        reset_profile
        ;;
    -h|--help|help)
        show_usage
        ;;
    *)
        echo "Error: Unknown command '$1'" >&2
        show_usage
        exit 1
        ;;
esac
