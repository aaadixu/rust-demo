

struct MyIterator {
    curr: usize, // 当前计数
    max: usize, // 最大计数
}

impl MyIterator {
    pub fn new(max: usize) -> Self {
        Self {
            curr: 0,
            max,
        }
    }
}

impl Iterator for MyIterator {
    type Item = usize;
    // Self  类型本身
    // self  当前实例
    fn next( &mut self) -> Option<Self::Item>{
        if self.curr < self.max{
            self.curr += 1;
            Some(self.curr)
        }else {
            None
        }
    }
}

pub fn customize_iterator_demo(){
    let mut iter = MyIterator::new(5);
    while let Some(n) =iter.next(){
        println!("count is {}",n)
    }
    // 也可以用 for 循环
    let counter2 = MyIterator::new(3);
    for n in counter2 {
        println!("for loop count = {}", n);
    }
}