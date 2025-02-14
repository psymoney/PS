struct ProductOfNumbers {
    arr: Vec<i32>,
}

impl ProductOfNumbers {
    fn new() -> Self {
        Self {
            arr: Vec::<i32>::new(),
        }
    }

    fn add(&mut self, num: i32) {
        self.arr.push(num);
    }

    fn get_product(&self, k: i32) -> i32 {
        self.arr.iter().rev().take(k as usize).product()
    }
}


/**
 * Your ProductOfNumbers object will be instantiated and called as such:
 * let obj = ProductOfNumbers::new();
 * obj.add(num);
 * let ret_2: i32 = obj.get_product(k);
 */