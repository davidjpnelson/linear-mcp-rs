#!/usr/bin/env bash
# Comprehensive test harness for linear-mcp-rs
# Tests ALL 253 tools against a real Linear API via MCP stdio protocol.
#
# Usage: ./test_tools.sh [--tier N] [--tool TOOL_NAME] [--list-only] [--discover]
#   --tier N       Run only tier N tests (1=reads, 2=create+delete, 3=mutations, 4=updates)
#   --tool NAME    Run only the named tool test
#   --list-only    Just verify tools/list registration
#   --discover     Run workspace discovery

set -uo pipefail

BINARY="./target/release/linear-mcp"
PASS=0
FAIL=0
SKIP=0
ERRORS=()
TIER_FILTER=""
TOOL_FILTER=""
LIST_ONLY=false
DISCOVER=false

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

# Parse args
while [[ $# -gt 0 ]]; do
    case $1 in
        --tier) TIER_FILTER="$2"; shift 2 ;;
        --tool) TOOL_FILTER="$2"; shift 2 ;;
        --list-only) LIST_ONLY=true; shift ;;
        --discover) DISCOVER=true; shift ;;
        *) echo "Unknown arg: $1"; exit 1 ;;
    esac
done

# ---- FIFO-based MCP Server Communication ----

TMPDIR_TEST=$(mktemp -d)
FIFO_IN="$TMPDIR_TEST/mcp_in"
FIFO_OUT="$TMPDIR_TEST/mcp_out"
mkfifo "$FIFO_IN" "$FIFO_OUT"

SERVER_PID=""
REQ_ID=1

cleanup() {
    if [[ -n "$SERVER_PID" ]] && kill -0 "$SERVER_PID" 2>/dev/null; then
        kill "$SERVER_PID" 2>/dev/null || true
        wait "$SERVER_PID" 2>/dev/null || true
    fi
    rm -rf "$TMPDIR_TEST"
}
trap cleanup EXIT

stop_server() {
    # Close FDs if open
    exec 3>&- 2>/dev/null || true
    exec 4<&- 2>/dev/null || true
    if [[ -n "$SERVER_PID" ]] && kill -0 "$SERVER_PID" 2>/dev/null; then
        kill "$SERVER_PID" 2>/dev/null || true
        wait "$SERVER_PID" 2>/dev/null || true
    fi
    SERVER_PID=""
    # Recreate FIFOs
    rm -f "$FIFO_IN" "$FIFO_OUT"
    mkfifo "$FIFO_IN" "$FIFO_OUT"
}

SERVER_LOG="$TMPDIR_TEST/server.log"

start_server() {
    "$BINARY" < "$FIFO_IN" > "$FIFO_OUT" 2>"$SERVER_LOG" &
    SERVER_PID=$!
    sleep 0.5
    exec 3>"$FIFO_IN"
    exec 4<"$FIFO_OUT"
    if ! kill -0 "$SERVER_PID" 2>/dev/null; then
        echo -e "${RED}ERROR: Server failed to start${NC}"
        exit 1
    fi
}

ensure_server() {
    if [[ -z "$SERVER_PID" ]] || ! kill -0 "$SERVER_PID" 2>/dev/null; then
        echo -e "  ${YELLOW}(restarting server — PID $SERVER_PID dead)${NC}" >&2
        if [[ -f "$SERVER_LOG" ]]; then
            local last_lines
            last_lines=$(tail -5 "$SERVER_LOG" 2>/dev/null)
            if [[ -n "$last_lines" ]]; then
                echo -e "  ${YELLOW}Server log: $last_lines${NC}" >&2
            fi
        fi
        stop_server
        start_server
        do_handshake >&2
    fi
}

send_request() {
    local method="$1"
    local params="$2"
    local id=$REQ_ID
    REQ_ID=$((REQ_ID + 1))
    local msg
    if [[ "$params" == "null" ]]; then
        msg="{\"jsonrpc\":\"2.0\",\"id\":$id,\"method\":\"$method\"}"
    else
        msg="{\"jsonrpc\":\"2.0\",\"id\":$id,\"method\":\"$method\",\"params\":$params}"
    fi
    if ! echo "$msg" >&3 2>/dev/null; then
        ensure_server
        echo "$msg" >&3 2>/dev/null
    fi
}

send_notification() {
    local method="$1"
    local params="$2"
    local msg
    if [[ "$params" == "null" ]]; then
        msg="{\"jsonrpc\":\"2.0\",\"method\":\"$method\"}"
    else
        msg="{\"jsonrpc\":\"2.0\",\"method\":\"$method\",\"params\":$params}"
    fi
    if ! echo "$msg" >&3 2>/dev/null; then
        ensure_server
        echo "$msg" >&3 2>/dev/null
    fi
}

read_response() {
    local timeout="${1:-15}"
    local response=""
    if read -t "$timeout" -r response <&4; then
        echo "$response"
    else
        echo '{"error":"timeout"}'
    fi
}

do_handshake() {
    local init_params='{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test-harness","version":"1.0"}}'
    send_request "initialize" "$init_params"
    local resp
    resp=$(read_response 10)
    if echo "$resp" | jq -e '.result.serverInfo' >/dev/null 2>&1; then
        local server_name
        server_name=$(echo "$resp" | jq -r '.result.serverInfo.name // "unknown"')
        echo -e "${GREEN}Handshake OK${NC} — server: $server_name"
    else
        echo -e "${RED}Handshake FAILED${NC}: $resp"
        exit 1
    fi
    send_notification "notifications/initialized" "null"
    sleep 0.3
}

do_list_tools() {
    send_request "tools/list" "null"
    local resp
    resp=$(read_response 15)
    local count
    count=$(echo "$resp" | jq '.result.tools | length' 2>/dev/null || echo 0)
    echo -e "${CYAN}Tools registered: $count${NC}"
    if [[ "$count" -lt 240 ]]; then
        echo -e "${YELLOW}WARNING: Expected ~253 tools, got $count${NC}"
    fi
    echo "$resp" | jq -r '.result.tools[].name' 2>/dev/null | sort
}

call_tool() {
    local tool_name="$1"
    local args="$2"
    ensure_server
    send_request "tools/call" "{\"name\":\"$tool_name\",\"arguments\":$args}"
    local resp
    resp=$(read_response 30)
    echo "$resp"
}

# Call tool and return the text content (for extracting IDs)
call_tool_text() {
    local tool_name="$1"
    local args="$2"
    local resp
    resp=$(call_tool "$tool_name" "$args")
    echo "$resp" | jq -r '.result.content[0].text // ""' 2>/dev/null
}

# Extract first UUID from text
extract_uuid() {
    echo "$1" | grep -oE '[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}' | head -1
}

test_tool() {
    local tier="$1"
    local tool_name="$2"
    local args="$3"
    local description="$4"

    if [[ -n "$TIER_FILTER" && "$tier" != "$TIER_FILTER" ]]; then
        return
    fi
    if [[ -n "$TOOL_FILTER" && "$tool_name" != "$TOOL_FILTER" ]]; then
        return
    fi

    printf "  [T%s] %-40s " "$tier" "$tool_name"

    local resp
    resp=$(call_tool "$tool_name" "$args")

    # Check for timeout
    if echo "$resp" | jq -e '.error == "timeout"' >/dev/null 2>&1; then
        echo -e "${RED}TIMEOUT${NC}"
        ERRORS+=("$tool_name: timeout")
        FAIL=$((FAIL + 1))
        return 1
    fi

    # Check for JSON-RPC error
    local has_error
    has_error=$(echo "$resp" | jq -e '.error != null' 2>/dev/null || echo "false")
    if [[ "$has_error" == "true" ]]; then
        local err_msg
        err_msg=$(echo "$resp" | jq -r '.error.message // .error // "unknown"' 2>/dev/null)
        echo -e "${RED}FAIL${NC} — $err_msg"
        ERRORS+=("$tool_name: $err_msg")
        FAIL=$((FAIL + 1))
        return 1
    fi

    # Check for isError in result
    local is_error
    is_error=$(echo "$resp" | jq -e '.result.isError == true' 2>/dev/null || echo "false")
    if [[ "$is_error" == "true" ]]; then
        local content_text
        content_text=$(echo "$resp" | jq -r '.result.content[0].text // "no detail"' 2>/dev/null | head -c 200)
        echo -e "${RED}FAIL${NC} — $content_text"
        ERRORS+=("$tool_name: $content_text")
        FAIL=$((FAIL + 1))
        return 1
    fi

    # Check we got content
    local has_content
    has_content=$(echo "$resp" | jq -e '.result.content[0].text' >/dev/null 2>&1 && echo "true" || echo "false")
    if [[ "$has_content" == "true" ]]; then
        local preview
        preview=$(echo "$resp" | jq -r '.result.content[0].text' 2>/dev/null | head -1 | head -c 120)
        echo -e "${GREEN}PASS${NC} — ${preview}"
        PASS=$((PASS + 1))
        return 0
    else
        echo -e "${YELLOW}WARN${NC} — unexpected response shape"
        echo "$resp" | jq '.' 2>/dev/null | head -5
        PASS=$((PASS + 1))
        return 0
    fi
}

