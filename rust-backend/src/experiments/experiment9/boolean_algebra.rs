pub trait Evaluate<Value, Context> {
    fn evaluate(&self, context: Context) -> Value;
}

impl<Context, Term: Evaluate<bool, Context>, ContextIterator: Iterator<Item = Context>>
    Evaluate<f64, ContextIterator> for Term
{
    fn evaluate(&self, context_iterator: ContextIterator) -> f64 {
        let (trues, falses) = context_iterator.fold((0u32, 0u32), |(trues, falses), context| {
            if self.evaluate(context) {
                (trues + 1, falses)
            } else {
                (trues, falses + 1)
            }
        });
        trues as f64 / (trues + falses) as f64
    }
}

// enum BooleanExpression<SubExpression, Variable> {
//     Var(Variable),
//     Not(SubExpression),
//     And(SubExpression, SubExpression),
//     Or(SubExpression, SubExpression),
// }

// struct BooleanTerm<Variable>(BooleanExpression<Rc<BooleanTerm<Variable>>, Variable>);
