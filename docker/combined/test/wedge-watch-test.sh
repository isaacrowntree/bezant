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
# The two properties that matter most:
#   * authenticated => NEVER restart. A bounce costs an IB Key tap, which is
#     the most expensive thing in this system.
#   * 401 => healthy. It is the correct answer before anyone has logged in;
#     counting it as a fault would make every logged-out gateway look wedged
#     and loop forever.
set -uo pipefail
PASS=0; FAIL=0
check(){ if [ "$2" = "$3" ]; then PASS=$((PASS+1)); echo "  ok   $1"; else FAIL=$((FAIL+1)); echo "  FAIL $1"; echo "     expected $3, got $2"; fi; }

THRESHOLD="${SSO_WEDGE_THRESHOLD:-5}"

decide(){ # auth_code sso_code faults_in -> "faults_out|action"
  local auth_code="$1" sso_code="$2" faults="$3"
  case "$auth_code" in
    2*) echo "0|skip-authenticated"; return ;;
    000) echo "$faults|skip-unreachable"; return ;;
  esac
  case "$sso_code" in
    5*) faults=$((faults+1)) ;;
    000) ;;
    *) faults=0 ;;
  esac
  if [ "$faults" -ge "$THRESHOLD" ]; then echo "$faults|RESTART"; else echo "$faults|wait"; fi
}

echo "sso_wedge_watch decision table"
check "authenticated => never restarts, and clears the count" "$(decide 200 500 4)" "0|skip-authenticated"
check "  ...even sitting on the threshold"                    "$(decide 200 500 9)" "0|skip-authenticated"
check "gateway unreachable => no verdict, count preserved"    "$(decide 000 000 3)" "3|skip-unreachable"
check "logged out + 5xx => counts up"                         "$(decide 401 500 0)" "1|wait"
check "  ...and restarts at the threshold"                    "$(decide 401 500 4)" "5|RESTART"
check "  ...but not one short of it"                          "$(decide 401 500 3)" "4|wait"
check "a 401 bridge is HEALTHY and resets"                    "$(decide 401 401 4)" "0|wait"
check "so does a 200"                                         "$(decide 401 200 4)" "0|wait"
check "a transport blip neither counts nor resets"            "$(decide 401 000 3)" "3|wait"

printf '\n%d passed, %d failed\n' "$PASS" "$FAIL"
[ "$FAIL" -eq 0 ]