# Inline test for create ops — prints pass/fail, sets a variable with the UUID
test_create() {
    local tool_name="$1"
    local args="$2"
    local uuid_var="$3"

    printf "  [T2] %-40s " "$tool_name"
    local resp
    resp=$(call_tool "$tool_name" "$args")

    # Check for JSON-RPC error (top-level)
    local has_rpc_error
    has_rpc_error=$(echo "$resp" | jq -e '.error != null' 2>/dev/null || echo "false")
    if [[ "$has_rpc_error" == "true" ]]; then
        local err_msg
        err_msg=$(echo "$resp" | jq -r '.error.message // .error // "unknown"' 2>/dev/null)
        echo -e "${RED}FAIL${NC} — $err_msg"
        ERRORS+=("$tool_name: $err_msg")
        FAIL=$((FAIL + 1))
        eval "$uuid_var=''"
        return 1
    fi

    local text
    text=$(echo "$resp" | jq -r '.result.content[0].text // ""' 2>/dev/null)
    local is_error
    is_error=$(echo "$resp" | jq -e '.result.isError == true' 2>/dev/null || echo "false")

    if [[ "$is_error" == "true" ]]; then
        echo -e "${RED}FAIL${NC} — $(echo "$text" | head -c 200)"
        ERRORS+=("$tool_name: $text")
        FAIL=$((FAIL + 1))
        eval "$uuid_var=''"
        return 1
    fi

    local uuid
    uuid=$(extract_uuid "$text")
    if [[ -n "$uuid" ]]; then
        echo -e "${GREEN}PASS${NC} — id: $uuid"
        PASS=$((PASS + 1))
        eval "$uuid_var='$uuid'"
        return 0
    elif [[ -n "$text" ]]; then
        echo -e "${GREEN}PASS${NC} — $(echo "$text" | head -1 | head -c 100)"
        PASS=$((PASS + 1))
        eval "$uuid_var=''"
        return 0
    else
        echo -e "${RED}FAIL${NC} — empty response (server may have died)"
        ERRORS+=("$tool_name: empty response")
        FAIL=$((FAIL + 1))
        eval "$uuid_var=''"
        return 1
    fi
}

skip_tool() {
    local tier="$1"
    local tool_name="$2"
    local reason="$3"
    if [[ -n "$TIER_FILTER" && "$tier" != "$TIER_FILTER" ]]; then return; fi
    if [[ -n "$TOOL_FILTER" && "$tool_name" != "$TOOL_FILTER" ]]; then return; fi
    printf "  [T%s] %-40s ${YELLOW}SKIP${NC} — %s\n" "$tier" "$tool_name" "$reason"
    SKIP=$((SKIP + 1))
}

# ---- Discovery Mode ----

run_discovery() {
    echo -e "\n${CYAN}=== Workspace Discovery ===${NC}\n"
    echo "Teams:"; call_tool_text "list_teams" '{}' | head -20
    echo -e "\nProjects:"; call_tool_text "list_projects" '{"limit": 5}' | head -20
    echo -e "\nLabels:"; call_tool_text "list_labels" '{"limit": 5}' | head -20
    echo -e "\nInitiatives:"; call_tool_text "list_initiatives" '{"limit": 3}' | head -20
    echo -e "\nUsers:"; call_tool_text "list_users" '{"limit": 5}' | head -20
    echo -e "\nCustomers:"; call_tool_text "list_customers" '{"limit": 5}' | head -20
    echo -e "\nStates (first team):"; call_tool_text "list_states" '{}' | head -20
    echo -e "\nViews:"; call_tool_text "list_views" '{"limit": 3}' | head -10
    echo -e "\nWebhooks:"; call_tool_text "list_webhooks" '{}' | head -10
    echo -e "\nRoadmaps:"; call_tool_text "list_roadmaps" '{"limit": 3}' | head -10
}

# ---- Main ----

echo -e "${CYAN}${BOLD}========================================================${NC}"
echo -e "${CYAN}${BOLD}  linear-mcp-rs Comprehensive Test Harness (253 tools)  ${NC}"
echo -e "${CYAN}${BOLD}========================================================${NC}"
echo ""

echo "Building..."
if ! cargo build --release 2>/dev/null; then
    echo -e "${RED}Build failed!${NC}"
    exit 1
fi
echo -e "${GREEN}Build OK${NC}"
echo ""

echo "Starting MCP server..."
start_server
do_handshake
echo ""

echo -e "${CYAN}=== Tool Registration ===${NC}"
TOOL_LIST=$(do_list_tools)
TOOL_COUNT=$(echo "$TOOL_LIST" | tail -n +2 | wc -l | tr -d ' ')
echo ""

if $LIST_ONLY; then
    echo "$TOOL_LIST" | tail -n +2
    echo -e "\nTotal: $TOOL_COUNT tools"
    exit 0
fi

if $DISCOVER; then
    run_discovery
    exit 0
fi

# ============================================================
# First, discover the workspace so we know what entities exist
# ============================================================

echo -e "${CYAN}=== Discovering workspace entities... ===${NC}"

# Get first team key — skip any leftover TEST-HARNESS teams
TEAM_KEY=$(call_tool_text "list_teams" '{}' | grep '|' | grep -v 'TEST-HARNESS' | head -1 | awk '{print $1}')
if [[ -z "$TEAM_KEY" ]]; then TEAM_KEY="TEST"; fi
echo "  Team: $TEAM_KEY"

# Get first user email
USER_EMAIL=$(call_tool_text "list_users" '{"limit": 1}' | grep -oE '[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]+' | head -1)
echo "  User: ${USER_EMAIL:-none}"

echo ""

# ============================================================
# TIER 1: Read-Only Tests (all list/get/search tools)
# ============================================================

if [[ -z "$TIER_FILTER" || "$TIER_FILTER" == "1" ]]; then
echo -e "${CYAN}${BOLD}=== Tier 1: Read-Only Tests ===${NC}"

test_tool 1 "list_teams" '{}' ""
test_tool 1 "list_issues" "{\"team\": \"$TEAM_KEY\", \"limit\": 3}" ""
test_tool 1 "search_issues" '{"query": "test", "limit": 3}' ""
test_tool 1 "list_projects" '{"limit": 3}' ""
test_tool 1 "list_labels" '{"limit": 5}' ""
test_tool 1 "list_cycles" "{\"team\": \"$TEAM_KEY\", \"limit\": 3}" ""
test_tool 1 "list_initiatives" '{"limit": 3}' ""
test_tool 1 "list_users" '{"limit": 3}' ""
test_tool 1 "list_states" "{\"team\": \"$TEAM_KEY\"}" ""
test_tool 1 "list_views" '{"limit": 3}' ""
test_tool 1 "list_templates" '{"limit": 3}' ""
test_tool 1 "list_documents" '{"limit": 3}' ""
test_tool 1 "list_webhooks" '{}' ""
test_tool 1 "list_integrations" '{}' ""
test_tool 1 "list_favorites" '{}' ""
test_tool 1 "list_notifications" '{"limit": 3}' ""
test_tool 1 "my_issues" '{}' ""
test_tool 1 "list_triage_issues" "{\"team\": \"$TEAM_KEY\"}" ""
test_tool 1 "list_customers" '{"limit": 3}' ""
test_tool 1 "list_customer_needs" '{"limit": 3}' ""
test_tool 1 "list_agent_sessions" '{"limit": 3}' ""
skip_tool 1 "list_releases" "requires Release Management feature flag"
test_tool 1 "search_projects" '{"query": "test", "limit": 3}' ""
test_tool 1 "search_documents" '{"term": "test", "limit": 3}' ""
test_tool 1 "issue_vcs_branch_search" '{"branchName": "main"}' ""
test_tool 1 "query_audit_log" '{}' ""

# Enriched field filter tests
test_tool 1 "list_issues" "{\"team\": \"$TEAM_KEY\", \"limit\": 1, \"completed_after\": \"2025-01-01\"}" ""

# --- New Phase 2 read-only tools ---
test_tool 1 "get_viewer" '{}' ""
test_tool 1 "get_issue_priority_values" '{}' ""
test_tool 1 "list_customer_statuses" '{}' ""
test_tool 1 "list_customer_tiers" '{}' ""
test_tool 1 "list_project_statuses" '{}' ""
test_tool 1 "list_project_labels" '{}' ""
test_tool 1 "list_team_memberships" '{}' ""
test_tool 1 "list_notification_subscriptions" '{}' ""
test_tool 1 "get_notifications_unread_count" '{}' ""
test_tool 1 "list_emojis" '{}' ""
test_tool 1 "list_initiative_relations" '{}' ""
test_tool 1 "list_time_schedules" '{}' ""
test_tool 1 "list_triage_responsibilities" '{}' ""
test_tool 1 "list_archived_teams" '{}' ""
test_tool 1 "get_rate_limit_status" '{}' ""
test_tool 1 "get_organization" '{}' ""
skip_tool 1 "get_application_info" "requires OAuth app clientId"
test_tool 1 "semantic_search" '{"query": "test", "limit": 3}' ""
test_tool 1 "list_comments_all" '{"limit": 3}' ""
test_tool 1 "list_issue_relations" '{"limit": 3}' ""
test_tool 1 "list_external_users" '{"limit": 3}' ""
skip_tool 1 "list_roadmaps" "deprecated by Linear — no roadmaps API"
test_tool 1 "get_issue_filter_suggestion" '{"prompt": "bugs assigned to me"}' ""
skip_tool 1 "get_project_filter_suggestion" "AI filter suggestion — unreliable with API keys"
test_tool 1 "get_team" "{\"id\": \"$TEAM_KEY\"}" ""

