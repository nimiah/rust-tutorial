@echo off
cd /d "%~dp0"
target\debug\backend.exe >> backend.log 2>> backend.err.log
