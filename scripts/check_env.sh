#!/bin/bash

# Function to check if a command is installed
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check and install Java
if ! command_exists java; then
    echo "Java is not installed. Installing..."

    # Linux
    if [[ $(uname) == "Linux" ]]; then
        sudo apt-get update
        sudo apt-get install default-jdk
    # macOS
    elif [[ $(uname) == "Darwin" ]]; then
        brew update
        brew install openjdk@11
    # Windows
    else
        choco install openjdk11
    fi
fi

# Check and install Rust
if ! command_exists rustc; then
    echo "Rust is not installed. Installing..."

    # Linux / macOS
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

    # Windows (using PowerShell)
    if [[ $(uname) == "Windows_NT" ]]; then
        powershell -Command "(Invoke-WebRequest -Uri https://sh.rustup.rs -UseBasicParsing).Content | sh"
    fi
fi

# Check and install Node.js
if ! command_exists node; then
    echo "Node.js is not installed. Installing..."

    # Linux / macOS
    if [[ $(uname) == "Linux" || $(uname) == "Darwin" ]]; then
        curl -fsSL https://deb.nodesource.com/setup_14.x | sudo -E bash -
        sudo apt-get install -y nodejs
    # Windows
    else
        choco install nodejs
    fi
fi

echo "All required environments are installed."

# Additional steps:
# You might want to check and install package managers for each language (e.g., npm for Node.js)
# You might want to add more specific checks for different package managers (e.g., Brew, Chocolatey)
# Windows installation assumes Chocolatey package manager is available