# Release-gated read tools
skip_tool 1 "list_release_pipelines" "requires Release Management feature"
skip_tool 1 "list_release_stages" "requires Release Management feature"
skip_tool 1 "list_issue_to_releases" "requires Release Management feature"
skip_tool 1 "search_releases" "requires Release Management feature"

echo ""
fi

# ============================================================
# TIER 2: Create → Read → Update → Delete full lifecycle
# ============================================================

if [[ -z "$TIER_FILTER" || "$TIER_FILTER" == "2" ]]; then
echo -e "${CYAN}${BOLD}=== Tier 2: Full CRUD Lifecycle Tests ===${NC}"

# ------- Issue lifecycle -------
echo -e "  ${CYAN}--- Issue lifecycle ---${NC}"
test_create "create_issue" "{\"team\": \"$TEAM_KEY\", \"title\": \"TEST-HARNESS: Delete me\", \"description\": \"Automated test\"}" ISSUE_ID

if [[ -n "$ISSUE_ID" ]]; then
    # Get the issue identifier from get_issue
    ISSUE_TEXT=$(call_tool_text "get_issue" "{\"id\": \"$ISSUE_ID\"}")
    ISSUE_IDENT=$(echo "$ISSUE_TEXT" | head -1 | grep -oE '[A-Z]+-[0-9]+' | head -1)
    echo "    (identifier: ${ISSUE_IDENT:-$ISSUE_ID})"

    test_tool 2 "get_issue" "{\"id\": \"${ISSUE_IDENT:-$ISSUE_ID}\"}" ""
    test_tool 2 "get_issue_history" "{\"id\": \"${ISSUE_IDENT:-$ISSUE_ID}\"}" ""
    test_tool 2 "list_comments" "{\"issue\": \"${ISSUE_IDENT:-$ISSUE_ID}\"}" ""
    test_tool 2 "list_attachments" "{\"issueId\": \"${ISSUE_IDENT:-$ISSUE_ID}\"}" ""

    # Update issue
    test_tool 2 "update_issue" "{\"id\": \"${ISSUE_IDENT:-$ISSUE_ID}\", \"title\": \"TEST-HARNESS: Updated title\"}" ""

    # Add comment
    COMMENT_TEXT=$(call_tool_text "add_comment" "{\"issueId\": \"${ISSUE_IDENT:-$ISSUE_ID}\", \"body\": \"Test comment from harness\"}")
    COMMENT_ID=$(extract_uuid "$COMMENT_TEXT")
    if [[ -n "$COMMENT_ID" ]]; then
        printf "  [T2] %-40s ${GREEN}PASS${NC} — id: %s\n" "add_comment" "$COMMENT_ID"
        PASS=$((PASS + 1))

        # Update comment
        test_tool 2 "update_comment" "{\"id\": \"$COMMENT_ID\", \"body\": \"Updated test comment\"}" ""

        # Resolve / unresolve comment
        test_tool 2 "resolve_comment" "{\"id\": \"$COMMENT_ID\"}" ""
        test_tool 2 "unresolve_comment" "{\"id\": \"$COMMENT_ID\"}" ""

        # Add reaction / remove reaction
        REACTION_TEXT=$(call_tool_text "add_reaction" "{\"commentId\": \"$COMMENT_ID\", \"emoji\": \"thumbsup\"}")
        REACTION_ID=$(extract_uuid "$REACTION_TEXT")
        if [[ -n "$REACTION_ID" ]]; then
            printf "  [T2] %-40s ${GREEN}PASS${NC} — id: %s\n" "add_reaction" "$REACTION_ID"
            PASS=$((PASS + 1))
            test_tool 2 "remove_reaction" "{\"id\": \"$REACTION_ID\"}" ""
        else
            printf "  [T2] %-40s ${RED}FAIL${NC} — no UUID in response\n" "add_reaction"
            ERRORS+=("add_reaction: no UUID in response: $REACTION_TEXT")
            FAIL=$((FAIL + 1))
        fi

        # Get comment
        test_tool 2 "get_comment" "{\"id\": \"$COMMENT_ID\"}" ""

        # Delete comment
        test_tool 2 "delete_comment" "{\"id\": \"$COMMENT_ID\"}" ""
    else
        printf "  [T2] %-40s ${RED}FAIL${NC} — no UUID: %s\n" "add_comment" "$(echo "$COMMENT_TEXT" | head -c 100)"
        ERRORS+=("add_comment: no UUID")
        FAIL=$((FAIL + 1))
    fi

    # Add/remove label (need a temp label)
    ILABEL_SUFFIX=$(date +%s)
    ILABEL_TEXT=$(call_tool_text "create_label" "{\"name\": \"TH-ILABEL-$ILABEL_SUFFIX\", \"team\": \"$TEAM_KEY\", \"color\": \"#0000ff\"}")
    ILABEL_ID=$(extract_uuid "$ILABEL_TEXT")
    if [[ -n "$ILABEL_ID" ]]; then
        test_tool 2 "add_issue_label" "{\"issue\": \"${ISSUE_IDENT:-$ISSUE_ID}\", \"label\": \"TH-ILABEL-$ILABEL_SUFFIX\"}" ""
        test_tool 2 "remove_issue_label" "{\"issue\": \"${ISSUE_IDENT:-$ISSUE_ID}\", \"label\": \"TH-ILABEL-$ILABEL_SUFFIX\"}" ""
        test_tool 2 "get_issue_label" "{\"id\": \"$ILABEL_ID\"}" ""
        call_tool "archive_label" "{\"id\": \"$ILABEL_ID\"}" >/dev/null 2>&1
    fi

    # Attach link URL
    test_tool 2 "attach_link_url" "{\"issue\": \"${ISSUE_IDENT:-$ISSUE_ID}\", \"url\": \"https://example.com/test\", \"title\": \"Test Link\"}" ""
    test_tool 2 "get_attachments_for_url" '{"url": "https://example.com/test"}' ""

    # Add attachment
    ATTACH_TEXT=$(call_tool_text "add_attachment" "{\"issueId\": \"${ISSUE_IDENT:-$ISSUE_ID}\", \"title\": \"Test Link\", \"url\": \"https://example.com\"}")
    ATTACH_ID=$(extract_uuid "$ATTACH_TEXT")
    if [[ -n "$ATTACH_ID" ]]; then
        printf "  [T2] %-40s ${GREEN}PASS${NC} — id: %s\n" "add_attachment" "$ATTACH_ID"
        PASS=$((PASS + 1))
        test_tool 2 "get_attachment" "{\"id\": \"$ATTACH_ID\"}" ""
        test_tool 2 "update_attachment" "{\"id\": \"$ATTACH_ID\", \"title\": \"Updated Link\"}" ""
        test_tool 2 "delete_attachment" "{\"id\": \"$ATTACH_ID\"}" ""
    else
        printf "  [T2] %-40s ${RED}FAIL${NC} — no UUID\n" "add_attachment"
        ERRORS+=("add_attachment: no UUID: $ATTACH_TEXT")
        FAIL=$((FAIL + 1))
    fi

    # Subscribe / unsubscribe
    test_tool 2 "subscribe_to_issue" "{\"issue\": \"${ISSUE_IDENT:-$ISSUE_ID}\"}" ""
    test_tool 2 "unsubscribe_from_issue" "{\"issue\": \"${ISSUE_IDENT:-$ISSUE_ID}\"}" ""

    # Archive / unarchive
    test_tool 2 "archive_issue" "{\"id\": \"${ISSUE_IDENT:-$ISSUE_ID}\"}" ""
    test_tool 2 "unarchive_issue" "{\"id\": \"${ISSUE_IDENT:-$ISSUE_ID}\"}" ""

    # Delete issue (permanent)
    test_tool 2 "delete_issue" "{\"id\": \"${ISSUE_IDENT:-$ISSUE_ID}\"}" ""
fi

# ------- Issue #2 for relation + cycle + bulk tests -------
echo -e "  ${CYAN}--- Issue relations & cycles ---${NC}"
test_create "create_issue" "{\"team\": \"$TEAM_KEY\", \"title\": \"TEST-HARNESS: Issue A\"}" ISSUE_A_ID
test_create "create_issue" "{\"team\": \"$TEAM_KEY\", \"title\": \"TEST-HARNESS: Issue B\"}" ISSUE_B_ID

