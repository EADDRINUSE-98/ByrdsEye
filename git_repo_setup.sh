#!/bin/bash
set -uo pipefail

REMOTE_GITHUB_REPO_SSH_URL="${1:-}"
if [[ -z "${REMOTE_GITHUB_REPO_SSH_URL}" ]]; then
  echo "Usage: ./git_repo_setup.sh [REMOTE_GITHUB_REPO_SSH_URL]"
  exit 1
fi

# Basic sanity check on the URL format (git@github.com:user/repo.git or ssh://...)
if [[ ! "${REMOTE_GITHUB_REPO_SSH_URL}" =~ ^(git@github\.com:|ssh://git@github\.com/) ]]; then
  echo "WARNING: '${REMOTE_GITHUB_REPO_SSH_URL}' doesn't look like a GitHub SSH URL."
  echo "Expected something like: git@github.com:user/repo.git"
  echo ""
fi

DEFAULT_GIT_BRANCH="$(git config --global --get init.defaultBranch)"
echo "Default branch: ${DEFAULT_GIT_BRANCH:-<unset>}"
echo ""

if [[ "${DEFAULT_GIT_BRANCH}" != "main" ]]; then
  git config --global init.defaultBranch main && echo "Changing default branch to: main"
  echo ""
fi

if [[ -d ".git" ]]; then
  echo "Git repo is already initialized here."
  echo ""
else
  git init && echo "Git repo initialized."
  echo ""
fi

SSH_GITHUB_AUTH_STATUS=$(ssh -T git@github.com 2>&1)
if echo "${SSH_GITHUB_AUTH_STATUS}" | grep -qi 'successfully authenticated'; then
  echo "SSH authentication successful."
  echo ""
else
  echo "ERROR: Either ssh-agent is not running or the ssh key is not added."
  echo "Details: ${SSH_GITHUB_AUTH_STATUS}"
  echo -e "Use: eval \"\$(ssh-agent -s)\"; ssh-add <path/to/privatekey>"
  exit 1
fi

if git remote set-url origin "${REMOTE_GITHUB_REPO_SSH_URL}" >/dev/null 2>/dev/null ||
  git remote add origin "${REMOTE_GITHUB_REPO_SSH_URL}"; then
  echo "origin set to: ${REMOTE_GITHUB_REPO_SSH_URL}, successfully."
fi
echo ""
