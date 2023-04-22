import ai.utils.*

trait Checkable[Term, Context]:
  def check(term: Term, context: Context): Option[Boolean]

case class At[Symbol](relativeIndex: Int, symbol: Symbol)
case class Cursor[Symbol](index: Int, sequence: IndexedSeq[Symbol])

given [Symbol]: Checkable[At[Symbol], Cursor[Symbol]] with
  def check(at: At[Symbol], cursor: Cursor[Symbol]): Option[Boolean] =
    cursor.sequence.lift(cursor.index + at.relativeIndex).map(_ == at.symbol)

def probability[Term, Context](term: Term, contexts: Iterable[Context])(using Checkable[Term, Context]): Float =
  val (trues, falses) = contexts.foldLeft((0, 0))({ case ((trues, falses), context) =>
    summon.check(term, context) match
      case Some(true) => (trues + 1, falses)
      case Some(false) => (trues, falses + 1)
      case None => (trues, falses)
  });
  trues.toFloat / (trues + falses).toFloat

def probabilityBy[Term, Context](term: Term, other: Term, contexts: Iterable[Context])(using Checkable[Term, Context]): Float =
  probability(term, contexts.filter((context) => summon.check(other, context).contains(true)))

extension [A](a: A) def into[B](using Into[A, B]): B = summon.into(a)

trait Into[A, B]:
  def into(a: A): B

given [Symbol]: Into[IndexedSeq[Symbol], Iterable[Cursor[Symbol]]] with
  def into(data: IndexedSeq[Symbol]): Iterable[Cursor[Symbol]] = (0 until data.size).map((index) => Cursor(index, data))

given Into[String, Iterable[Cursor[Char]]] with
  def into(data: String): Iterable[Cursor[Char]] = summon[Into[IndexedSeq[Char], Iterable[Cursor[Char]]]].into(data)

def probabilityOfCurrentSymbol[Symbol](sequence: IndexedSeq[Symbol]): Map[Symbol, Float] =
  sequence.toSet.view.map((symbol) => (symbol, probability(At(0, symbol), sequence.into))).toMap

def probabilityOfCurrentSymbolBy[Symbol, Term](sequence: IndexedSeq[Symbol], other: Term)(using
    Into[At[Symbol], Term],
    Checkable[Term, Cursor[Symbol]]
): Map[Symbol, Float] =
  sequence.toSet.view.map((symbol) => (symbol, probabilityBy(At(0, symbol).into, other, sequence.into))).toMap

enum Term[Extension]:
  case Ext(extension: Extension)
  case Not(term: Term[Extension])
  case And(left: Term[Extension], right: Term[Extension])
  case Or(left: Term[Extension], right: Term[Extension])

given [Extension, Context](using Checkable[Extension, Context]): Checkable[Term[Extension], Context] with
  def check(term: Term[Extension], context: Context): Option[Boolean] =
    term match
      case Term.Ext(extension) => summon.check(extension, context)
      case Term.Not(term) => for (term <- check(term, context)) yield !term
      case Term.And(left, right) => for (left <- check(left, context); right <- check(right, context)) yield left && right
      case Term.Or(left, right) => for (left <- check(left, context); right <- check(right, context)) yield left || right

import Term.*

given [Symbol]: Conversion[At[Symbol], Term[At[Symbol]]] with
  def apply(at: At[Symbol]): Term[At[Symbol]] = Ext(at)

given [Extension]: Into[Extension, Term[Extension]] with
  def into(extension: Extension): Term[Extension] = Ext(extension)

def getCurrentSymbolTerms[Symbol](sequence: IndexedSeq[Symbol]): Iterable[Term[At[Symbol]]] =
  sequence.toSet.view.map((symbol) => At(0, symbol))

probability(At(0, 'm'), "mamma mamma mamma mamma".into)
probability(At(0, 'a'), "mamma mamma mamma mamma".into)
probability(At(0, ' '), "mamma mamma mamma mamma".into)

probabilityOfCurrentSymbol("mamma mamma mamma mamma")
probabilityOfCurrentSymbol("mamma mamma mamma mamma").values.sum

probabilityBy(At(0, 'm'), At(-1, 'm'), "mamma mamma mamma mamma".into)
probabilityBy(At(0, 'a'), At(-1, 'm'), "mamma mamma mamma mamma".into)
probabilityBy(At(0, ' '), At(-1, 'm'), "mamma mamma mamma mamma".into)
probabilityBy(At(0, 'm'), At(-1, 'a'), "mamma mamma mamma mamma".into)
probabilityBy(At(0, 'a'), At(-1, 'a'), "mamma mamma mamma mamma".into)
probabilityBy(At(0, ' '), At(-1, 'a'), "mamma mamma mamma mamma".into)
probabilityBy(At(0, 'm'), At(-1, ' '), "mamma mamma mamma mamma".into)
probabilityBy(At(0, 'a'), At(-1, ' '), "mamma mamma mamma mamma".into)
probabilityBy(At(0, ' '), At(-1, ' '), "mamma mamma mamma mamma".into)

