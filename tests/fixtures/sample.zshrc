# Sample ZSH configuration for testing

# Git aliases
#@: show git status :f#
alias gs='git status'
#@: stage files for commit :f#
alias ga='git add'
#@: create a commit :f#
alias gc='git commit'

# Navigation shortcuts
# alf
# navigate to projects directory
# fla
alias proj='cd ~/projects'

# Docker helpers
#@: list running docker containers :f#
alias dps='docker ps'

# Example function
# alf
# list all git branches sorted by last commit date
# fla
function gbr() {
    git branch --sort=-committerdate
}

# Complex function with multiple lines
#@: create directory and change into it :f#
function mkcd() {
    mkdir -p "$1"
    cd "$1"
}
