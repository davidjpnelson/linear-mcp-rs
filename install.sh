#!/usr/bin/env bash
set -euo pipefail

REPO="davidjpnelson/linear-mcp-rs"
INSTALL_DIR="$HOME/.local/bin"

OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Darwin)
    case "$ARCH" in
      arm64) ARTIFACT="linear-mcp-darwin-arm64" ;;
      x86_64) ARTIFACT="linear-mcp-darwin-x64" ;;
      *) echo "Unsupported macOS architecture: $ARCH"; exit 1 ;;
    esac
    ;;
  Linux)
    case "$ARCH" in
      x86_64) ARTIFACT="linear-mcp-linux-x64" ;;
      *) echo "Unsupported Linux architecture: $ARCH"; exit 1 ;;
    esac
    ;;
  *) echo "Unsupported OS: $OS"; exit 1 ;;
esac

echo "Downloading $ARTIFACT..."

mkdir -p "$INSTALL_DIR"

if command -v gh &>/dev/null; then
  gh release download --repo "$REPO" -p "$ARTIFACT" -O "$INSTALL_DIR/linear-mcp" --clobber
elif [ -n "${GITHUB_TOKEN:-}" ]; then
  if ! command -v jq &>/dev/null; then
    echo "Error: 'jq' is required for GITHUB_TOKEN-based downloads."
    echo "Install jq: https://jqlang.github.io/jq/download/"
    exit 1
  fi
  RELEASE_JSON="$(curl -fsL -H "Authorization: token $GITHUB_TOKEN" \
    "https://api.github.com/repos/$REPO/releases/latest")"
  ASSET_URL="$(echo "$RELEASE_JSON" | jq -r --arg name "$ARTIFACT" \
    '.assets[] | select(.name == $name) | .url')"
  if [ -z "$ASSET_URL" ]; then
    echo "Error: Asset '$ARTIFACT' not found in latest release."
    exit 1
  fi
  curl -fsL -H "Authorization: token $GITHUB_TOKEN" -H "Accept: application/octet-stream" \
    "$ASSET_URL" -o "$INSTALL_DIR/linear-mcp"
else
  echo "Error: 'gh' CLI not found and GITHUB_TOKEN not set."
  echo "Install gh: https://cli.github.com"
  echo "Or set GITHUB_TOKEN for curl-based download."
  exit 1
fi

chmod +x "$INSTALL_DIR/linear-mcp"

echo ""
echo "Installed to $INSTALL_DIR/linear-mcp"
echo ""
echo "Next steps:"
echo ""
echo "1. Add your Linear API key (create at https://linear.app/settings/api):"
echo ""
echo "   # Option A: env var (add to ~/.zshrc)"
echo "   export LINEAR_API_KEY=\"lin_api_your_key_here\""
echo ""
echo "   # Option B: macOS Keychain"
echo "   security add-generic-password -s linear-api-key -a \"\$USER\" -w \"lin_api_your_key_here\""
echo ""
echo "2. Add to Claude Code:"
echo ""
echo "   claude mcp add linear-mcp-rs $INSTALL_DIR/linear-mcp"
echo ""
echo "3. Restart Claude Code"
