# '류호연' 작성함.

rust-fundamentals를 공부하면서, 예제가 조금 더 많았으면 좋겠다고 생각하였습니다. 그래서 Claude에게 문제를 출제해 달라고 부탁 한 후,
출제된 문제를 풀이하기로 하였습니다. Claude는 제가 [shipthatcode Rust Fundamentals](https://github.com/hoyeon-lyu/shipthatcode-study-Rust-Fundamentals)에 커밋한 내용을 바탕으로 문제를 출제합니다. 제 문제풀이에 Claude가 관여하지는 아니하고, Claude는 오직 문제 출제, 문제 채점(run_tests.sh 파일 사용), 그리고 해당 폴더 push 이후 문제 피드백만을 수행합니다. 문제 풀이를 통해서 저의 코딩 실력이 실질적으로 좋아지기를 바랍니다.

이후 내용은 Claude가 작성하였습니다.

# Rust Fundamentals 복습 예제

[shipthatcode Rust Fundamentals](https://github.com/hoyeon-lyu/shipthatcode-study-Rust-Fundamentals) 레슨 01–04를 공부한 뒤, 배운 내용을 확인하려고 푼 복습 문제 모음입니다. 문제마다 입력/출력 테스트를 두고 스크립트로 채점합니다.

## 문제 목록
| # | 문제 | 다루는 개념 | 풀이 | 테스트 |
|---|---|---|---|---|
| 1 | [자기소개](problems/ex1.md) | `println!`, 섀도잉, `parse` | [ex1.rs](solutions/ex1.rs) | ❌ 0/2 (출력 끝 `.` 누락) |
| 2 | [초를 시·분·초로](problems/ex2.md) | `i64`, `/`, `%` | [ex2.rs](solutions/ex2.rs) | ✅ 2/2 |
| 3 | [누적합과 평균](problems/ex3.md) | `let mut`, 몫·나머지 | [ex3.rs](solutions/ex3.rs) | ❌ 0/2 (출력 형식 다름) |
| 4 | [글자 수 vs 바이트 수](problems/ex4.md) | `String`, `&str`, UTF-8 | [ex4.rs](solutions/ex4.rs) | ✅ 2/2 |

## 실행 방법
Rust(`rustc`)가 설치되어 있어야 합니다.
```sh
./run_tests.sh        # 전체 채점
./run_tests.sh ex4    # 예제 하나만
```
틀리면 실제 출력과 기대 출력의 차이(diff)를 보여줍니다.

## 폴더 구조
```
problems/   문제 설명
solutions/  내 풀이 (exN.rs)
tests/      exN/1.in, 1.out … 입력과 기대 출력
run_tests.sh
```