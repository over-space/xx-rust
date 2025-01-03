pub mod phantom_data_01 {
    use std::marker::PhantomData;

    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct Identifier<T> {
        pub id: u64,
        _marker: PhantomData<T>,
    }

    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct Book {
        pub id: Identifier<Self>,
    }

    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct Product {
        pub id: Identifier<Self>,
    }
}

mod phantom_data_02 {
    use std::{
        marker::PhantomData,
        sync::atomic::{AtomicU64, Ordering},
    };

    pub struct FreePlan;
    pub struct PersonalPlan(f32);

    static NEXT_ID: AtomicU64 = AtomicU64::new(1);

    pub struct Customer<T> {
        id: u64,
        name: String,
        _type: PhantomData<T>,
    }

    pub trait Free {
        fn feature1(&self);
        fn feature2(&self);
    }

    pub trait Personal: Free {
        fn advance_feature(&self);
    }

    impl<T> Free for Customer<T> {
        fn feature1(&self) {
            println!("feature 1 for {}", self.name);
        }

        fn feature2(&self) {
            println!("feature 2 for {}", self.name);
        }
    }

    impl Personal for Customer<PersonalPlan> {
        fn advance_feature(&self) {
            println!(
                "Dear {} (as our valuable customer {}), enjoy this advanced feature!",
                self.name, self.id
            );
        }
    }

    impl<T> Customer<T> {
        pub fn new(name: String) -> Self {
            Self {
                id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
                name,
                _type: PhantomData::default(),
            }
        }
    }

    // 将免费转换成付费用户
    impl From<Customer<FreePlan>> for Customer<PersonalPlan> {
        fn from(c: Customer<FreePlan>) -> Self {
            Self::new(c.name)
        }
    }

    /// 订阅成为付费用户
    // pub fn subscribe(customer: Customer<FreePlan>, payment: f32) -> Customer<PersonalPlan> {
    //     // 用户订阅付费了.
    //     let _plan = PersonalPlan(payment);
    //     // 转换成付费账号
    //     customer.into()
    // }

    impl Customer<FreePlan> {
        pub fn subscribe(self, payment: f32) -> Customer<PersonalPlan> {
            // 用户订阅付费了.
            let _plan = PersonalPlan(payment);

            // 转换成付费账号
            self.into()
        }
    }
}

mod phantom_data_03 {
    use std::marker::PhantomData;

    #[derive(Debug, Default)]
    pub struct Linear;

    #[derive(Debug, Default)]
    pub struct Quadratic;

    #[derive(Debug, Default)]
    pub struct Equation<T> {
        current: u32,
        _method: PhantomData<T>,
    }

    impl Iterator for Equation<Linear> {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            self.current += 1;
            if self.current >= u32::MAX {
                return None;
            }
            Some(self.current)
        }
    }

    impl Iterator for Equation<Quadratic> {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            self.current += 1;
            if self.current >= u16::MAX as u32 {
                return None;
            }
            Some(self.current * self.current)
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {

    use crate::{
        phantom_data_01::{Book, Product},
        phantom_data_02::{Customer, Free, FreePlan, Personal},
        phantom_data_03::{Equation, Linear, Quadratic},
    };

    #[test]
    fn test_phantom_data_01() {
        let book = Book::default();
        let product = Product::default();

        // 不能比较
        // assert_eq!(book.id, product.id);
        assert_eq!(book.id.id, product.id.id);
    }

    #[test]
    fn test_phantom_data_02() {
        let free_customer = Customer::<FreePlan>::new("lisi".to_owned());
        free_customer.feature1();
        free_customer.feature2();

        let customer = free_customer.subscribe(8.88);
        // let customer = subscribe(free_customer, 9.99);
        customer.feature1();
        customer.feature2();
        customer.advance_feature();
    }

    #[test]
    fn test_phantom_data_03() {
        let mut linear = Equation::<Linear>::default();
        assert_eq!(Some(1), linear.next());
        assert_eq!(Some(2), linear.next());
        assert_eq!(Some(3), linear.next());

        let mut equation = Equation::<Quadratic>::default();
        assert_eq!(Some(1), equation.next());
        assert_eq!(Some(4), equation.next());
        assert_eq!(Some(9), equation.next());
    }
}
