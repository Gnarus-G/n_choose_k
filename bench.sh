echo One Thread Naive
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
time nck 25000 15000 bloat &>/dev/null
time nck 25000 15000 bloat &>/dev/null
time nck 99999 9999 bloat &>/dev/null
time nck 999999 9999 bloat &>/dev/null
set +x

echo One Thread + Caching
echo -----------

set -x
time nck 25000 15000 cache &>/dev/null
time nck 25000 15000 cache &>/dev/null
time nck 99999 9999 cache &>/dev/null
time nck 999999 9999 cache &>/dev/null
set +x

echo More Thread
echo -----------

set -x
time nck 25000 15000 multi &>/dev/null
time nck 25000 15000 multi &>/dev/null
time nck 99999 9999 multi &>/dev/null
time nck 999999 9999 multi &>/dev/null
set +x