probabilityOfCurrentSymbolBy("mamma mamma mamma mamma", Ext(At(-1, 'm')))
probabilityOfCurrentSymbolBy("mamma mamma mamma mamma", Ext(At(-1, 'm'))).values.sum
probabilityOfCurrentSymbolBy("mamma mamma mamma mamma", Ext(At(-2, 'm')))
probabilityOfCurrentSymbolBy("mamma mamma mamma mamma", Ext(At(-2, 'm'))).values.sum
probabilityOfCurrentSymbolBy("mamma mamma mamma mamma", And(At(-2, 'm'), At(-1, 'm')))
probabilityOfCurrentSymbolBy("mamma mamma mamma mamma", And(At(-2, 'm'), At(-1, 'm'))).values.sum
probabilityOfCurrentSymbolBy("mamma mamma mamma mamma", Not(At(-1, 'm')))
probabilityOfCurrentSymbolBy("mamma mamma mamma mamma", Not(At(-1, 'a')))

extension [Item](iterator: Iterator[Item]) def withPrevious: Iterator[(Option[Item], Item)] =
  var last: Option[Item] = None
  iterator.map(item => {
    val result = (last, item)
    last = Some(item)
    result
  })

val bigData = scala.io.Source.fromResource("il-piccolo-principe.txt").simplifyItalian.mkString

probabilityOfCurrentSymbol(bigData).toVector.sortBy(_._2).reverse
probabilityOfCurrentSymbolBy(bigData, Ext(At(-1, 'm'))).toVector.sortBy(_._2).reverse
probabilityOfCurrentSymbolBy(bigData, And(At(-2, 'p'), At(-1, 'r'))).toVector.sortBy(_._2).reverse

def atTerms[Symbol](relativeIndex: Int, symbols: Iterable[Symbol]): Iterable[Term[At[Symbol]]] =
  symbols.view.map((symbol) => Ext(At(relativeIndex, symbol)))

atTerms(0, bigData.toSet).toVector
atTerms(-1, bigData.toSet).toVector
atTerms(-2, bigData.toSet).toVector

atTerms(0, bigData.toSet).toVector.map(term => (term, probability(term, bigData.into))).toMap

