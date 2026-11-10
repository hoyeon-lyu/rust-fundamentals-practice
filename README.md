# '류호연' 작성함.

## 서론
rust-fundamentals를 공부하면서, 예제가 조금 더 많았으면 좋겠다고 생각하였습니다. 그래서 Claude에게 문제를 출제해 달라고 부탁 한 후,
출제된 문제를 풀이하기로 하였습니다. Claude는 제가 [shipthatcode Rust Fundamentals](https://github.com/hoyeon-lyu/shipthatcode-study-Rust-Fundamentals)에 커밋한 내용을 바탕으로 문제를 출제합니다. 제 문제풀이에 Claude가 관여하지는 아니하고, Claude는 오직 문제 출제, 문제 채점(run_tests.sh 파일 사용), 그리고 해당 폴더 push 이후 문제 피드백만을 수행합니다. 문제 풀이를 통해서 저의 코딩 실력이 실질적으로 좋아지기를 바랍니다.

## 배운점
**rust-fundamentals를 공부하면서** 모듈이나 트레이트를 간편히 불러오려면 `use std::io::{self, BufRead};`와 같이 작성해야함과, BufRead의 lines()를 통해서 입력을 한줄씩 꺼내는 반복자를 만들어, 결과적으로 next()를 이용해서 값을 받아올 수 있다는 것을 알게 되었습니다. 또한, let은 변수를 선언하며 기본적으로 변경 불가(immutable)한 변수로 선언함을, const는 항상 일정한 값을 가지는 상수를 선언함을, mut는 let뒤에 오며, 변경 가능한 변수로 선언함을 앎으로써 변수선언방식을 제 나름대로 이해하였습니다. 

### String / Control Flow
String, Control Flow, Functions, Owning and Borrowing을 배우면서는 유독 Python과 많이 대조하게 되었습니다. Python에서는 `{var:.2f}`와 같이 쓰던것을 Rust에서는 `{:.2}`와 같이 쓰는것, Python에서는 `if condition:`와 같이 조건은 괄호 없이 쓰고 body에는 colon과 들여쓰기로 진입하는 것이 Rust에서는 `if condition{...}`와 같이 (Java와 비슷하다.(`if (condition) {...}`)) 쓰는것이 있습니다. Control Flow에서 Python에서는 `if i in s`, `for i in range(2)`와 같이 광범위 하게 이터러블 객체이기만 하면 되던것이 Rust에서는 `for~in list`나 `for~in range`와 같이 Python보다 좁은범위에서 사용 가능한 점(if문에서 in키워드가 적용되지 않고, for문에서 in키워드를 사용하면 Python보다 적은 경우에 대해서 적용 가능하다.)에서 많이 헤매었습니다(수정 전 커밋 중에 Rust에서 if문에서 in키워드를 사용 가능하다고 작성한 것이 있었는데, 이는 적절치 않은 표현이었습니다.). 제 코드를 보면 이를 해결하기 위해서 `for t in s.chars() {if DIGITS.contains(t) {counts += 1}}`로 적은 것을 볼 수 있습니다. Python에서와 같이 for문, while문이 있지만 Rust에서는 loop문과 loop키워드, 그리고 if문과 약간 다르게 choice와 같이 case에 따라 대입값을 정할 수 있는 것과 유사한 match키워드를 배울 수 있었습니다. 
`<,^,>`는 왼쪽정렬, 가운데 정렬, 오른쪽 정렬을 의미하고, 03은 width를 3으로 하되 빈곳은 0으로 채우라는 의미이고, `{:?}, vec![1,2]`와 같이 쓰면 벡터 [1,2]를 그대로 출력하라는 의미입니다. 해당 내용은 Python에서도 {:?}를 제외하면 동일한 내용이라 이해하는데 어렵지 않았습니다.

### Owning and Borrowing
Owning and Borrowing 개념은 Rust에서 처음 들었습니다. 타입이 String인 객체는 heap에 저장되는 문자열을 가지며, 크기는 자유롭게 조정될 수 있습니다. 문자열을 가지는 String타입의 객체를 참조하는 객체의 타입을 &str이라고 합니다. &을 앞에 붙이면 읽을 수 있음, &mut는 수정 가능함을 의미합니다. 예를 들면, `let mut s:String = String::from("Hello World!");`와 같이 s를 정의하면, s는 Hello World!라는 문자열을 가지는 문자열 객체입니다. 이를 &mut으로 `func(&mut s);`와 같이 작성하면 `(v: &mut String)`으로 함수의 인자로 사용하고, `*v = v.to_uppercase();`처럼 인자 v가 참조하는 s객체가 가지는 문자열을 직접 수정 가능합니다. &String과 같이 작성하면 &str으로 자동 변환 됩니다. Rust에서는 읽을 수 있는 참조 여러개 아니면 수정가능한 참조 한개만 가능하도록 만들어 놓았습니다. 중요한 점은, 문자열을 소유하고 있는 객체 a가 있다고 할때, `let b: String = a;`와 같이 작성하면, a의 문자열에 대한 소유가 b로 움직이게(move) 되어 그 이후로 a를 사용할 수 없다는 점입니다.

### Functions
Python에서는 `def func():`와 같이 쓰던것을 Rust에서는 `fn func(){}`와 같이 쓰고, 특히 Python에서는 함수를 정의할때 들어가는 인자와 나오는 반환값의 타입을 표기하는 것이 필수가 아니고 그저 타입힌트를 전달해줄 뿐인데, Rust에서는 Java에서와 유사하게 `(p1: i32, p2: &str) -> &str`와 같이 작성하면 인자 p1과 p2는 i32와 &str가 되어야하고 &str를 반환해야합니다(Java에서는 `... String func(int num1, String name)...`와 같이 작성한다.). Functions파트에서도 이와같은 대조를 통해서 깊은 이해를 할 수 있었습니다. Python에서는 `input()`이면 되던게 Rust에서는 여러 절차를 거쳐야 해서 익숙치 않았는데, Functions를 배우면서 `input()`을 비슷하게 구현함으로써 Python생각도 많이 하게 되고, 함수의 중요성을 다시 깨닫게 되는 경험을 했습니다. 

### Collections(Vector, HashMap, Iterator)
해당 파트에서는 벡터는 단일 타입을 가지는, 해쉬맵은 key와 value를 가지고 자료들의 순서가 정해져있지 않는 자료들의 묶음이라는 것을 알 수 있었습니다. 그리고 벡터객체는 `vec!`매크로를 이용하거나 `Vec::new()`를 이용하면 생성 가능하고, 해쉬맵객체는, 우선 `use std::collections::HashMap;`으로 HashMap을 "HashMap"이라는 이름으로 사용하겠다고 선언한 후, `HashMap::new()`를 이용하면 생성 가능함을 알게 되었습니다. 벡터와 해쉬맵은 기본적으로 반복자(iterator)가 될 수 있으며, .iter()를 통해서 반복자로 변환가능하고, 반복자로 변환하고 나면 `map()`이나 `filter()`를 사용할 수 있습니다. `.split_whitespace()`와 같은 method를 사용하면 이터레이터 객체를 반환하고, 이터레이터 객체를 `.collect()`를 통해서 벡터객체로 변환 가능함을 알게 되었습니다. Python에서는 `map(func, seq)`임과 같이, 러스트에서는 `iter.map(|item| item.parse::<i64>())`와 같이 구성되어있음을 알 수 있었습니다. 파이썬에서는 map함수를 이용할때에  seq의 각각의 요소(성분)에 적용할 func을 map()함수의 인자로 넣어준 반면에, 러스트에서는 iter.map()의 인자에 각각의 요소(성분)을 어떻게 부를 것인가를 `|item|`와 같이 나타내고, 그 뒤에 item를 이용한 변환식(func)을 적어준다는 것을 알게 되었습니다. filter()도 동일하게 작동함을 알 수 있었습니다. `.split_whitespace().filter().map().collect()`와 같이 여러 메소드로 묶여있는 것도 Python과 유사한 점이 많아 어렵지 않게 해결하였습니다.

### Struct, Enum, Error Handling
struct, enum, result/option은 러스트에서 처음 마주하여 조금 당황하였으나, 코드를 작성하다 보니 어느정도 적응되어 나중에는 어렵지 않게 문제를 해결할 수 있었습니다. struct는 객체를 생성하고, 객체에 속성을 부여할 수 있게 해주고, impl을 통해서 객체가 함수를 가지도록 정의할 수 있었습니다. 예를 들어, `let student1 = Student { name: String::from("lyu"), score_korean: 80 ... };`와 같이 이름이 "류"이고 국어점수가 80점인 학생 객체를 생성해 student1으로 정의할 수 있고, `impl Student { fn total(&self) -> i64 { ... } }`와 같이 작성함으로써 Student객체에 저장된 점수값을 손쉽게 더하고 나타낼 수 있었습니다. 이는 파이썬에서의 class와 메소드 정의 def와 매우 유사했습니다. Enum은 어떠한 자료를 라벨링하는 것과 같았습니다. 라벨링함으로써 해당 자료가 어떤 의미를 갖는지, 어떤 부류로 묶일 수 있는지, 흐름에 어떻게 관여하는지 등을 알 수 있었습니다. 예를 들어, 피연산자들끼리의 연산을 그냥 HashMap으로 {"left_operand": 1, "right_operand": 2}으로 나타내는 것보다, `enum Op { plus(i64,i64) }`와 같이 나타내는 것이 더 깔끔하다는 것(enum을 정의하면 자료들의 기준이 되서 자료를 저장하는 코드를 작성할때마다 동일한 자료구조를 유지할 수 있고, 코드가 겉보기에 정렬되어있음.)이 장점이라고 생각합니다. 또한, HashMap은 HashMap<K, V>로 정의되어 key와 value가 각각 정해진 자료형만을 가질 수 있는 반면에, enum으로 정의하면 훨씬 다양한 구조를 가질 수 있다는 것이 가장 큰 장점이라고 생각합니다. 에러 핸들링(Error Handling)은 enum으로 정의된 Result/Option으로 자료를 감싸서 수행한다고 이해했습니다. 예를 들어, `let s = "asdf";`, `let i = "123";`와 같이 정의한 상태에서 `s.parse::<i64>()`와 `i.parse::<i64>()`를 불러오면, 각각 Result::Err와 Result::Ok를 얻을 수 있습니다. Result는 잘 반환됐을때의 타입과 에러를 구분하고, 에러났을때의 메시지를 받거나 수정할 수 있고, Err()를 보냄으로써 에러를 반환시킬수도 있다는 것을 이해했습니다.

글을 적으면서 Java에 대한 말도 하게 된 점 양해 부탁드립니다(제 첫 입문 언어였습니다.).

**이후 Claude의 조언과 문제 풀이를 통해서** 첫번째로, rust-fundamentals의 복습 예제 풀이 중 lines()를 이용해서 여러번 값을 불러오는 것은 좋지 못한 코드 형태이며, stdin.lock().lines()를 통해서 lines을 mut로 정의한 다음에 lines을 이용해야 함을 알게 되었습니다. 두번째로, interface와 비슷한 개념인 trait(트레이트)에 대해 조금 알게 되었습니다. 해당 지식은 BufRead를 불러오는 과정을 이해하는데에 쓰였습니다. String, Control Flow, Functions, Owning and Borrowing에 대한 문제를 풀면서 String::from(&str매개변수) -> String, &str문자열.contains(&str문자열) -> bool, String문자열.replace(" ","") -> String, 그리고 문자열을 가지는 String객체에 대해 String문자열.clone()에 대해서 습득 할 수 있었습니다. 보통 이론 수업을 듣거나 그러한 article을 보더라도 직접 코드를 짜봐야 메소드/함수들이 뭐가 있는지 알 수 있고, 또 손에 익힐 수 있어서, 그런 점에 있어서 좋았습니다. 마지막 회차에서, match구문 안에서 오직 하나의 타입을 반환하여 match구문 바로 바깥의 `let operation = `에 전달하는 것 뿐만 아니라, 그 안에서 return 또한 동시에 할 수 있다는 것을 알게 되었습니다. 그 이유는 `return`이 `!` (never)타입이기 때문이라고 합니다. 그래서 대입하는 다른 값들의 자료형을 고려할 필요가 없다는 겁니다.

이후 내용은 Claude가 작성하였습니다.

# Rust Fundamentals 복습 예제

[shipthatcode Rust Fundamentals](https://github.com/hoyeon-lyu/shipthatcode-study-Rust-Fundamentals) 레슨을 공부한 뒤(01–15 완료), 배운 내용을 확인하려고 푼 복습 문제 모음입니다. 문제마다 입력/출력 테스트를 두고 스크립트로 채점합니다.

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
| 9 | [점수 통계](problems/ex9.md) | `Vec`, 반복자 `map`·`sum`, `sort` | [ex9.rs](solutions/ex9.rs) | ✅ 3/3 |
| 10 | [알파벳 빈도](problems/ex10.md) | `HashMap`, `entry().or_insert()` | [ex10.rs](solutions/ex10.rs) | ✅ 2/2 |
| 11 | [성적표](problems/ex11.md) | `struct`, `impl` 메서드, `&Student` 참조 | [ex11.rs](solutions/ex11.rs) | ✅ 2/2 |
| 12 | [안전한 계산기](problems/ex12.md) | `enum`, `match`, `Result` | [ex12.rs](solutions/ex12.rs) | ✅ 2/2 |

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
