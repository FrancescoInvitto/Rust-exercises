pub enum Expression{
    Literal(f64),
    Product(Box<Expression>, Box<Expression>),
    Sum(Box<Expression>, Box<Expression>),
}

impl Expression{
    pub fn eval(&self) -> f64{
        match self{
            Expression::Literal(value) => *value,
            Expression::Product(op1, op2) => op1.eval() * op2.eval(),
            Expression::Sum(op1, op2) => op1.eval() + op2.eval(),
        }
    }
}

fn main(){
    let prod1 = Expression::Product(Box::new(Expression::Literal(3.0)), Box::new(Expression::Literal(2.0)));
    let sum1 = Expression::Sum(Box::new(Expression::Literal(4.0)), Box::new(prod1));
    let prod2 = Expression::Product(Box::new(sum1), Box::new(Expression::Literal(5.0)));

    println!("Result = {}", prod2.eval())
}