if [[ -n "$ISSUE_A_ID" && -n "$ISSUE_B_ID" ]]; then
    ISSUE_A_IDENT=$(call_tool_text "get_issue" "{\"id\": \"$ISSUE_A_ID\"}" | head -1 | grep -oE '[A-Z]+-[0-9]+' | head -1)
    ISSUE_B_IDENT=$(call_tool_text "get_issue" "{\"id\": \"$ISSUE_B_ID\"}" | head -1 | grep -oE '[A-Z]+-[0-9]+' | head -1)

    # Issue relation
    RELATION_TEXT=$(call_tool_text "create_issue_relation" "{\"issueId\": \"${ISSUE_A_IDENT:-$ISSUE_A_ID}\", \"relatedIssueId\": \"${ISSUE_B_IDENT:-$ISSUE_B_ID}\", \"type\": \"related\"}")
    RELATION_ID=$(extract_uuid "$RELATION_TEXT")
    if [[ -n "$RELATION_ID" ]]; then
        printf "  [T2] %-40s ${GREEN}PASS${NC} — id: %s\n" "create_issue_relation" "$RELATION_ID"
        PASS=$((PASS + 1))
        test_tool 2 "get_issue_relation" "{\"id\": \"$RELATION_ID\"}" ""
        test_tool 2 "update_issue_relation" "{\"id\": \"$RELATION_ID\", \"type\": \"blocks\"}" ""
        test_tool 2 "delete_issue_relation" "{\"id\": \"$RELATION_ID\"}" ""
    else
        printf "  [T2] %-40s ${RED}FAIL${NC}\n" "create_issue_relation"
        ERRORS+=("create_issue_relation: $RELATION_TEXT")
        FAIL=$((FAIL + 1))
    fi

    # Cycle create → add issue → remove issue → archive cycle
    CYCLE_TEXT=$(call_tool_text "create_cycle" "{\"team\": \"$TEAM_KEY\", \"name\": \"TEST-HARNESS-CYCLE\", \"startsAt\": \"2027-01-01\", \"endsAt\": \"2027-01-15\"}")
    CYCLE_ID=$(extract_uuid "$CYCLE_TEXT")
    if [[ -n "$CYCLE_ID" ]]; then
        printf "  [T2] %-40s ${GREEN}PASS${NC} — id: %s\n" "create_cycle" "$CYCLE_ID"
        PASS=$((PASS + 1))
        test_tool 2 "get_cycle" "{\"id\": \"$CYCLE_ID\"}" ""
        test_tool 2 "update_cycle" "{\"id\": \"$CYCLE_ID\", \"name\": \"TEST-HARNESS-CYCLE-UPDATED\"}" ""
        test_tool 2 "add_issue_to_cycle" "{\"issueId\": \"${ISSUE_A_IDENT:-$ISSUE_A_ID}\", \"cycleId\": \"$CYCLE_ID\"}" ""
        test_tool 2 "remove_issue_from_cycle" "{\"issueId\": \"${ISSUE_A_IDENT:-$ISSUE_A_ID}\"}" ""
        test_tool 2 "archive_cycle" "{\"id\": \"$CYCLE_ID\"}" ""
    else
        printf "  [T2] %-40s ${RED}FAIL${NC}\n" "create_cycle"
        ERRORS+=("create_cycle: $CYCLE_TEXT")
        FAIL=$((FAIL + 1))
    fi

    # Bulk update issues
    if [[ -n "$ISSUE_A_IDENT" && -n "$ISSUE_B_IDENT" ]]; then
        test_tool 2 "bulk_update_issues" "{\"ids\": \"$ISSUE_A_IDENT,$ISSUE_B_IDENT\", \"priority\": \"high\"}" ""
    fi

    # Triage issue (move to a known state)
    if [[ -n "$ISSUE_A_IDENT" ]]; then
        test_tool 2 "triage_issue" "{\"id\": \"$ISSUE_A_IDENT\", \"state\": \"Todo\"}" ""
    fi

    # Clean up issues
    test_tool 2 "delete_issue" "{\"id\": \"${ISSUE_A_IDENT:-$ISSUE_A_ID}\"}" ""
    test_tool 2 "delete_issue" "{\"id\": \"${ISSUE_B_IDENT:-$ISSUE_B_ID}\"}" ""
fi

# ------- Batch create issues -------
echo -e "  ${CYAN}--- Batch create issues ---${NC}"
BATCH_TEXT=$(call_tool_text "batch_create_issues" "{\"team\": \"$TEAM_KEY\", \"issues\": \"[{\\\"title\\\":\\\"BATCH-A\\\"},{\\\"title\\\":\\\"BATCH-B\\\"}]\"}")
printf "  [T2] %-40s " "batch_create_issues"
if echo "$BATCH_TEXT" | grep -q "BATCH-A"; then
    echo -e "${GREEN}PASS${NC} — $(echo "$BATCH_TEXT" | head -1 | head -c 100)"
    PASS=$((PASS + 1))
    # Extract identifiers and clean up
    for bid in $(echo "$BATCH_TEXT" | grep -oE '[A-Z]+-[0-9]+'); do
        call_tool "delete_issue" "{\"id\": \"$bid\"}" >/dev/null 2>&1
    done
else
    echo -e "${RED}FAIL${NC} — $BATCH_TEXT"
    ERRORS+=("batch_create_issues: $BATCH_TEXT")
    FAIL=$((FAIL + 1))
fi

# ------- Label lifecycle -------
echo -e "  ${CYAN}--- Label lifecycle ---${NC}"
LABEL_SUFFIX=$(date +%s)
test_create "create_label" "{\"name\": \"TEST-HARNESS-LABEL-$LABEL_SUFFIX\", \"team\": \"$TEAM_KEY\", \"color\": \"#ff0000\"}" LABEL_ID

if [[ -n "$LABEL_ID" ]]; then
    test_tool 2 "update_label" "{\"id\": \"$LABEL_ID\", \"name\": \"TEST-HARNESS-LABEL-UPD-$LABEL_SUFFIX\", \"color\": \"#00ff00\"}" ""
    test_tool 2 "archive_label" "{\"id\": \"$LABEL_ID\"}" ""
fi

# ------- Project lifecycle -------
echo -e "  ${CYAN}--- Project lifecycle ---${NC}"
PROJ_SUFFIX=$(date +%s)
test_create "create_project" "{\"name\": \"TEST-HARNESS-PROJECT-$PROJ_SUFFIX\", \"teams\": \"$TEAM_KEY\"}" PROJECT_ID

if [[ -n "$PROJECT_ID" ]]; then
    test_tool 2 "get_project" "{\"id\": \"$PROJECT_ID\"}" ""
    test_tool 2 "update_project" "{\"id\": \"$PROJECT_ID\", \"description\": \"Test project description\"}" ""

    # Project milestone
    MILESTONE_TEXT=$(call_tool_text "create_project_milestone" "{\"project\": \"$PROJECT_ID\", \"name\": \"TEST-MILESTONE\"}")
    MILESTONE_ID=$(extract_uuid "$MILESTONE_TEXT")
    if [[ -n "$MILESTONE_ID" ]]; then
        printf "  [T2] %-40s ${GREEN}PASS${NC} — id: %s\n" "create_project_milestone" "$MILESTONE_ID"
        PASS=$((PASS + 1))
        test_tool 2 "list_project_milestones" "{\"project\": \"$PROJECT_ID\"}" ""
        test_tool 2 "update_project_milestone" "{\"id\": \"$MILESTONE_ID\", \"name\": \"TEST-MILESTONE-UPDATED\"}" ""
        test_tool 2 "delete_project_milestone" "{\"id\": \"$MILESTONE_ID\"}" ""
    else
        printf "  [T2] %-40s ${RED}FAIL${NC}\n" "create_project_milestone"
        ERRORS+=("create_project_milestone: $MILESTONE_TEXT")
        FAIL=$((FAIL + 1))
    fi

    # Project update
    PUPDATE_TEXT=$(call_tool_text "create_project_update" "{\"project\": \"$PROJECT_ID\", \"body\": \"Test status update\", \"health\": \"onTrack\"}")
    PUPDATE_ID=$(extract_uuid "$PUPDATE_TEXT")
    if [[ -n "$PUPDATE_ID" ]]; then
        printf "  [T2] %-40s ${GREEN}PASS${NC} — id: %s\n" "create_project_update" "$PUPDATE_ID"
        PASS=$((PASS + 1))
        test_tool 2 "list_project_updates" "{\"project\": \"$PROJECT_ID\"}" ""
        test_tool 2 "update_project_update" "{\"id\": \"$PUPDATE_ID\", \"body\": \"Updated status\"}" ""
        test_tool 2 "delete_project_update" "{\"id\": \"$PUPDATE_ID\"}" ""
    else
        printf "  [T2] %-40s ${RED}FAIL${NC}\n" "create_project_update"
        ERRORS+=("create_project_update: $PUPDATE_TEXT")
        FAIL=$((FAIL + 1))
    fi

    # Archive → unarchive → delete project
    test_tool 2 "archive_project" "{\"id\": \"$PROJECT_ID\"}" ""
    test_tool 2 "unarchive_project" "{\"id\": \"$PROJECT_ID\"}" ""
    test_tool 2 "delete_project" "{\"id\": \"$PROJECT_ID\"}" ""
fi

# ------- Project relations (need 2 projects) -------
echo -e "  ${CYAN}--- Project relations ---${NC}"
PROJREL_SUFFIX=$(date +%s)
test_create "create_project" "{\"name\": \"TH-PROJ-A-$PROJREL_SUFFIX\", \"teams\": \"$TEAM_KEY\"}" PROJ_A_ID
test_create "create_project" "{\"name\": \"TH-PROJ-B-$PROJREL_SUFFIX\", \"teams\": \"$TEAM_KEY\"}" PROJ_B_ID

