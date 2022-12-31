import java.math.BigInteger

data class ValueStartIndexPair(val value: BigInteger, val startIndex: Int)

fun day20() {
    val day = 20
    println("day $day")
    val startTime = System.nanoTime()

    val filename = "$day-input.txt"
    val text = getTextFromResource(filename).trim()
    val lines = text.split("\n").map { it.toLong() }

    val input = lines.mapIndexed { index, i -> ValueStartIndexPair(i.toBigInteger(), index) }
    val decrypted = input.toMutableList()

    input.forEach(rotate(decrypted))

    println("part 1 ${score(decrypted)}") // 19070
    println("part 1 time: ${elapsedTimeInSecondsSince(startTime)} seconds")


    debugln("part 2")
    val part2StartTime = System.nanoTime()

    val magic = 811589153.toBigInteger()
    val keyed = lines.mapIndexed { index, i -> ValueStartIndexPair(i.toBigInteger() * magic, index) }
    val decrypted2 = keyed.toMutableList()

    repeat(10) {
        keyed.toList().forEach(rotate(decrypted2))
    }

    val part2 = score(decrypted2)
    println("part 2 $part2") // 14773357352059
    if (part2 != 14773357352059.toBigInteger()) throw Exception("wrong part 2")
    println("part 2 time: ${elapsedTimeInSecondsSince(part2StartTime)} seconds")
    println("total time: ${elapsedTimeInSecondsSince(startTime)} seconds")
}

fun score(decrypted: List<ValueStartIndexPair>): BigInteger {
    val zero = decrypted.find { it.value == 0.toBigInteger() }
    val posn0 = decrypted.indexOf(zero)
    var sum = decrypted[(posn0 + 1000) % decrypted.size].value
    sum += decrypted[(posn0 + 2000) % decrypted.size].value
    sum += decrypted[(posn0 + 3000) % decrypted.size].value
    return sum
}

private fun rotate(
    decrypted: MutableList<ValueStartIndexPair>,
) = rotateAction@{ pair: ValueStartIndexPair ->
    val currentPosition = decrypted.indexOf(pair).toBigInteger()
    var newPosition: BigInteger

    if (pair.value == 0.toBigInteger()) return@rotateAction

    newPosition = currentPosition + pair.value
    newPosition %= (decrypted.size - 1).toBigInteger()

    if (newPosition < 1.toBigInteger()) {
        newPosition += (decrypted.size - 1).toBigInteger()
    } else if (newPosition == 0.toBigInteger()) {
        newPosition = (decrypted.size - 1).toBigInteger()
    }

    if (newPosition != currentPosition) {
        decrypted.remove(pair)
        decrypted.add(newPosition.toInt(), pair)
    }
}
