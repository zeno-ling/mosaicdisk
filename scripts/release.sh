#!/usr/bin/env bash
# MosaicDisk 一键打包发布脚本（Linux / macOS）
# 用法：./scripts/release.sh [-v v0.1.0] [-r owner/repo] [-s] [-n]
#   -v  指定版本（默认从 Cargo.toml 读取）
#   -r  GitHub 仓库（默认 zeno-ling/mosaicdisk）
#   -s  跳过编译，用现有二进制打包
#   -n  只打包，不上传 GitHub Release

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPO="zeno-ling/mosaicdisk"
VERSION=""
SKIP_BUILD=0
NO_UPLOAD=0

while getopts "v:r:sn" opt; do
    case "$opt" in
        v) VERSION="$OPTARG" ;;
        r) REPO="$OPTARG" ;;
        s) SKIP_BUILD=1 ;;
        n) NO_UPLOAD=1 ;;
        *) echo "用法: $0 [-v 版本] [-r 仓库] [-s 跳过编译] [-n 只打包]" >&2; exit 1 ;;
    esac
done

cd "$ROOT"

# 1) 版本号（从 Cargo.toml 的 package.version 读取）
if [[ -z "$VERSION" ]]; then
    VERSION="$(sed -n 's/^version[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' Cargo.toml | head -n1)"
    if [[ -z "$VERSION" ]]; then
        echo "错误: 无法从 Cargo.toml 读取版本号，请用 -v 指定" >&2
        exit 1
    fi
fi
VER="${VERSION#v}"
TAG="v$VER"

# 2) 平台与架构
OS_RAW="$(uname -s)"
case "$OS_RAW" in
    Linux)  OS="linux" ;;
    Darwin) OS="macos" ;;
    *)      OS="$(echo "$OS_RAW" | tr '[:upper:]' '[:lower:]')" ;;
esac
ARCH_RAW="$(uname -m)"
case "$ARCH_RAW" in
    x86_64|amd64) ARCH="x64" ;;
    aarch64|arm64) ARCH="arm64" ;;
    *) ARCH="$ARCH_RAW" ;;
esac
TAR_NAME="MosaicDisk-$VER-$OS-$ARCH.tar.gz"

echo "== MosaicDisk 发布打包 $TAG ($OS-$ARCH) =="

# 3) 编译
if [[ "$SKIP_BUILD" -eq 0 ]]; then
    echo "编译 release（--no-default-features 使用系统分配器）..."
    cargo build --release --no-default-features
fi

BIN="target/release/mosaicdisk"
if [[ ! -f "$BIN" ]]; then
    echo "错误: 找不到 $BIN，请先编译" >&2
    exit 1
fi

# 4) 汇集文件
STAGE="target/release-stage-$OS-$ARCH"
rm -rf "$STAGE"
mkdir -p "$STAGE/locale/zh_CN/LC_MESSAGES"
cp "$BIN" "$STAGE/mosaicdisk"
cp -R locale/. "$STAGE/locale/"
cp README.md LICENSE "$STAGE/"
# 确保语言包与源码一致
cp locale/zh_CN/LC_MESSAGES/default.po "$STAGE/locale/zh_CN/LC_MESSAGES/default.po"

# 5) 压缩
TAR="target/$TAR_NAME"
rm -f "$TAR"
tar -czf "$TAR" -C "$STAGE" .
echo "已生成 $TAR_NAME ($(du -h "$TAR" | cut -f1))"

# 6) 上传 GitHub Release
if [[ "$NO_UPLOAD" -eq 0 ]]; then
    if ! command -v gh >/dev/null 2>&1; then
        echo "错误: 未安装 GitHub CLI (gh)，或加 -n 只打包" >&2
        exit 1
    fi
    if ! gh auth status >/dev/null 2>&1; then
        echo "错误: gh 未登录，请先运行 gh auth login，或加 -n" >&2
        exit 1
    fi

    if gh release view "$TAG" --repo "$REPO" >/dev/null 2>&1; then
        echo "Release $TAG 已存在，覆盖上传资产..."
        gh release upload "$TAG" "$TAR" --repo "$REPO" --clobber
    else
        echo "创建 Release $TAG 并上传资产..."
        NOTES="MosaicDisk $VER — $OS-$ARCH 绿色版，解压即用。内置简体中文界面，直接读取 NTFS MFT。"
        gh release create "$TAG" "$TAR" --repo "$REPO" \
            --title "MosaicDisk $VER — $OS-$ARCH" --notes "$NOTES"
    fi
    echo "发布完成: https://github.com/$REPO/releases/tag/$TAG"
else
    echo "跳过上传 (-n)。资产位于: $TAR"
fi
