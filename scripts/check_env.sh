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
    fi
fi

# Check and install Rust
if ! command_exists rustc; then
    echo "Rust is not installed. Installing..."

    # Linux / macOS
    if [[ $(uname) == "Linux" || $(uname) == "Darwin" ]]; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    fi
fi

# Check and install Node.js
if ! command_exists node; then
    echo "Node.js is not installed. Installing..."

    # Linux
    if [[ $(uname) == "Linux" ]]; then
        sudo apt-get update
        sudo apt-get install -y nodejs
    # macOS (using Homebrew)
    elif [[ $(uname) == "Darwin" ]]; then
        brew install node
    fi
fi

# Check and install Maven
if ! command_exists mvn; then
    echo "Maven is not installed. Installing..."

    # Linux
    if [[ $(uname) == "Linux" ]]; then
        sudo apt-get install maven
    # macOS (using Homebrew)
    elif [[ $(uname) == "Darwin" ]]; then
        brew install maven
    fi
fi

echo "All required environments are installed."
