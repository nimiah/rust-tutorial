@echo off
cd /d "%~dp0"
corepack pnpm dev >> frontend.log 2>> frontend.err.log
