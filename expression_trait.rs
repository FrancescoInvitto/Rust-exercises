pub trait Expression{
    fn eval(&self) -> f64;
}

pub struct Literal{
    value: f64
}

impl Expression for Literal{
    fn eval(&self) -> f64{
        self.value
    }
}

pub struct Product{
    op1: Box<dyn Expression>,
    op2: Box<dyn Expression>,
}

impl Expression for Product{
    fn eval(&self) -> f64{
        self.op1.eval() * self.op2.eval()
    }
}

pub struct Sum{
    op1: Box<dyn Expression>,
    op2: Box<dyn Expression>,
}

impl Expression for Sum{
    fn eval(&self) -> f64{
        self.op1.eval() + self.op2.eval()
    }
}

fn main(){
    let prod1 = Product{op1: Box::new(Literal{value: 3.0}), op2: Box::new(Literal{value: 2.0})};
    let sum1 = Sum{op1: Box::new(Literal{value: 4.0}), op2: Box::new(prod1)};
    let prod2 = Product{op1: Box::new(sum1), op2: Box::new(Literal{value: 5.0})};

    println!("Result = {}", prod2.eval())
}