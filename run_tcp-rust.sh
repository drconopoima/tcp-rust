#!/bin/bash
set -eEuo pipefail
script_file=$(find "${0}" -type f)
readonly script_file
script_name=$(basename --suffix ".sh" -- "${script_file}")
readonly script_name
script_version='0.1.0'
readonly script_version

printf "%s (%s)\n" "${script_name}" "${script_version}"
lockfile="/var/lock/${script_name}.lock"
readonly lockfile

function handler_cleanup() {
    if grep -q -- $$ "${lockfile}"; then
        rm -v -- "${lockfile}" && echo "Removed lock ${lockfile}"
    else
        printf "Lock %s is currently held by process %s. Not removing.\n" "${lockfile}" "$(cat "${lockfile}")"
    fi
}
readonly -f handler_cleanup

trap handler_cleanup EXIT

readonly CARGO_TARGET_DIR=${1:-"./target"}
readonly IP_RANGE_TUNTAP=${2:-"192.168.0.1/24"}
readonly TUNTAP_DEVICE_NAME=${3:-"tun0"}
readonly binary="tcp-rust"

if [[ ! -f "${lockfile}" ]]; then
    (echo $$ >"${lockfile}") && echo "Adquired lock ${lockfile}"
    set -x
    cargo build --release
    { set +x; } 2>/dev/null
    sudo -p "Please enter your password: " whoami 1>/dev/null && {
        set -x
        sudo setcap cap_net_admin=eip "$CARGO_TARGET_DIR/release/${binary}"
        { set +x; } 2>/dev/null
    }
    command=("$CARGO_TARGET_DIR/release/${binary}")
    set -x
    ${command[0]} &
    PROCESS_PID="$!"
    { set +x; } 2>/dev/null
    readonly PROCESS_PID
    sudo -p "Please enter your password: " whoami 1>/dev/null && {
        set -x
        sudo ip addr add "${IP_RANGE_TUNTAP}" dev "${TUNTAP_DEVICE_NAME}"
        sudo ip link set up dev "${TUNTAP_DEVICE_NAME}"
        { set +x; } 2>/dev/null
    }
    function handler_stop() {
        set +eEuo pipefail
        kill -- "$PROCESS_PID"
        (ps -o comm -p "${PROCESS_PID}" | head -n+2 | grep -q -- "${binary}") && kill -9 -- "${PROCESS_PID}"
    }
    readonly -f handler_stop
    trap handler_stop SIGHUP SIGINT SIGQUIT SIGTERM # 1 2 3 15

    wait -- "${PROCESS_PID}"
fi
