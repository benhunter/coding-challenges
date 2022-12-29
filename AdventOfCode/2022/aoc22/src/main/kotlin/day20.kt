fun day20() {
    val day = 20
    println("day $day")
    val startTime = System.nanoTime()

//    val filename = "$day-input.txt"
    val filename = "$day-test.txt"
    val text = getTextFromResource(filename).trim()
    val lines = text.split("\n").map { it.toInt() }
    debugln(lines)

    val decrypted = lines.toMutableList()

    var part1 = 0
    lines.forEachIndexed() { index, line ->
        debug("index $index. line $line. ")

        val currentPosition = decrypted.indexOf(line)
        debug("currentPosition $currentPosition. ")
        var newPosition = (currentPosition + line) % lines.size
        if (newPosition < 0) newPosition += lines.size - 1
        debug("newPosition $newPosition. ")
        decrypted.add(newPosition + 1, line)
        decrypted.remove(line)

        debug("decrypted $decrypted. ")

        debugln()
    }
    debugln()
    println("part 1 $part1")

    val timeSeconds = elapsedTimeInSecondsSince(startTime)
    println("part 1 time: $timeSeconds seconds")


//    val part2StartTime = System.nanoTime()
//    debugln("part 2")
//
//    var part2 = 0
//    lines.forEach { line ->
//        debug("input: $line ### ")
//
//        debugln("score $part2")
//    }
//    println("part 2 $part2")
//
//    val part2timeSeconds = elapsedTimeInSecondsSince(part2StartTime)
//    println("part 2 time: $part2timeSeconds seconds")
//    val timeSeconds = elapsedTimeInSecondsSince(startTimeBlueprint)
//    println("total time: $timeSeconds seconds")
}
