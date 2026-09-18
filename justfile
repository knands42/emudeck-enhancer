setup-hook:
    cp scripts/commit-hook.sh .git/hooks/pre-commit
    chmod +x .git/hooks/pre-commit