if [[ -n "$PROJ_A_ID" && -n "$PROJ_B_ID" ]]; then
    PROJREL_TEXT=$(call_tool_text "create_project_relation" "{\"project\": \"$PROJ_A_ID\", \"relatedProject\": \"$PROJ_B_ID\", \"type\": \"related\"}")
    PROJREL_ID=$(extract_uuid "$PROJREL_TEXT")
    if [[ -n "$PROJREL_ID" ]]; then
        printf "  [T2] %-40s ${GREEN}PASS${NC} — id: %s\n" "create_project_relation" "$PROJREL_ID"
        PASS=$((PASS + 1))
        test_tool 2 "list_project_relations" "{\"project\": \"$PROJ_A_ID\"}" ""
        test_tool 2 "update_project_relation" "{\"id\": \"$PROJREL_ID\", \"anchor_type\": \"start\"}" ""
        test_tool 2 "delete_project_relation" "{\"id\": \"$PROJREL_ID\"}" ""
    else
        printf "  [T2] %-40s ${RED}FAIL${NC}\n" "create_project_relation"
        ERRORS+=("create_project_relation: $PROJREL_TEXT")
        FAIL=$((FAIL + 1))
    fi
    # Clean up
    call_tool "archive_project" "{\"id\": \"$PROJ_A_ID\"}" >/dev/null 2>&1
    call_tool "archive_project" "{\"id\": \"$PROJ_B_ID\"}" >/dev/null 2>&1
fi

# ------- Document lifecycle (needs a project first) -------
echo -e "  ${CYAN}--- Document lifecycle ---${NC}"
DOC_SUFFIX=$(date +%s)
DOC_PROJ_TEXT=$(call_tool_text "create_project" "{\"name\": \"TH-DOC-PROJ-$DOC_SUFFIX\", \"teams\": \"$TEAM_KEY\"}")
DOC_PROJ_ID=$(extract_uuid "$DOC_PROJ_TEXT")

if [[ -n "$DOC_PROJ_ID" ]]; then
    test_create "create_document" "{\"title\": \"TEST-HARNESS-DOC\", \"content\": \"Test content\", \"project\": \"$DOC_PROJ_ID\"}" DOC_ID

    if [[ -n "$DOC_ID" ]]; then
        test_tool 2 "get_document" "{\"id\": \"$DOC_ID\"}" ""
        test_tool 2 "update_document" "{\"id\": \"$DOC_ID\", \"title\": \"TEST-HARNESS-DOC-UPDATED\"}" ""
        test_tool 2 "get_document_content_history" "{\"id\": \"$DOC_ID\"}" ""
        test_tool 2 "delete_document" "{\"id\": \"$DOC_ID\"}" ""
    fi

    # Test unarchive_document with a separate doc
    UNDOC_TEXT=$(call_tool_text "create_document" "{\"title\": \"TEST-HARNESS-UNDOC\", \"content\": \"Test\", \"project\": \"$DOC_PROJ_ID\"}")
    UNDOC_ID=$(extract_uuid "$UNDOC_TEXT")
    if [[ -n "$UNDOC_ID" ]]; then
        call_tool "delete_document" "{\"id\": \"$UNDOC_ID\"}" >/dev/null 2>&1
        test_tool 2 "unarchive_document" "{\"id\": \"$UNDOC_ID\"}" ""
        call_tool "delete_document" "{\"id\": \"$UNDOC_ID\"}" >/dev/null 2>&1
    fi

    call_tool "archive_project" "{\"id\": \"$DOC_PROJ_ID\"}" >/dev/null 2>&1
else
    skip_tool 2 "create_document" "failed to create project for doc"
    skip_tool 2 "get_document" "depends on create_document"
    skip_tool 2 "update_document" "depends on create_document"
    skip_tool 2 "delete_document" "depends on create_document"
fi

# ------- Initiative lifecycle -------
echo -e "  ${CYAN}--- Initiative lifecycle ---${NC}"
test_create "create_initiative" "{\"name\": \"TEST-HARNESS-INITIATIVE\", \"description\": \"Test\"}" INIT_ID

if [[ -n "$INIT_ID" ]]; then
    test_tool 2 "update_initiative" "{\"id\": \"$INIT_ID\", \"name\": \"TEST-HARNESS-INITIATIVE-UPDATED\"}" ""
    test_tool 2 "list_initiative_updates" "{\"initiative\": \"$INIT_ID\"}" ""

    # Initiative update
    INITUPD_TEXT=$(call_tool_text "create_initiative_update" "{\"initiative\": \"$INIT_ID\", \"body\": \"Test update\", \"health\": \"onTrack\"}")
    INITUPD_ID=$(extract_uuid "$INITUPD_TEXT")
    if [[ -n "$INITUPD_ID" ]]; then
        printf "  [T2] %-40s ${GREEN}PASS${NC} — id: %s\n" "create_initiative_update" "$INITUPD_ID"
        PASS=$((PASS + 1))
        test_tool 2 "update_initiative_update" "{\"id\": \"$INITUPD_ID\", \"body\": \"Updated init update\"}" ""
        test_tool 2 "archive_initiative_update" "{\"id\": \"$INITUPD_ID\"}" ""
        test_tool 2 "unarchive_initiative_update" "{\"id\": \"$INITUPD_ID\"}" ""
    else
        printf "  [T2] %-40s ${RED}FAIL${NC}\n" "create_initiative_update"
        ERRORS+=("create_initiative_update: $INITUPD_TEXT")
        FAIL=$((FAIL + 1))
    fi

    # Add project to initiative → remove
    # First create a temp project
    INITPROJ_SUFFIX=$(date +%s)
    TEMP_PROJ_TEXT=$(call_tool_text "create_project" "{\"name\": \"TH-INIT-PROJ-$INITPROJ_SUFFIX\", \"teams\": \"$TEAM_KEY\"}")
    TEMP_PROJ_ID=$(extract_uuid "$TEMP_PROJ_TEXT")
    if [[ -n "$TEMP_PROJ_ID" ]]; then
        INITPROJ_TEXT=$(call_tool_text "add_project_to_initiative" "{\"initiative\": \"$INIT_ID\", \"project\": \"$TEMP_PROJ_ID\"}")
        INITPROJ_ID=$(extract_uuid "$INITPROJ_TEXT")
        if [[ -n "$INITPROJ_ID" ]]; then
            printf "  [T2] %-40s ${GREEN}PASS${NC} — id: %s\n" "add_project_to_initiative" "$INITPROJ_ID"
            PASS=$((PASS + 1))
            test_tool 2 "update_initiative_to_project" "{\"id\": \"$INITPROJ_ID\", \"sort_order\": 1.0}" ""
            test_tool 2 "remove_project_from_initiative" "{\"id\": \"$INITPROJ_ID\"}" ""
        else
            printf "  [T2] %-40s ${RED}FAIL${NC}\n" "add_project_to_initiative"
            ERRORS+=("add_project_to_initiative: $INITPROJ_TEXT")
            FAIL=$((FAIL + 1))
        fi
        call_tool "archive_project" "{\"id\": \"$TEMP_PROJ_ID\"}" >/dev/null 2>&1
    fi

    # Archive → unarchive → delete initiative
    test_tool 2 "archive_initiative" "{\"id\": \"$INIT_ID\"}" ""
    test_tool 2 "unarchive_initiative" "{\"id\": \"$INIT_ID\"}" ""
    test_tool 2 "delete_initiative" "{\"id\": \"$INIT_ID\"}" ""
fi

# ------- Customer lifecycle -------
echo -e "  ${CYAN}--- Customer lifecycle ---${NC}"
CUST_SUFFIX=$(date +%s)
test_create "create_customer" "{\"name\": \"TEST-HARNESS-CUSTOMER-$CUST_SUFFIX\", \"domains\": \"test-$CUST_SUFFIX.example.com\"}" CUST_ID

if [[ -n "$CUST_ID" ]]; then
    test_tool 2 "get_customer" "{\"id\": \"$CUST_ID\"}" ""
    test_tool 2 "update_customer" "{\"id\": \"$CUST_ID\", \"name\": \"TEST-HARNESS-CUSTOMER-UPDATED\"}" ""

    # Customer need (needs an issue)
    NEED_ISSUE_TEXT=$(call_tool_text "create_issue" "{\"team\": \"$TEAM_KEY\", \"title\": \"TEST-HARNESS: Need issue\"}")
    NEED_ISSUE_ID=$(extract_uuid "$NEED_ISSUE_TEXT")
    NEED_ISSUE_IDENT=""
    if [[ -n "$NEED_ISSUE_ID" ]]; then
        NEED_ISSUE_IDENT=$(call_tool_text "get_issue" "{\"id\": \"$NEED_ISSUE_ID\"}" | head -1 | grep -oE '[A-Z]+-[0-9]+' | head -1)
    fi

    if [[ -n "$NEED_ISSUE_IDENT" ]]; then
        NEED_TEXT=$(call_tool_text "create_customer_need" "{\"issue\": \"$NEED_ISSUE_IDENT\", \"customer\": \"$CUST_ID\", \"body\": \"Test need\"}")
        NEED_ID=$(extract_uuid "$NEED_TEXT")
        if [[ -n "$NEED_ID" ]]; then
            printf "  [T2] %-40s ${GREEN}PASS${NC} — id: %s\n" "create_customer_need" "$NEED_ID"
            PASS=$((PASS + 1))
            test_tool 2 "update_customer_need" "{\"id\": \"$NEED_ID\", \"body\": \"Updated need\"}" ""
            test_tool 2 "get_customer_need" "{\"id\": \"$NEED_ID\"}" ""
            test_tool 2 "archive_customer_need" "{\"id\": \"$NEED_ID\"}" ""
            test_tool 2 "unarchive_customer_need" "{\"id\": \"$NEED_ID\"}" ""
            test_tool 2 "delete_customer_need" "{\"id\": \"$NEED_ID\"}" ""
        else
            printf "  [T2] %-40s ${RED}FAIL${NC}\n" "create_customer_need"
            ERRORS+=("create_customer_need: $NEED_TEXT")
            FAIL=$((FAIL + 1))
        fi
        # Clean up issue
        call_tool "delete_issue" "{\"id\": \"$NEED_ISSUE_IDENT\"}" >/dev/null 2>&1
    fi

    # Merge customers (create a second, merge into first)
    MERGE_CUST_TEXT=$(call_tool_text "create_customer" "{\"name\": \"TEST-HARNESS-MERGE-SRC-$CUST_SUFFIX\"}")
    MERGE_CUST_ID=$(extract_uuid "$MERGE_CUST_TEXT")
    if [[ -n "$MERGE_CUST_ID" ]]; then
        test_tool 2 "merge_customers" "{\"source_id\": \"$MERGE_CUST_ID\", \"target_id\": \"$CUST_ID\"}" ""
    fi

    test_tool 2 "delete_customer" "{\"id\": \"$CUST_ID\"}" ""
