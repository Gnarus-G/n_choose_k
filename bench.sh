echo One Thread
echo -----------

set -x
time nck 25000 15000 &>/dev/null
time nck 25000 15000 &>/dev/null
time nck 99999 9999 &>/dev/null
time nck 999999 9999 &>/dev/null
set +x

echo More Thread
echo -----------

set -x
time nck -m 25000 15000 &>/dev/null
time nck -m 25000 15000 &>/dev/null
time nck -m 99999 9999 &>/dev/null
time nck -m 999999 9999 &>/dev/null
set +x