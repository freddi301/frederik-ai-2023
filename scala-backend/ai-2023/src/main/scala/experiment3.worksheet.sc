def probabilityOfCurrentCharacter(text: String): Map[Char, Float] =
  val countByCharacter = scala.collection.mutable.HashMap[Char, Int]()
  var total = 0
  for currentIndex <- 0 until text.size
  do
    total += 1
    countByCharacter.updateWith(text(currentIndex))({
      case Some(count) => Some(count + 1)
      case None => Some(1)
    })
  countByCharacter.view.mapValues((count) => count.toFloat / total.toFloat).toMap

probabilityOfCurrentCharacter("mamma mamma mamma mamma")

def probabilityOfCurrentCharacterGivePrevious(text: String, previous: Char): Map[Char, Float] =
  val countByCharacter = scala.collection.mutable.HashMap[Char, Int]()
  var total = 0
  for {
    currentIndex <- 0 until text.size
    if currentIndex > 0 && text(currentIndex - 1) == previous
  } do {
    total += 1
    countByCharacter.updateWith(text(currentIndex))({
      case Some(count) => Some(count + 1)
      case None => Some(1)
    })
  }
  countByCharacter.view.mapValues((count) => count.toFloat / total.toFloat).toMap

probabilityOfCurrentCharacterGivePrevious("mamma mamma mamma mamma", ' ')
probabilityOfCurrentCharacterGivePrevious("mamma mamma mamma mamma", 'a')
probabilityOfCurrentCharacterGivePrevious("mamma mamma mamma mamma", 'm')

def probabilityOfCurrentCharacterGivenOthers(text: String, other: Map[Int, Char]): Map[Char, Float] =
  val countByCharacter = scala.collection.mutable.HashMap[Char, Int]()
  var total = 0
  for {
    currentIndex <- 0 until text.size
    currentCharacter = text(currentIndex)
  } do {
    if other.forall((otherRelativeIndex, otherCharacter) => {
        val otherAbsoluteIndex = currentIndex + otherRelativeIndex
        val isObserved = otherAbsoluteIndex >= 0 && otherAbsoluteIndex < text.size
        if isObserved
        then text(otherAbsoluteIndex) == otherCharacter
        else false
      })
    then {
      total += 1
      countByCharacter.updateWith(currentCharacter)(inc)
    }
  }
  countByCharacter.view.mapValues((count) => count.toFloat / total.toFloat).toMap
def inc(value: Option[Int]) = value match {
  case Some(count) => Some(count + 1)
  case None => Some(1)
}

probabilityOfCurrentCharacterGivenOthers("mamma mamma mamma mamma", Map())
probabilityOfCurrentCharacterGivenOthers("mamma mamma mamma mamma", Map(-1 -> ' '))
probabilityOfCurrentCharacterGivenOthers("mamma mamma mamma mamma", Map(-1 -> 'a'))
probabilityOfCurrentCharacterGivenOthers("mamma mamma mamma mamma", Map(-1 -> 'm'))
probabilityOfCurrentCharacterGivenOthers("mamma mamma mamma mamma", Map(-2 -> 'm', -1 -> 'a'))
probabilityOfCurrentCharacterGivenOthers("mamma mamma mamma mamma", Map(-2 -> 'm', -1 -> 'm'))
probabilityOfCurrentCharacterGivenOthers("mamma mamma mamma mamma", Map(-2 -> 'a', -1 -> 'm'))

Seq(1, 2, 3).forall(_ => true)
Seq().forall(_ => false)
