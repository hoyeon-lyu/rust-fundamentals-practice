# '류호연' 작성함.

## 서론
rust-fundamentals를 공부하면서, 예제가 조금 더 많았으면 좋겠다고 생각하였습니다. 그래서 Claude에게 문제를 출제해 달라고 부탁 한 후,
출제된 문제를 풀이하기로 하였습니다. Claude는 제가 [shipthatcode Rust Fundamentals](https://github.com/hoyeon-lyu/shipthatcode-study-Rust-Fundamentals)에 커밋한 내용을 바탕으로 문제를 출제합니다. 제 문제풀이에 Claude가 관여하지는 아니하고, Claude는 오직 문제 출제, 문제 채점(run_tests.sh 파일 사용), 그리고 해당 폴더 push 이후 문제 피드백만을 수행합니다. 문제 풀이를 통해서 저의 코딩 실력이 실질적으로 좋아지기를 바랍니다.

## 배운점
**rust-fundamentals를 공부하면서** 모듈이나 트레이트를 간편히 불러오려면 `use std::io::{self, BufRead};`와 같이 작성해야함과, BufRead의 lines()를 통해서 입력을 한줄씩 꺼내는 반복자를 만들어, 결과적으로 next()를 이용해서 값을 받아올 수 있다는 것을 알게 되었습니다. 또한, let은 변수를 선언하며 기본적으로 변경 불가(immutable)한 변수로 선언함을, const는 항상 일정한 값을 가지는 상수를 선언함을, mut는 let뒤에 오며, 변경 가능한 변수로 선언함을 앎으로써 변수선언방식을 제 나름대로 이해하였습니다.

**이후 Claude의 조언과 문제 풀이를 통해서** 첫번째로, rust-fundamentals의 복습 예제 풀이 중 lines()를 이용해서 여러번 값을 불러오는 것은 좋지 못한 코드 형태이며, stdin.lock().lines()를 통해서 line을 mut로 정의한 다음에 line을 이용해야 함을 알게 되었습니다. 두번째로, interface와 비슷한 개념인 trait(트레이트)에 대해 조금 알게 되었습니다. 해당 지식은 BufRead를 불러오는 과정을 이해하는데에 쓰였습니다.

이후 내용은 Claude가 작성하였습니다.

# Rust Fundamentals 복습 예제

[shipthatcode Rust Fundamentals](https://github.com/hoyeon-lyu/shipthatcode-study-Rust-Fundamentals) 레슨을 공부한 뒤(현재 01–09), 배운 내용을 확인하려고 푼 복습 문제 모음입니다. 문제마다 입력/출력 테스트를 두고 스크립트로 채점합니다.

## 문제 목록
| # | 문제 | 다루는 개념 | 풀이 | 테스트 |
|---|---|---|---|---|
| 1 | [자기소개](problems/ex1.md) | `println!`, 섀도잉, `parse` | [ex1.rs](solutions/ex1.rs) | ✅ 2/2 |
| 2 | [초를 시·분·초로](problems/ex2.md) | `i64`, `/`, `%` | [ex2.rs](solutions/ex2.rs) | ✅ 2/2 |
| 3 | [누적합과 평균](problems/ex3.md) | `let mut`, 몫·나머지 | [ex3.rs](solutions/ex3.rs) | ✅ 2/2 |
| 4 | [글자 수 vs 바이트 수](problems/ex4.md) | `String`, `&str`, UTF-8 | [ex4.rs](solutions/ex4.rs) | ✅ 2/2 |
| 5 | [시계 형식으로 출력하기](problems/ex5.md) | `{:02}` 포맷 지정 | [ex5.rs](solutions/ex5.rs) | ✅ 3/3 |
| 6 | [학점 계산기](problems/ex6.md) | `match` 범위 패턴 | [ex6.rs](solutions/ex6.rs) | ✅ 3/3 |
| 7 | [콜라츠 수열](problems/ex7.md) | `loop`, `break`, `if / else` | [ex7.rs](solutions/ex7.rs) | ✅ 3/3 |
| 8 | [빌려 쓰기와 고쳐 쓰기](problems/ex8.md) | 함수, `&str`, `&mut String` | [ex8.rs](solutions/ex8.rs) | ✅ 2/2 |

## 실행 방법
Rust(`rustc`)가 설치되어 있어야 합니다.
```sh
./run_tests.sh        # 전체 채점
./run_tests.sh ex4    # 예제 하나만
```
틀리면 실제 출력과 기대 출력의 차이(diff)를 보여줍니다. 테스트 하나당 2초 제한이 있어서, 무한 루프가 생기면 멈추고 `시간 초과`로 표시합니다.

## 폴더 구조
```
problems/   문제 설명
solutions/  내 풀이 (exN.rs)
tests/      exN/1.in, 1.out … 입력과 기대 출력
run_tests.sh
```
