#!/usr/bin/env bash
set -euo pipefail
if [[ -n "${DEBUG_MODE:-}" ]];then
    set -x
fi
IFS=' '
WORKSPACE=$(unset CDPATH && cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
UT_PATH="${WORKSPACE}"/tests
OUTPUT_PATH="${WORKSPACE}"/output
# import utils
source "${WORKSPACE}/ci/log.sh"

function run_unit_test() {
    log::Info "running unit test"
    cd "${WORKSPACE}"
    cargo test
    exit $?
}

# 方便另一个脚本对job_1进行单元测试(https://github.com/torokmark/assert.sh)
function main() {
    run_unit_test
}
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main
fi