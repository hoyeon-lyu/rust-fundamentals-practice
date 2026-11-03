#!/usr/bin/env bash
# 사용법: ./run_tests.sh        (전체)
#        ./run_tests.sh ex2    (예제 하나)
set -u
cd "$(dirname "$0")"
mkdir -p build
TIME_LIMIT=2          # 테스트 하나당 제한 시간(초)
MAX_BYTES=100000      # 출력 최대 크기(바이트)
targets=${1:-$(ls solutions | sed 's/\.rs$//')}
pass=0; fail=0
for ex in $targets; do
  if [ ! -s "solutions/$ex.rs" ]; then
    echo "[$ex] solutions/$ex.rs 가 없거나 비어 있음"; fail=$((fail+1)); continue
  fi
  if ! rustc --edition 2021 -O "solutions/$ex.rs" -o "build/$ex" 2>build/$ex.err; then
    echo "[$ex] 컴파일 실패"; cat build/$ex.err; fail=$((fail+1)); continue
  fi
  for in in tests/$ex/*.in; do
    exp="${in%.in}.out"
    name=$(basename "$in" .in)
    # 제한 시간을 넘기면 종료하고, 출력은 MAX_BYTES까지만 받음 (무한 루프 대비)
    perl -e 'alarm shift; exec @ARGV' "$TIME_LIMIT" "./build/$ex" < "$in" 2>/dev/null | head -c "$MAX_BYTES" > "build/$ex.$name.got"
    status=${PIPESTATUS[0]}
    got=$(cat "build/$ex.$name.got")
    size=$(wc -c < "build/$ex.$name.got" | tr -d " ")
    if [ "$status" -eq 142 ] || [ "$status" -eq 141 ] || [ "$size" -ge "$MAX_BYTES" ]; then
      echo "[$ex] $name: FAIL (시간 초과 또는 출력이 너무 김 → 무한 루프인지 확인)"
      echo "    출력 앞부분: $(printf '%s' "$got" | head -c 120 | tr '\n' ' ')…"; fail=$((fail+1))
    elif [ "$got" == "$(cat "$exp")" ]; then
      echo "[$ex] $name: PASS"; pass=$((pass+1))
    else
      echo "[$ex] $name: FAIL"
      diff <(printf '%s\n' "$got") "$exp" | head -20 | cut -c1-200 | sed 's/^/    /'; fail=$((fail+1))
    fi
  done
done
echo "-- PASS $pass / FAIL $fail"
[ $fail -eq 0 ]