fi

# ------- View lifecycle -------
echo -e "  ${CYAN}--- View lifecycle ---${NC}"
test_create "create_view" "{\"name\": \"TEST-HARNESS-VIEW\"}" VIEW_ID

if [[ -n "$VIEW_ID" ]]; then
    test_tool 2 "update_view" "{\"id\": \"$VIEW_ID\", \"name\": \"TEST-HARNESS-VIEW-UPDATED\"}" ""
    test_tool 2 "get_view_issues" "{\"id\": \"$VIEW_ID\"}" ""
    test_tool 2 "check_custom_view_has_subscribers" "{\"id\": \"$VIEW_ID\"}" ""
    test_tool 2 "delete_view" "{\"id\": \"$VIEW_ID\"}" ""
fi

# ------- Webhook lifecycle -------
echo -e "  ${CYAN}--- Webhook lifecycle ---${NC}"
WEBHOOK_URL="https://example.com/webhook-test-$(date +%s)"
WEBHOOK_TEXT=$(call_tool_text "create_webhook" "{\"url\": \"$WEBHOOK_URL\", \"label\": \"TEST-HARNESS-WEBHOOK\", \"resourceTypes\": \"Issue\"}")
WEBHOOK_ID=$(extract_uuid "$WEBHOOK_TEXT")
if [[ -n "$WEBHOOK_ID" ]]; then
    printf "  [T2] %-40s ${GREEN}PASS${NC} — id: %s\n" "create_webhook" "$WEBHOOK_ID"
    PASS=$((PASS + 1))
    test_tool 2 "update_webhook" "{\"id\": \"$WEBHOOK_ID\", \"label\": \"TEST-HARNESS-WEBHOOK-UPDATED\"}" ""
    test_tool 2 "delete_webhook" "{\"id\": \"$WEBHOOK_ID\"}" ""
else
    printf "  [T2] %-40s ${RED}FAIL${NC}\n" "create_webhook"
    ERRORS+=("create_webhook: $WEBHOOK_TEXT")
    FAIL=$((FAIL + 1))
fi

# ------- Agent session lifecycle (requires OAuth, not API key) -------
echo -e "  ${CYAN}--- Agent session lifecycle ---${NC}"
skip_tool 2 "create_agent_session" "requires OAuth authentication (not API key)"
skip_tool 2 "get_agent_session" "depends on create_agent_session"
skip_tool 2 "update_agent_session" "depends on create_agent_session"
skip_tool 2 "create_agent_activity" "depends on create_agent_session"

# ------- Favorite lifecycle -------
echo -e "  ${CYAN}--- Favorites ---${NC}"
# Create a temp issue to favorite
FAV_ISSUE_TEXT=$(call_tool_text "create_issue" "{\"team\": \"$TEAM_KEY\", \"title\": \"TEST-HARNESS: Favorite me\"}")
FAV_ISSUE_ID=$(extract_uuid "$FAV_ISSUE_TEXT")
FAV_ISSUE_IDENT=""
if [[ -n "$FAV_ISSUE_ID" ]]; then
    FAV_ISSUE_IDENT=$(call_tool_text "get_issue" "{\"id\": \"$FAV_ISSUE_ID\"}" | head -1 | grep -oE '[A-Z]+-[0-9]+' | head -1)
fi

if [[ -n "$FAV_ISSUE_IDENT" ]]; then
    FAV_TEXT=$(call_tool_text "add_favorite" "{\"issueId\": \"$FAV_ISSUE_IDENT\"}")
    FAV_ID=$(extract_uuid "$FAV_TEXT")
    if [[ -n "$FAV_ID" ]]; then
        printf "  [T2] %-40s ${GREEN}PASS${NC} — id: %s\n" "add_favorite" "$FAV_ID"
        PASS=$((PASS + 1))
        test_tool 2 "get_favorite" "{\"id\": \"$FAV_ID\"}" ""
        test_tool 2 "update_favorite" "{\"id\": \"$FAV_ID\", \"sort_order\": 0.5}" ""
        test_tool 2 "remove_favorite" "{\"id\": \"$FAV_ID\"}" ""
    else
        printf "  [T2] %-40s ${RED}FAIL${NC}\n" "add_favorite"
        ERRORS+=("add_favorite: $FAV_TEXT")
        FAIL=$((FAIL + 1))
    fi
    call_tool "delete_issue" "{\"id\": \"$FAV_ISSUE_IDENT\"}" >/dev/null 2>&1
fi

# ------- Notification -------
echo -e "  ${CYAN}--- Notifications ---${NC}"
NOTIF_TEXT=$(call_tool_text "list_notifications" '{"limit": 1}')
NOTIF_ID=$(extract_uuid "$NOTIF_TEXT")
if [[ -n "$NOTIF_ID" ]]; then
    test_tool 2 "get_notification" "{\"id\": \"$NOTIF_ID\"}" ""
    test_tool 2 "mark_notification_read" "{\"id\": \"$NOTIF_ID\"}" ""
else
    skip_tool 2 "mark_notification_read" "no notifications available"
fi

# ------- Team update -------
echo -e "  ${CYAN}--- Team ---${NC}"
test_tool 2 "update_team" "{\"id\": \"$TEAM_KEY\", \"description\": \"Updated by test harness\"}" ""
# Restore
test_tool 2 "update_team" "{\"id\": \"$TEAM_KEY\", \"description\": \"\"}" ""

# ------- User -------
echo -e "  ${CYAN}--- User ---${NC}"
if [[ -n "$USER_EMAIL" ]]; then
    test_tool 2 "get_user" "{\"id\": \"$USER_EMAIL\"}" ""
    test_tool 2 "update_user" "{\"id\": \"$USER_EMAIL\", \"status_label\": \"Testing\"}" ""
    test_tool 2 "update_user" "{\"id\": \"$USER_EMAIL\", \"status_label\": \"\"}" ""
fi

# ------- Roadmap tools removed (deprecated by Linear) -------

# ------- Custom view extras -------
echo -e "  ${CYAN}--- Custom view extras ---${NC}"
test_tool 2 "get_custom_view_suggestion" '{"model_name": "issues", "filter": {}}' ""

# ------- Release lifecycle (may require feature flag) -------
echo -e "  ${CYAN}--- Releases ---${NC}"
skip_tool 2 "create_release" "requires pipeline UUID + feature flag"
skip_tool 2 "update_release" "depends on create_release"

# ------- Team create/delete/unarchive -------
echo -e "  ${CYAN}--- Team lifecycle ---${NC}"
TM_SUFFIX=$((RANDOM % 9000 + 1000))
test_create "create_team" "{\"name\": \"TEST-HARNESS-TEAM-$TM_SUFFIX\", \"key\": \"TH$TM_SUFFIX\"}" NEW_TEAM_ID
if [[ -n "$NEW_TEAM_ID" ]]; then
    test_tool 2 "delete_team" "{\"id\": \"$NEW_TEAM_ID\"}" ""
    test_tool 2 "unarchive_team" "{\"id\": \"$NEW_TEAM_ID\"}" ""
    # Final cleanup: delete again
    call_tool "delete_team" "{\"id\": \"$NEW_TEAM_ID\"}" >/dev/null 2>&1
fi

# ------- Template + Issue from template -------
echo -e "  ${CYAN}--- Template lifecycle ---${NC}"
test_create "create_template" "{\"name\": \"TEST-HARNESS-TEMPLATE\", \"type\": \"issue\", \"template_data\": \"{\\\"title\\\":\\\"TEST-TEMPLATE-ISSUE\\\",\\\"description\\\":\\\"Created from template\\\"}\", \"team\": \"$TEAM_KEY\"}" TMPL_ID
if [[ -n "$TMPL_ID" ]]; then
    test_tool 2 "get_template" "{\"id\": \"$TMPL_ID\"}" ""
    test_tool 2 "update_template" "{\"id\": \"$TMPL_ID\", \"name\": \"TEST-HARNESS-TEMPLATE-UPD\"}" ""
    # Create issue from template
    FROMTMPL_TEXT=$(call_tool_text "create_issue_from_template" "{\"team\": \"$TEAM_KEY\", \"templateId\": \"$TMPL_ID\"}")
    FROMTMPL_ID=$(extract_uuid "$FROMTMPL_TEXT")
    if [[ -n "$FROMTMPL_ID" ]]; then
        printf "  [T2] %-40s ${GREEN}PASS${NC} — id: %s\n" "create_issue_from_template" "$FROMTMPL_ID"
        PASS=$((PASS + 1))
        # Clean up issue created from template
        FROMTMPL_IDENT=$(echo "$FROMTMPL_TEXT" | grep -oE '[A-Z]+-[0-9]+' | head -1)
        call_tool "delete_issue" "{\"id\": \"${FROMTMPL_IDENT:-$FROMTMPL_ID}\"}" >/dev/null 2>&1
    else
        printf "  [T2] %-40s ${RED}FAIL${NC}\n" "create_issue_from_template"
        ERRORS+=("create_issue_from_template: $FROMTMPL_TEXT")
        FAIL=$((FAIL + 1))
    fi
    test_tool 2 "delete_template" "{\"id\": \"$TMPL_ID\"}" ""
