# Sample ZSH configuration for testing

# Git aliases
alias gs='git status'
alias ga='git add'
alias gc='git commit'

# Navigation shortcuts
# Navigate to projects directory
alias proj='cd ~/projects'

# Docker helpers
alias dps='docker ps'

# Example function
# Lists all git branches sorted by last commit date
function gbr() {
    git branch --sort=-committerdate
}

# Complex function with multiple lines
function mkcd() {
    mkdir -p "$1"
    cd "$1"
}
