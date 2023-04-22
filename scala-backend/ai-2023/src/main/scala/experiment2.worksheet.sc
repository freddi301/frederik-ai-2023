// val nodes = 100
// val weights = scala.collection.mutable.ArraySeq.fill(nodes * nodes)(0.0f)
// val charges = scala.collection.mutable.ArraySeq.fill(nodes)(0.0f)

def weight(weights: Vector[Float], from: Int, to: Int): Float = weights(from * math.sqrt(weights.size).toInt + to)

def charge(charges: Vector[Float], node: Int): Float = charges(node)

def next(weights: Vector[Float], charges: Vector[Float]): Vector[Float] =
  (0 until charges.size).flatMap(to => (0 until charges.size).map(from => charge(charges, from) * weight(weights, from, to))).toVector

def steps(weights: Vector[Float], charges: Vector[Float], steps: Int): Vector[Float] =
  (1 to steps)
    .scanLeft(charges)((charges, _) => next(weights, charges))
    .reduceLeft((summed, charges) => summed.zip(charges).map((s, c) => s + c))
