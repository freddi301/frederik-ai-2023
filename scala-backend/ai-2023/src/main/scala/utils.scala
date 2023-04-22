package ai.utils

extension (characters: Iterator[Char])
  def simplifyItalian = characters.map(character => {
    character.toLower match
      case 'a' => 'a'
      case 'b' => 'b'
      case 'c' => 'c'
      case 'd' => 'd'
      case 'e' => 'e'
      case 'f' => 'f'
      case 'g' => 'g'
      case 'h' => 'h'
      case 'i' => 'i'
      case 'j' => 'j'
      case 'k' => 'k'
      case 'l' => 'l'
      case 'm' => 'm'
      case 'n' => 'n'
      case 'o' => 'o'
      case 'p' => 'p'
      case 'q' => 'q'
      case 'r' => 'r'
      case 's' => 's'
      case 't' => 't'
      case 'u' => 'u'
      case 'v' => 'v'
      case 'w' => 'w'
      case 'x' => 'x'
      case 'y' => 'y'
      case 'z' => 'z'
      case 'à' => 'a'
      case 'è' => 'e'
      case 'é' => 'e'
      case 'ì' => 'i'
      case 'ò' => 'o'
      case 'ù' => 'u'
      case ' ' => ' '
      case _ => ' '
  }).withPrevious.filter((prev, curr) => !(prev.contains(' ') && curr == ' ')).map(_._2)


extension [Item](iterator: Iterator[Item]) def withPrevious: Iterator[(Option[Item], Item)] =
  var last: Option[Item] = None
  iterator.map(item => {
    val result = (last, item)
    last = Some(item)
    result
  })