def haha[Term, Context](left: Term, right: Term, contexts: Iterable[Context])(using Checkable[Term, Context]) =
  var leftIsTrueCount = 0;
  var leftIsFalseCount = 0;
  var rightIsTrueCount = 0;
  var rightIsFalseCount = 0;
  var whenLeftIsTrueRightIsTrueCount = 0;
  var whenLeftIsTrueRightIsFalseCount = 0;
  var whenLeftIsFalseRightIsTrueCount = 0;
  var whenLeftIsFalseRightIsFalseCount = 0;
  var whenRightIsTrueLeftIsTrueCount = 0;
  var whenRightIsTrueLeftIsFalseCount = 0;
  var whenRightIsFalseLeftIsTrueCount = 0;
  var whenRightIsFalseLeftIsFalseCount = 0;
  for {
    context <- contexts
    left <- summon.check(left, context)
    right <- summon.check(right, context)
  } {
    if left then leftIsTrueCount += 1 else leftIsFalseCount += 1
    if right then rightIsTrueCount += 1 else rightIsFalseCount += 1
    if left && right then whenLeftIsTrueRightIsTrueCount += 1
    if left && !right then whenLeftIsTrueRightIsFalseCount += 1
    if !left && right then whenLeftIsFalseRightIsTrueCount += 1
    if !left && !right then whenLeftIsFalseRightIsFalseCount += 1
    if right && left then whenRightIsTrueLeftIsTrueCount += 1
    if right && !left then whenRightIsTrueLeftIsFalseCount += 1
    if !right && left then whenRightIsFalseLeftIsTrueCount += 1
    if !right && !left then whenRightIsFalseLeftIsFalseCount += 1
  }
  var leftRelative = leftIsTrueCount.toFloat / (leftIsTrueCount + leftIsFalseCount).toFloat
  var rightRelative = rightIsTrueCount.toFloat / (rightIsTrueCount + rightIsFalseCount).toFloat
  var leftTrueRelative = whenLeftIsTrueRightIsTrueCount.toFloat / leftIsTrueCount.toFloat
  assert(leftTrueRelative == whenLeftIsTrueRightIsTrueCount.toFloat / (whenLeftIsTrueRightIsTrueCount.toFloat + whenLeftIsTrueRightIsFalseCount.toFloat))
  var leftFalseRelative = whenLeftIsFalseRightIsTrueCount.toFloat / leftIsFalseCount.toFloat
  assert(leftFalseRelative == whenLeftIsFalseRightIsTrueCount.toFloat / (whenLeftIsFalseRightIsTrueCount.toFloat + whenLeftIsFalseRightIsFalseCount.toFloat))
  var rightTrueRelative = whenRightIsTrueLeftIsTrueCount.toFloat / rightIsTrueCount.toFloat
  assert(rightTrueRelative == whenRightIsTrueLeftIsTrueCount.toFloat / (whenRightIsTrueLeftIsTrueCount.toFloat + whenRightIsTrueLeftIsFalseCount.toFloat))
  var rightFalseRelative = whenRightIsFalseLeftIsTrueCount.toFloat / rightIsFalseCount.toFloat
  assert(rightFalseRelative == whenRightIsFalseLeftIsTrueCount.toFloat / (whenRightIsFalseLeftIsTrueCount.toFloat + whenRightIsFalseLeftIsFalseCount.toFloat))
  var oddsRatioA = (leftIsTrueCount.toFloat / leftIsFalseCount.toFloat) / (rightIsTrueCount.toFloat / rightIsFalseCount.toFloat)
  var oddsRatioB = (rightIsTrueCount.toFloat / rightIsFalseCount.toFloat) / (leftIsTrueCount.toFloat / leftIsFalseCount.toFloat)
  Vector(
    "leftIsTrueCount" -> leftIsTrueCount,
    "leftIsFalseCount" -> leftIsFalseCount,
    "leftRelative" -> leftRelative,
    "rightIsTrueCount" -> rightIsTrueCount,
    "rightIsFalseCount" -> rightIsFalseCount,
    "rightRelative" -> rightRelative,
    "whenLeftIsTrueRightIsTrueCount" -> whenLeftIsTrueRightIsTrueCount,
    "whenLeftIsTrueRightIsFalseCount" -> whenLeftIsTrueRightIsFalseCount,
    "leftTrueRelative" -> leftTrueRelative,
    "whenLeftIsFalseRightIsTrueCount" -> whenLeftIsFalseRightIsTrueCount,
    "whenLeftIsFalseRightIsFalseCount" -> whenLeftIsFalseRightIsFalseCount,
    "leftFalseRelative" -> leftFalseRelative,
    "whenRightIsTrueLeftIsTrueCount" -> whenRightIsTrueLeftIsTrueCount,
    "whenRightIsTrueLeftIsFalseCount" -> whenRightIsTrueLeftIsFalseCount,
    "rightTrueRelative" -> rightTrueRelative,
    "whenRightIsFalseLeftIsTrueCount" -> whenRightIsFalseLeftIsTrueCount,
    "whenRightIsFalseLeftIsFalseCount" -> whenRightIsFalseLeftIsFalseCount,
    "rightFalseRelative" -> rightFalseRelative,
    "oddsRatioA" -> oddsRatioA,
    "oddsRatioB" -> oddsRatioB,
    "" -> ("" + math.log(oddsRatioA).toString + " " + math.log(oddsRatioB).toString)
  )

haha(At(-1, 'r'), At(0, 'e'), bigData.into)

haha(At(-1, 'a'), At(0, 'a'), "mamamamamama".into)
haha(At(-1, 'a'), At(0, 'a'), "manamanamana".into)
haha(At(-1, 'm'), At(0, 'a'), "manamanamana".into)
haha(At(-1, 'm'), At(0, 'a'), "mamamamamama".into)
haha(At(-1, 'm'), At(0, 'a'), "manamamamama".into)
haha(At(-1, 'm'), At(0, 'm'), "mamamamamama".into)
haha(At(-1, 'n'), At(0, 'a'), "manamamamama".into)
haha(At(-1, 'm'), At(0, 'a'), "manamamamama".into)


probabilityBy(At(0, 'm'), At(-1, 'm'), "mamma mamma mamma mamma".into)
probabilityBy(At(0, 'a'), At(-1, 'm'), "mamma mamma mamma mamma".into)
probabilityBy(At(0, ' '), At(-1, 'm'), "mamma mamma mamma mamma".into)

probabilityBy(At(0, 'm'), At(-1, 'a'), "mamma mamma mamma mamma".into)
probabilityBy(At(0, 'a'), At(-1, 'a'), "mamma mamma mamma mamma".into)
probabilityBy(At(0, ' '), At(-1, 'a'), "mamma mamma mamma mamma".into)

probabilityBy(At(0, 'm'), At(-2, 'm'), "mamma mamma mamma mamma".into)
probabilityBy(At(0, 'a'), At(-2, 'm'), "mamma mamma mamma mamma".into)
probabilityBy(At(0, ' '), At(-2, 'm'), "mamma mamma mamma mamma".into)

Map('a' -> 0.5) == Map('a' -> 0.5)

/*

c = a
c = !a
c = a & b
c = a | b
c = a = b

true  true  true  | a |    | a & b | a | b | a = b |
true  true  false |   | !a |       |       |       |
true  false true  | a |    |       | a | b |       |
true  false false |   | !a | a & b |       | a = b |
false true  true  |   | !a |       | a | b |       |
false true  false | a |    | a & b |       | a = b |
false false true  |   | !a |       |       | a = b |
false false false | a | !a | a & b | a | b |       |
*/
