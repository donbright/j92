trait ConvertTo<T> {
    fn convert(&self) -> T;
}

impl ConvertTo<f64> for i32 {
    fn convert(&self) -> f64 {
        *self as f64
    }
}

trait MyTrait {
    type Item;

    // Specify the correct lifetime bound
    fn iter(&self) -> Box<dyn Iterator<Item = &Self::Item> + '_>;

    fn my_adapter<T>(&self) -> MyAdapter<T, Self::Item>
    where
        Self::Item: ConvertTo<T> + 'static,
    {
        MyAdapter {
            iter: self.iter(),
            _marker: std::marker::PhantomData,
        }
    }
}

struct MyStruct {
    data: Vec<i32>,
}

impl MyTrait for MyStruct {
    type Item = i32;

    fn iter(&self) -> Box<dyn Iterator<Item = &Self::Item> + '_> {
        Box::new(self.data.iter())
    }
}

struct MyAdapter<'a, T, R>
where
    R: ConvertTo<T> + 'a,
{
    iter: Box<dyn Iterator<Item = &'a R> + 'a>,
    _marker: std::marker::PhantomData<T>,
}

impl<'a, T, R> Iterator for MyAdapter<'a, T, R>
where
    R: ConvertTo<T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| x.convert())
    }
}

#[test]
fn test_iter() {
    let mystruct = MyStruct { data: vec![1, 2, 3] };

    let x: Vec<f64> = mystruct.my_adapter().collect();
    println!("{:?}", x); // [1.0, 2.0, 3.0]
}