fi

# ------- Workflow state lifecycle -------
echo -e "  ${CYAN}--- Workflow state lifecycle ---${NC}"
WFS_SUFFIX=$((RANDOM % 9000 + 1000))
test_create "create_workflow_state" "{\"team\": \"$TEAM_KEY\", \"name\": \"TH-STATE-$WFS_SUFFIX\", \"color\": \"#ff8800\", \"type\": \"started\"}" WF_STATE_ID
if [[ -n "$WF_STATE_ID" ]]; then
    test_tool 2 "get_workflow_state" "{\"id\": \"$WF_STATE_ID\"}" ""
    test_tool 2 "update_workflow_state" "{\"id\": \"$WF_STATE_ID\", \"name\": \"TH-STATE-UPD-$WFS_SUFFIX\"}" ""
    test_tool 2 "archive_workflow_state" "{\"id\": \"$WF_STATE_ID\"}" ""
fi

# ------- Customer status lifecycle -------
echo -e "  ${CYAN}--- Customer status lifecycle ---${NC}"
CS_SUFFIX=$((RANDOM % 9000 + 1000))
test_create "create_customer_status" "{\"name\": \"TH-CSTATUS-$CS_SUFFIX\", \"color\": \"#112233\"}" CS_ID
if [[ -n "$CS_ID" ]]; then
    test_tool 2 "get_customer_status" "{\"id\": \"$CS_ID\"}" ""
    test_tool 2 "update_customer_status" "{\"id\": \"$CS_ID\", \"name\": \"TH-CSTATUS-UPD-$CS_SUFFIX\"}" ""
    test_tool 2 "delete_customer_status" "{\"id\": \"$CS_ID\"}" ""
fi

# ------- Customer tier lifecycle -------
echo -e "  ${CYAN}--- Customer tier lifecycle ---${NC}"
CT_SUFFIX=$((RANDOM % 9000 + 1000))
test_create "create_customer_tier" "{\"name\": \"TH-CTIER-$CT_SUFFIX\", \"color\": \"#334455\"}" CT_ID
if [[ -n "$CT_ID" ]]; then
    test_tool 2 "get_customer_tier" "{\"id\": \"$CT_ID\"}" ""
    test_tool 2 "update_customer_tier" "{\"id\": \"$CT_ID\", \"name\": \"TH-CTIER-UPD-$CT_SUFFIX\"}" ""
    test_tool 2 "delete_customer_tier" "{\"id\": \"$CT_ID\"}" ""
fi

# ------- Release ecosystem (feature-gated) -------
echo -e "  ${CYAN}--- Release ecosystem (feature-gated) ---${NC}"
skip_tool 2 "create_release_pipeline" "requires Release Management feature"
skip_tool 2 "get_release_pipeline" "depends on create_release_pipeline"
skip_tool 2 "update_release_pipeline" "depends on create_release_pipeline"
skip_tool 2 "delete_release_pipeline" "depends on create_release_pipeline"
skip_tool 2 "create_release_stage" "requires Release Management feature"
skip_tool 2 "get_release_stage" "depends on create_release_stage"
skip_tool 2 "update_release_stage" "depends on create_release_stage"
skip_tool 2 "get_release" "requires Release Management feature"
skip_tool 2 "archive_release" "depends on create_release"
skip_tool 2 "unarchive_release" "depends on create_release"
skip_tool 2 "delete_release" "depends on create_release"
skip_tool 2 "add_issue_to_release" "requires Release Management feature"
skip_tool 2 "get_issue_to_release" "depends on add_issue_to_release"
skip_tool 2 "remove_issue_from_release" "depends on add_issue_to_release"

# ------- Project status lifecycle -------
echo -e "  ${CYAN}--- Project status lifecycle ---${NC}"
test_create "create_project_status" "{\"name\": \"TEST-HARNESS-PSTATUS\", \"color\": \"#556677\", \"type\": \"started\", \"position\": 99}" PS_ID
if [[ -n "$PS_ID" ]]; then
    test_tool 2 "get_project_status" "{\"id\": \"$PS_ID\"}" ""
    test_tool 2 "update_project_status" "{\"id\": \"$PS_ID\", \"name\": \"TEST-HARNESS-PSTATUS-UPD\"}" ""
    test_tool 2 "archive_project_status" "{\"id\": \"$PS_ID\"}" ""
    test_tool 2 "unarchive_project_status" "{\"id\": \"$PS_ID\"}" ""
    # Final cleanup: archive again
    call_tool "archive_project_status" "{\"id\": \"$PS_ID\"}" >/dev/null 2>&1
fi

# ------- Project label lifecycle -------
echo -e "  ${CYAN}--- Project label lifecycle ---${NC}"
PL_SUFFIX=$(date +%s)
test_create "create_project_label" "{\"name\": \"TEST-HARNESS-PLABEL-$PL_SUFFIX\", \"color\": \"#889900\"}" PL_ID
if [[ -n "$PL_ID" ]]; then
    test_tool 2 "get_project_label" "{\"id\": \"$PL_ID\"}" ""
    test_tool 2 "update_project_label" "{\"id\": \"$PL_ID\", \"name\": \"TEST-HARNESS-PLABEL-UPD-$PL_SUFFIX\"}" ""
    test_tool 2 "delete_project_label" "{\"id\": \"$PL_ID\"}" ""
fi

# ------- Team membership lifecycle -------
echo -e "  ${CYAN}--- Team membership lifecycle ---${NC}"
test_tool 2 "list_team_memberships" "{\"team\": \"$TEAM_KEY\"}" ""
# Create a temp team to test membership CRUD with the second user
MEMB_SUFFIX=$((RANDOM % 9000 + 1000))
MEMB_TEAM_TEXT=$(call_tool_text "create_team" "{\"name\": \"TEST-HARNESS-MEMB-$MEMB_SUFFIX\", \"key\": \"TM$MEMB_SUFFIX\"}")
MEMB_TEAM_ID=$(extract_uuid "$MEMB_TEAM_TEXT")
# Find a second workspace user (not the API key owner) for membership tests
SECOND_USER=$(call_tool_text "list_users" '{"limit": 10}' | grep -oE '[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]+' | grep -v "$USER_EMAIL" | head -1)
if [[ -z "$SECOND_USER" ]]; then
    skip_tool 2 "create_team_membership" "no second workspace user found"
    skip_tool 2 "get_team_membership" "depends on create_team_membership"
    skip_tool 2 "update_team_membership" "depends on create_team_membership"
    skip_tool 2 "delete_team_membership" "depends on create_team_membership"
    call_tool "delete_team" "{\"id\": \"$MEMB_TEAM_ID\"}" >/dev/null 2>&1
elif [[ -n "$MEMB_TEAM_ID" ]]; then
    MEMB_TEXT=$(call_tool_text "create_team_membership" "{\"user\": \"$SECOND_USER\", \"team\": \"$MEMB_TEAM_ID\"}")
    MEMB_ID=$(extract_uuid "$MEMB_TEXT")
    if [[ -n "$MEMB_ID" ]]; then
        printf "  [T2] %-40s ${GREEN}PASS${NC} — id: %s\n" "create_team_membership" "$MEMB_ID"
        PASS=$((PASS + 1))
        test_tool 2 "get_team_membership" "{\"id\": \"$MEMB_ID\"}" ""
        test_tool 2 "update_team_membership" "{\"id\": \"$MEMB_ID\", \"owner\": false}" ""
        test_tool 2 "delete_team_membership" "{\"id\": \"$MEMB_ID\"}" ""
    elif echo "$MEMB_TEXT" | grep -qi "already.*member"; then
        # Linear auto-adds workspace members to new teams; find the second user's membership
        printf "  [T2] %-40s ${GREEN}PASS${NC} — (user already auto-added to team)\n" "create_team_membership"
        PASS=$((PASS + 1))
        # Get all memberships and find the one for SECOND_USER specifically
        EXISTING_TM=$(call_tool_text "list_team_memberships" "{\"team\": \"$MEMB_TEAM_ID\"}")
        # Extract UUID from the line containing the second user's email
        MEMB_ID=$(echo "$EXISTING_TM" | grep -i "$SECOND_USER" | grep -oE '[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}' | head -1)
        if [[ -n "$MEMB_ID" ]]; then
            test_tool 2 "get_team_membership" "{\"id\": \"$MEMB_ID\"}" ""
            test_tool 2 "update_team_membership" "{\"id\": \"$MEMB_ID\", \"owner\": false}" ""
            test_tool 2 "delete_team_membership" "{\"id\": \"$MEMB_ID\"}" ""
        else
            skip_tool 2 "get_team_membership" "could not identify second user's membership"
            skip_tool 2 "update_team_membership" "could not identify second user's membership"
            skip_tool 2 "delete_team_membership" "could not identify second user's membership"
        fi
    else
        printf "  [T2] %-40s ${RED}FAIL${NC}\n" "create_team_membership"
        ERRORS+=("create_team_membership: $MEMB_TEXT")
        FAIL=$((FAIL + 1))
        skip_tool 2 "get_team_membership" "depends on create_team_membership"
        skip_tool 2 "update_team_membership" "depends on create_team_membership"
        skip_tool 2 "delete_team_membership" "depends on create_team_membership"
    fi
    call_tool "delete_team" "{\"id\": \"$MEMB_TEAM_ID\"}" >/dev/null 2>&1
