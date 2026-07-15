#!/usr/bin/env bash

set -Eeuo pipefail

########################################
# Repository Configuration
########################################

USER_NAME="trapoom"
REPO_NAME="debian-apt-push"

REPO_URL="https://${USER_NAME}.github.io/${REPO_NAME}"
KEY_URL="${REPO_URL}/traphumi-archive-keyring.gpg"

KEYRING="/usr/share/keyrings/traphumi-archive-keyring.gpg"
LIST_FILE="/etc/apt/sources.list.d/traphumi.list"

########################################

error() {
    echo "Error: Command failed on line ${1}"
    exit 1
}

trap 'error $LINENO' ERR

require_command() {
    command -v "$1" >/dev/null 2>&1
}

install_dependencies() {
    local packages=()

    require_command wget || packages+=(wget)
    require_command gpg || packages+=(gnupg)

    if [ ${#packages[@]} -gt 0 ]; then
        sudo apt-get update
        sudo apt-get install -y "${packages[@]}"
    fi
}

check_network() {
    wget -q --spider https://github.com || {
        echo "No Internet connection."
        exit 1
    }
}

download_key() {

    sudo rm -f "$KEYRING"

    if wget -qO- "$KEY_URL" | sudo gpg --dearmor -o "$KEYRING" 2>/dev/null; then
        return
    fi

    sudo wget -qO "$KEYRING" "$KEY_URL"
}

configure_repository() {

    echo "deb [signed-by=${KEYRING}] ${REPO_URL} stable main" | \
        sudo tee "$LIST_FILE" >/dev/null
}

update_repository() {

    if ! sudo apt-get update; then
        echo "APT update failed."
        exit 1
    fi
}

main() {

    if [ "$(uname -s)" != "Linux" ]; then
        echo "This installer supports Linux only."
        exit 1
    fi

    require_command sudo || {
        echo "sudo is required."
        exit 1
    }

    check_network

    install_dependencies

    download_key

    configure_repository

    update_repository

    echo
    echo "Repository setup completed."
    echo
    echo "Install package:"
    echo "sudo apt install moniter"
}

main "$@"
