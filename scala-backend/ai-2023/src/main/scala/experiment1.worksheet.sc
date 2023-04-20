import scala.deriving.*
import scala.compiletime.{erasedValue, summonInline}

inline def summonAll[T <: Tuple]: List[Eq[_]] =
  inline erasedValue[T] match
    case _: EmptyTuple => Nil
    case _: (t *: ts) => summonInline[Eq[t]] :: summonAll[ts]

trait Eq[T]:
  def eqv(x: T, y: T): Boolean

def eqv[T](x: T, y: T)(using Eq[T]): Boolean = summon[Eq[T]].eqv(x, y)

object Eq:
  given Eq[Int] with
    def eqv(x: Int, y: Int) = x == y

  def check(elem: Eq[_])(x: Any, y: Any): Boolean =
    elem.asInstanceOf[Eq[Any]].eqv(x, y)

  def iterator[T](p: T) = p.asInstanceOf[Product].productIterator

  def eqSum[T](s: Mirror.SumOf[T], elems: => List[Eq[_]]): Eq[T] =
    new Eq[T]:
      def eqv(x: T, y: T): Boolean =
        val ordx = s.ordinal(x)
        (s.ordinal(y) == ordx) && check(elems(ordx))(x, y)

  def eqProduct[T](p: Mirror.ProductOf[T], elems: => List[Eq[_]]): Eq[T] =
    new Eq[T]:
      def eqv(x: T, y: T): Boolean =
        iterator(x).zip(iterator(y)).zip(elems.iterator).forall { case ((x, y), elem) =>
          check(elem)(x, y)
        }

  inline given derived[T](using m: Mirror.Of[T]): Eq[T] =
    lazy val elemInstances = summonAll[m.MirroredElemTypes]
    inline m match
      case s: Mirror.SumOf[T] => eqSum(s, elemInstances)
      case p: Mirror.ProductOf[T] => eqProduct(p, elemInstances)

  given Eq[String] with
    def eqv(x: String, y: String) = x == y

  given Eq[Boolean] with
    def eqv(x: Boolean, y: Boolean) = x == y

  given [Item](using itemEq: Eq[Item]): Eq[Vector[Item]] with
    def eqv(x: Vector[Item], y: Vector[Item]): Boolean =
      x.size == y.size && x.zip(y).forall((a, b) => itemEq.eqv(a, b))

end Eq

def levelUp[Variable](
    level: Set[Term[Variable]]
): Set[Term[Variable]] =
  level.map(x => Term.Not(x)) ++
    level.flatMap(x => level.map(y => Term.And(x, y))) ++
    level.flatMap(x => level.map(y => Term.Or(x, y)))

trait Quantify[Variable, Context]:
  def apply(variable: Variable, context: Context): Boolean

enum Term[Variable] derives Eq:
  case Var(x: Variable)
  case Not(x: Term[Variable])
  case And(x: Term[Variable], y: Term[Variable])
  case Or(x: Term[Variable], y: Term[Variable])
  def getVariables(using Ordering[Variable]): scala.collection.SortedSet[Variable] = this match
    case Var(x) => scala.collection.SortedSet(x)
    case Not(x) => x.getVariables
    case And(x, y) => x.getVariables ++ y.getVariables
    case Or(x, y) => x.getVariables ++ y.getVariables
  def evaluate[Context](context: Context)(using quantify: Quantify[Variable, Context]): Boolean = this match
    case Var(x) => quantify(x, context)
    case Not(x) => !x.evaluate(context)
    case And(x, y) => x.evaluate(context) && y.evaluate(context)
    case Or(x, y) => x.evaluate(context) || y.evaluate(context)
  def getTruthTable(using Ordering[Variable]): TruthTable[Variable] =
    val variables = getVariables.toVector
    val variablesIndex = variables.zipWithIndex.map((item, index) => item -> index).toMap
    given Quantify[Variable, Int] with
      def apply(variable: Variable, combinationIndex: Int): Boolean =
        ((combinationIndex >> variablesIndex(variable)) & 1) == 1
    val results = Vector.tabulate(1 << variables.size)(evaluate)
    TruthTable(variables, results)

case class TruthTable[Variable](
    variables: Vector[Variable],
    results: Vector[Boolean]
) derives Eq:
  def evaluate[Context](context: Context)(using quantify: Quantify[Variable, Context]): Boolean =
    results(
      variables.foldLeft((0, 0))((memo, variable) => {
        val (resultIndex, variableIndex) = memo
        if quantify(variable, context) then (resultIndex | (1 << variableIndex), variableIndex + 1)
        else (resultIndex, variableIndex + 1)
      })(0)
    )

levelUp(Set("a", "b", "c").map(Term.Var.apply)).size;
levelUp(Set("a", "b", "c").map(Term.Var.apply)).map(_.getTruthTable).size;

levelUp(levelUp(Set("a", "b", "c", "d").map(Term.Var.apply))).size;
levelUp(levelUp(Set("a", "b", "c", "d").map(Term.Var.apply))).map(_.getTruthTable).size;

given Quantify[String, Map[String, Boolean]] with
  def apply(variable: String, context: Map[String, Boolean]): Boolean = context(variable)

Term.And(Term.Var("a"), Term.Var("a")).getTruthTable.evaluate(Map("a" -> false, "b" -> true))

eqv(Term.And(Term.Var("a"), Term.Var("a")).getTruthTable, Term.Var("a").getTruthTable)