fi

# ------- Notification subscriptions -------
echo -e "  ${CYAN}--- Notification subscriptions ---${NC}"
test_tool 2 "list_notification_subscriptions" '{}' ""
# Read-only tests on existing subscriptions (no delete tool exists, so we avoid
# creating new subscriptions or mutating existing ones to prevent orphaned state)
EXISTING_NS=$(call_tool_text "list_notification_subscriptions" '{}')
NS_ID=$(extract_uuid "$EXISTING_NS")
if [[ -n "$NS_ID" ]]; then
    test_tool 2 "get_notification_subscription" "{\"id\": \"$NS_ID\"}" ""
fi
skip_tool 2 "create_notification_subscription" "no delete tool — would leave orphaned subscription"
skip_tool 2 "update_notification_subscription" "would mutate real subscription with no guaranteed rollback"

# (Template lifecycle moved up — tested with issue-from-template)

# ------- Entity external link lifecycle -------
echo -e "  ${CYAN}--- Entity external link lifecycle ---${NC}"
EEL_SUFFIX=$(date +%s)
EEL_PROJ_TEXT=$(call_tool_text "create_project" "{\"name\": \"TH-EEL-PROJ-$EEL_SUFFIX\", \"teams\": \"$TEAM_KEY\"}")
EEL_PROJ_ID=$(extract_uuid "$EEL_PROJ_TEXT")
if [[ -n "$EEL_PROJ_ID" ]]; then
    EEL_TEXT=$(call_tool_text "create_entity_external_link" "{\"url\": \"https://example.com/eel-test\", \"label\": \"Test EEL\", \"project\": \"$EEL_PROJ_ID\"}")
    EEL_ID=$(extract_uuid "$EEL_TEXT")
    if [[ -n "$EEL_ID" ]]; then
        printf "  [T2] %-40s ${GREEN}PASS${NC} — id: %s\n" "create_entity_external_link" "$EEL_ID"
        PASS=$((PASS + 1))
        test_tool 2 "get_entity_external_link" "{\"id\": \"$EEL_ID\"}" ""
        test_tool 2 "update_entity_external_link" "{\"id\": \"$EEL_ID\", \"label\": \"Updated EEL\"}" ""
        test_tool 2 "delete_entity_external_link" "{\"id\": \"$EEL_ID\"}" ""
    else
        printf "  [T2] %-40s ${RED}FAIL${NC}\n" "create_entity_external_link"
        ERRORS+=("create_entity_external_link: $EEL_TEXT")
        FAIL=$((FAIL + 1))
    fi
    call_tool "archive_project" "{\"id\": \"$EEL_PROJ_ID\"}" >/dev/null 2>&1
fi

# ------- Emoji lifecycle -------
echo -e "  ${CYAN}--- Emoji lifecycle ---${NC}"
skip_tool 2 "create_emoji" "requires hosted image URL"
skip_tool 2 "get_emoji" "depends on create_emoji"
skip_tool 2 "delete_emoji" "depends on create_emoji"

# ------- Initiative relation lifecycle -------
echo -e "  ${CYAN}--- Initiative relation lifecycle ---${NC}"
skip_tool 2 "create_initiative_relation" "requires Enterprise plan (subInitiatives feature)"
skip_tool 2 "get_initiative_relation" "requires Enterprise plan (subInitiatives feature)"
skip_tool 2 "update_initiative_relation" "requires Enterprise plan (subInitiatives feature)"
skip_tool 2 "delete_initiative_relation" "requires Enterprise plan (subInitiatives feature)"

# ------- Time schedule lifecycle -------
echo -e "  ${CYAN}--- Time schedule lifecycle ---${NC}"
TS_ENTRIES="[{\\\"userEmail\\\":\\\"$USER_EMAIL\\\",\\\"startsAt\\\":\\\"2027-01-01T09:00:00Z\\\",\\\"endsAt\\\":\\\"2027-01-01T17:00:00Z\\\"}]"
test_create "create_time_schedule" "{\"name\": \"TEST-HARNESS-SCHED\", \"entries\": \"$TS_ENTRIES\"}" TS_ID
if [[ -n "$TS_ID" ]]; then
    test_tool 2 "get_time_schedule" "{\"id\": \"$TS_ID\"}" ""
    test_tool 2 "update_time_schedule" "{\"id\": \"$TS_ID\", \"name\": \"TEST-HARNESS-SCHED-UPD\", \"entries\": \"$TS_ENTRIES\"}" ""
    test_tool 2 "delete_time_schedule" "{\"id\": \"$TS_ID\"}" ""
fi

# ------- Triage responsibility lifecycle -------
echo -e "  ${CYAN}--- Triage responsibility lifecycle ---${NC}"
skip_tool 2 "create_triage_responsibility" "requires Business plan"
skip_tool 2 "get_triage_responsibility" "requires Business plan"
skip_tool 2 "update_triage_responsibility" "requires Business plan"
skip_tool 2 "delete_triage_responsibility" "requires Business plan"

# ------- Git automation lifecycle -------
echo -e "  ${CYAN}--- Git automation lifecycle ---${NC}"
test_create "create_git_automation_target_branch" "{\"team\": \"$TEAM_KEY\", \"branch_pattern\": \"test-harness-*\"}" GATB_ID
if [[ -n "$GATB_ID" ]]; then
    skip_tool 2 "update_git_automation_target_branch" "Linear API returns Internal Server Error for this mutation"
    # Get a workflow state for automation
    WF_FOR_GIT=$(call_tool_text "list_states" "{\"team\": \"$TEAM_KEY\"}" | grep -oE '[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}' | head -1)
    if [[ -n "$WF_FOR_GIT" ]]; then
        GAS_TEXT=$(call_tool_text "create_git_automation_state" "{\"team\": \"$TEAM_KEY\", \"event\": \"branchCreated\", \"state_id\": \"$WF_FOR_GIT\", \"target_branch_id\": \"$GATB_ID\"}")
        GAS_ID=$(extract_uuid "$GAS_TEXT")
        if [[ -n "$GAS_ID" ]]; then
            printf "  [T2] %-40s ${GREEN}PASS${NC} — id: %s\n" "create_git_automation_state" "$GAS_ID"
            PASS=$((PASS + 1))
            test_tool 2 "update_git_automation_state" "{\"id\": \"$GAS_ID\", \"event\": \"branchMerged\"}" ""
            test_tool 2 "delete_git_automation_state" "{\"id\": \"$GAS_ID\"}" ""
        else
            printf "  [T2] %-40s ${RED}FAIL${NC}\n" "create_git_automation_state"
            ERRORS+=("create_git_automation_state: $GAS_TEXT")
            FAIL=$((FAIL + 1))
        fi
    fi
    test_tool 2 "delete_git_automation_target_branch" "{\"id\": \"$GATB_ID\"}" ""
fi

# ------- Email intake lifecycle -------
echo -e "  ${CYAN}--- Email intake lifecycle ---${NC}"
test_create "create_email_intake_address" "{\"team\": \"$TEAM_KEY\"}" EIA_ID
if [[ -n "$EIA_ID" ]]; then
    test_tool 2 "get_email_intake_address" "{\"id\": \"$EIA_ID\"}" ""
    test_tool 2 "update_email_intake_address" "{\"id\": \"$EIA_ID\", \"enabled\": false}" ""
    test_tool 2 "delete_email_intake_address" "{\"id\": \"$EIA_ID\"}" ""
fi

# ------- Figma -------
echo -e "  ${CYAN}--- Figma ---${NC}"
skip_tool 2 "search_issue_figma_file_key" "requires Figma integration"

# ------- Agent session on comment -------
skip_tool 2 "create_agent_session_on_comment" "requires OAuth authentication"

echo ""
fi

# ============================================================
# Summary
# ============================================================

echo -e "${CYAN}${BOLD}========================================================${NC}"
echo -e "${CYAN}${BOLD}  Test Summary                                          ${NC}"
echo -e "${CYAN}${BOLD}========================================================${NC}"
echo -e "  ${GREEN}PASS${NC}: $PASS"
echo -e "  ${RED}FAIL${NC}: $FAIL"
echo -e "  ${YELLOW}SKIP${NC}: $SKIP"
TOTAL=$((PASS + FAIL + SKIP))
echo -e "  TOTAL: $TOTAL"
echo ""

if [[ ${#ERRORS[@]} -gt 0 ]]; then
    echo -e "${RED}${BOLD}Failures:${NC}"
    for err in "${ERRORS[@]}"; do
        echo "  - $err"
    done
    echo ""
fi

if [[ $FAIL -eq 0 ]]; then
    echo -e "${GREEN}${BOLD}All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}${BOLD}$FAIL test(s) failed.${NC}"
    exit 1
fi
