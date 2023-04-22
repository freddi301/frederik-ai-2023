import ai.utils.*

trait Checkable[Term, Context]:
  extension (term: Term) def check(context: Context): Option[Boolean]

case class At[Symbol](relativeIndex: Int, symbol: Symbol)
given [Symbol](using Ordering[Symbol]): Ordering[At[Symbol]] = Ordering.by(Tuple.fromProductTyped[At[Symbol]])

case class Cursor[Symbol](index: Int, sequence: scala.collection.immutable.IndexedSeq[Symbol])
  // require(index >= 0 && index < sequence.length)

given [Symbol]: Checkable[At[Symbol], Cursor[Symbol]] with
  extension (at: At[Symbol]) def check(cursor: Cursor[Symbol]): Option[Boolean] =
    cursor.sequence.lift(cursor.index + at.relativeIndex).map(_ == at.symbol)

case class Case[Variable](condition: scala.collection.immutable.SortedMap[Variable, Boolean], consequence: Variable):
  require(!condition.contains(consequence))
  def occurrenceAndAccuracy[Context](contexts: Iterable[Context])(using Checkable[Variable, Context]): (Double, Double) =
    val (isOccurredCount, isAccurateCount) = contexts.foldLeft((0,0))({ case ((isOccurredCount, isAccurateCount), context) =>
      val isOccurred = condition.forall((variable, expected) => variable.check(context) == Some(expected))
      val isAccurate = isOccurred && consequence.check(context).contains(true)
      (isOccurredCount + (if isOccurred then 1 else 0), isAccurateCount + (if isAccurate then 1 else 0))
    })
    (isOccurredCount.toDouble / contexts.size.toDouble, isAccurateCount.toDouble / isOccurredCount.toDouble)

// ---

extension [Symbol](data: scala.collection.immutable.IndexedSeq[Symbol])
  def into: Iterable[Cursor[Symbol]] = (0 until data.size).map((index) => Cursor(index, data))

extension (data: String)
  def into: Iterable[Cursor[Char]] = data.toIndexedSeq.into

given [K, V](using Ordering[K]): Conversion[Iterable[(K, V)], scala.collection.immutable.SortedMap[K, V]] with
  def apply(iterable: Iterable[(K, V)]): scala.collection.immutable.SortedMap[K, V] = scala.collection.immutable.SortedMap.from(iterable)

val ilPiccoloPrincipe = scala.io.Source.fromResource("il-piccolo-principe.txt").simplifyItalian.mkString.into

Case(Seq[(At[Char], Boolean)](), At(0, 'a')).occurrenceAndAccuracy(ilPiccoloPrincipe)
Case(Seq[(At[Char], Boolean)](), At(0, ' ')).occurrenceAndAccuracy(ilPiccoloPrincipe)
Case(Seq(At(-1, 'm') -> true), At(0, 'a')).occurrenceAndAccuracy(ilPiccoloPrincipe)
Case(Seq(At(-2, 'a') -> true, At(-1, 'r') -> true), At(0, 'e')).occurrenceAndAccuracy(ilPiccoloPrincipe)
