#!/usr/bin/env bash
# 사용법: ./run_tests.sh        (전체)
#        ./run_tests.sh ex2    (예제 하나)
set -u
cd "$(dirname "$0")"
mkdir -p build
targets=${1:-$(ls solutions | sed 's/\.rs$//')}
pass=0; fail=0
for ex in $targets; do
  if ! rustc --edition 2021 -O "solutions/$ex.rs" -o "build/$ex" 2>build/$ex.err; then
    echo "[$ex] 컴파일 실패"; cat build/$ex.err; fail=$((fail+1)); continue
  fi
  for in in tests/$ex/*.in; do
    exp="${in%.in}.out"
    got=$("./build/$ex" < "$in" 2>/dev/null)
    if [ "$got" == "$(cat "$exp")" ]; then
      echo "[$ex] $(basename "$in" .in): PASS"; pass=$((pass+1))
    else
      echo "[$ex] $(basename "$in" .in): FAIL"
      diff <(echo "$got") "$exp" | sed 's/^/    /'; fail=$((fail+1))
    fi
  done
done
echo "-- PASS $pass / FAIL $fail"
[ $fail -eq 0 ]
