# 예제 8. 빌려 쓰기와 고쳐 쓰기
관련 레슨: 08 함수, 09 소유권과 빌림

한 줄을 입력받아 앞뒤 공백을 `trim()`한 `String`을 만든다. 그리고 아래 두 함수를 만들어 쓴다.
- `fn count_digits(s: &str) -> usize` : 숫자(0~9)가 몇 개인지 센다. **읽기만** 하므로 `&str`로 빌린다.
- `fn remove_spaces(s: &mut String)` : 문자열에서 공백 `' '`을 모두 지운다. **원본을 고치므로** `&mut String`으로 빌린다.

출력은 세 줄이다.
1. 숫자 개수
2. 공백을 지운 문자열
3. `지우기 전 길이 -> 지운 뒤 길이` (바이트 수, `.len()`)

| 입력 | 출력 |
|---|---|
| `Rust 2024 edition` | `4`<br>`Rust2024edition`<br>`17 -> 15` |
| `  a b c 1 2  ` | `2`<br>`abc12`<br>`9 -> 5` |

💡 문자가 숫자인지는 `ch.is_ascii_digit()`, 빈 `String`은 `String::new()`, 글자 하나 붙이기는 `.push(ch)`.
