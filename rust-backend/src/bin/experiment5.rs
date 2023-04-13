mod boolean_algebra {
    use std::rc::Rc;
    #[derive(Debug)]
    enum Base<Variable> {
        Variable(Rc<Variable>),
        Not(Rc<Self>),
        And(Rc<Self>, Rc<Self>),
        Or(Rc<Self>, Rc<Self>),
    }

    #[derive(Debug)]
    enum Extended<Variable> {
        Variable(Rc<Variable>),
        Not(Rc<Self>),
        And(Rc<Self>, Rc<Self>),
        Or(Rc<Self>, Rc<Self>),
        Xor(Rc<Self>, Rc<Self>),
        Conditional(Rc<Self>, Rc<Self>),
        Biconditional(Rc<Self>, Rc<Self>),
    }

    #[derive(Debug)]
    enum Normal<Variable> {
        Variable(Rc<Variable>),
        True,
        And(Rc<Self>, Rc<Self>),
        Xor(Rc<Self>, Rc<Self>),
    }

    // https://en.wikipedia.org/wiki/Boolean_algebra#Secondary_operations
    impl<Variable> From<&Extended<Variable>> for Base<Variable> {
        fn from(term: &Extended<Variable>) -> Self {
            fn conv<Variable>(term: &Extended<Variable>) -> Rc<Base<Variable>> {
                Rc::new(Base::from(term))
            }
            match term {
                Extended::Variable(variable) => Base::Variable(variable.clone()),
                Extended::Not(x) => Base::Not(conv(x)),
                Extended::And(x, y) => Base::And(conv(x), conv(y)),
                Extended::Or(x, y) => Base::Or(conv(x), conv(y)),
                Extended::Xor(x, y) => {
                    let x = conv(x);
                    let y = conv(y);
                    Base::And(
                        Rc::new(Base::Or(x.clone(), y.clone())),
                        Rc::new(Base::Not(Rc::new(Base::And(x.clone(), y.clone())))),
                    )
                }
                Extended::Conditional(x, y) => {
                    let x = conv(x);
                    let y = conv(y);
                    Base::Or(Rc::new(Base::Not(x)), y)
                }
                Extended::Biconditional(x, y) => {
                    let x = conv(x);
                    let y = conv(y);
                    Base::Or(
                        Rc::new(Base::And(x.clone(), y.clone())),
                        Rc::new(Base::And(
                            Rc::new(Base::Not(x.clone())),
                            Rc::new(Base::Not(y.clone())),
                        )),
                    )
                }
            }
        }
    }

    // https://en.wikipedia.org/wiki/Algebraic_normal_form#Converting_to_algebraic_normal_form
    // TODO
    // impl<Variable> From<&Base<Variable>> for Normal<Variable> {}

    trait Evaluate<Context> {
        fn evaluate(&self, context: &Context) -> bool;
    }

    impl<Context, Variable: Evaluate<Context>> Evaluate<Context> for Base<Variable> {
        fn evaluate(&self, context: &Context) -> bool {
            use Base::*;
            match self {
                Variable(variable) => variable.evaluate(context),
                Not(x) => !x.evaluate(context),
                And(x, y) => x.evaluate(context) && y.evaluate(context),
                Or(x, y) => x.evaluate(context) || y.evaluate(context),
            }
        }
    }
}

fn main() {}
