#!/usr/bin/env bash
# Decision table for entrypoint.sh's sso_wedge_watch.
#
#     docker/combined/test/wedge-watch-test.sh
#
# This loop can KILL THE GATEWAY, so the cost of getting it wrong is either a
# container that never heals or one that restart-loops a live trading session.
# Neither is visible in a unit test of the Rust, and spinning a real container
# to find out is slow — so the branch logic is mirrored here and asserted
# directly. Keep `decide()` in step with the case statement in entrypoint.sh.
#
# The properties that matter most:
#   * authenticated => NEVER restart. A bounce costs an IB Key tap, which is
#     the most expensive thing in this system.
#   * 401 => healthy. It is the correct answer before anyone has logged in;
#     counting it as a fault would make every logged-out gateway look wedged
#     and loop forever.
#   * only a definite logged-out auth/status (401/403) is judged at all.
#   * the host's session lock => hands off entirely.
set -uo pipefail
PASS=0; FAIL=0
check(){ if [ "$2" = "$3" ]; then PASS=$((PASS+1)); echo "  ok   $1"; else FAIL=$((FAIL+1)); echo "  FAIL $1"; echo "     expected $3, got $2"; fi; }

THRESHOLD="${SSO_WEDGE_THRESHOLD:-5}"

decide(){ # lock auth_code sso_code faults_in -> "faults_out|action"
  local lock="$1" auth_code="$2" sso_code="$3" faults="$4"
  [ "$lock" = held ] && { echo "$faults|skip-locked"; return; }
  case "$auth_code" in
    2*) echo "0|skip-authenticated"; return ;;
    401|403) ;;
    *) echo "$faults|skip-no-verdict"; return ;;
  esac
  case "$sso_code" in
    5*) faults=$((faults+1)) ;;
    000) ;;
    *) faults=0 ;;
  esac
  if [ "$faults" -ge "$THRESHOLD" ]; then echo "$faults|RESTART"; else echo "$faults|wait"; fi
}

echo "sso_wedge_watch decision table"
check "authenticated => never restarts, and clears the count" "$(decide free 200 500 4)" "0|skip-authenticated"
check "  ...even sitting on the threshold"                    "$(decide free 200 500 9)" "0|skip-authenticated"
check "gateway unreachable => no verdict, count preserved"    "$(decide free 000 000 3)" "3|skip-no-verdict"
check "auth/status 5xx => no verdict, count preserved"        "$(decide free 500 500 4)" "4|skip-no-verdict"
check "auth/status redirect => no verdict either"             "$(decide free 302 500 4)" "4|skip-no-verdict"
check "logged out + 5xx => counts up"                         "$(decide free 401 500 0)" "1|wait"
check "  ...403 is logged out too"                            "$(decide free 403 500 0)" "1|wait"
check "  ...and restarts at the threshold"                    "$(decide free 401 500 4)" "5|RESTART"
check "  ...but not one short of it"                          "$(decide free 401 500 3)" "4|wait"
check "a 401 bridge is HEALTHY and resets"                    "$(decide free 401 401 4)" "0|wait"
check "so does a 200"                                         "$(decide free 401 200 4)" "0|wait"
check "a transport blip neither counts nor resets"            "$(decide free 401 000 3)" "3|wait"
check "session lock held => hands off, count preserved"       "$(decide held 401 500 4)" "4|skip-locked"
check "  ...even when authenticated"                          "$(decide held 200 500 4)" "4|skip-locked"

# session_locked() itself, lifted verbatim out of entrypoint.sh so the parse of
# the host's holder.json is tested against the real function, not a copy.
eval "$(sed -n '/^session_locked() {/,/^}/p' "$(dirname "$0")/../entrypoint.sh")"
SESSION_LOCK_DIR="$(mktemp -d)"
trap 'rm -rf "$SESSION_LOCK_DIR"' EXIT
lockstate(){ if session_locked; then echo held; else echo free; fi; }
check "no lock directory contents => free"                    "$(lockstate)" "free"
printf '{"owner":"relogin","pid":1,"host":"h","token":"t","acquiredAt":"x","expiresAt":"y","expiresAtEpoch":%s}\n' \
  "$(( $(date +%s) + 300 ))" > "$SESSION_LOCK_DIR/holder.json"
check "a live lease => held"                                  "$(lockstate)" "held"
printf '{"owner":"relogin","expiresAtEpoch":%s,"token":"t"}' "$(( $(date +%s) - 1 ))" > "$SESSION_LOCK_DIR/holder.json"
check "an expired lease => free"                              "$(lockstate)" "free"
printf 'garbage' > "$SESSION_LOCK_DIR/holder.json"
check "an unreadable holder => free"                          "$(lockstate)" "free"

printf '\n%d passed, %d failed\n' "$PASS" "$FAIL"
[ "$FAIL" -eq 0 ]
