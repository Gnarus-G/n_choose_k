echo One Thread w/out Pointless Bloat
echo -----------

set -x
time nck 25000 15000 &>/dev/null
time nck 25000 15000 &>/dev/null
time nck 99999 9999 &>/dev/null
time nck 999999 9999 &>/dev/null
set +x

echo One Thread w/ Pointless Bloat
echo -----------

set -x
time nck -b 25000 15000 &>/dev/null
time nck -b 25000 15000 &>/dev/null
time nck -b 99999 9999 &>/dev/null
time nck -b 999999 9999 &>/dev/null
set +x

echo One Thread + Caching
echo -----------

set -x
time nck -c 25000 15000 &>/dev/null
time nck -c 25000 15000 &>/dev/null
time nck -c 99999 9999 &>/dev/null
time nck -c 999999 9999 &>/dev/null
set +x

echo More Thread
echo -----------

set -x
time nck -m 25000 15000 &>/dev/null
time nck -m 25000 15000 &>/dev/null
time nck -m 99999 9999 &>/dev/null
time nck -m 999999 9999 &>/dev/null
set +x
