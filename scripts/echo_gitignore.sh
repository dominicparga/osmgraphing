#!/usr/bin/env sh

#------------------------------------------------------------------------------#
# This file uses gitignore.io, which uses CRLF.
# To replace them by LF, you can use dos2unix
#
# ./scripts/echo_gitignore.sh > .gitignore && dos2unix .gitignore

echo '#------------------------------------------------------------------------------#'
echo '# gitignore'
curl -L -s 'https://www.gitignore.io/api/code,intellij,linux,macos,python,rust,visualstudiocode,windows'
echo ''
echo '#------------------------------------------------------------------------------#'
echo '# custom'
echo ''
echo '/custom/'
echo ''
echo '.vscode/'
echo '!Cargo.lock'
echo ''
echo '# Any map of stuttgart-regbez'
echo '/resources/stuttgart-regbez_[0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]/graph*'
echo ''
echo '# Any map of saarland'
echo '/resources/saarland_[0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]/graph*'
