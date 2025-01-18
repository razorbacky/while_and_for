// for_loop 25.01.18
// for를 이용한 컬렉션에 대한 반복문

fn main() {
    let a = [10, 20, 30, 40, 50]; // [ ] 는 배열이다.
    let mut index = 0;

    // 이런 방식으로 오류가 발생하기 쉽다.
    /* 배열의 정의를 변경 했는데, while 의 조건문 고치는 걸 잊어버린다면 패닉을 일으킨다.
    또한, 컴파일러가 매 반복 회차마다 인덱스가 범위 안에 있는지 조건문 검사를 수행하기에 때문에 느려진다. */
    while index < 5 {
        println!("the value is : {}", a[index]);

        index += 1;
    }
    println!(" ");
    println!("구분을 위한 구분선 -------------");
    println!(" ");

    // 동일한 기능을 하지만, for 문을 사용하여 이렇게 바꾸는 것이 훨씬 쉽고 간결하며 빠르다.
    // for 문은 그냥 있는 것을 검사해서 수행하기 때문에 인덱스가 범위에 벗어나도 상관없다.
    for element in a {
        println!("the value is : {element}");
    }
}
