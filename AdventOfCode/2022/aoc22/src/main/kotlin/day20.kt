fun day20() {

//    val list = List(7) { it }.toMutableList()
//    list.forEach { debugln(it) }
//
//    list.removeAt(2)
//    list.add(4, 2)
//    list.add(7, 7)


    val day = 20
    println("day $day")
    val startTime = System.nanoTime()

    val filename = "$day-input.txt"
//    val filename = "$day-test.txt"
//    val filename = "$day-test2.txt"
    val text = getTextFromResource(filename).trim()
    val lines = text.split("\n").map { it.toInt() }
//    debugln(lines)

    val decrypted = lines.toMutableList()

    val answer = getTextFromResource("20-test-answer.txt").trim().split("\n")
//    val answer = getTextFromResource("20-test2-answer.txt").trim().split("\n")
        .map { line -> line.split(",").map { it.trim().toInt() } }

    var part1 = 0
    lines.forEachIndexed { index, line ->
        debug("$line. ")

        val currentPosition = decrypted.indexOf(line) // TODO fails here because input lines are not unique!
        debug("currentPosition $currentPosition. ")

        var newPosition: Int
        if (line > 0) {
            newPosition = currentPosition + line

            // wrap
            if (newPosition > (lines.size - 1)) {
                newPosition %= lines.size - 1
                if (newPosition == 0) {
                    newPosition = lines.size - 1
                }
            }
            debug("newPosition $newPosition. ")
        } else if (line < 0) {
            newPosition = currentPosition + line

            // wrap
            if (newPosition < 1) {
                newPosition %= lines.size - 1
                newPosition += lines.size - 1
                if (newPosition == 0) {
                    newPosition = lines.size - 1
                }
            }
            debug("newPosition $newPosition. ")
        } else {
            // line == 0
            newPosition = currentPosition
        }

        if (newPosition != currentPosition) {
            decrypted.remove(line)
            decrypted.add(newPosition, line)
        }

        debug("decrypted $decrypted. ")
//        debug("answer: ${answer[index]} ")
//        val match = answer[index] == decrypted
//        debug("match: $match ")
//        if (!match) throw Exception("failed to match answer")
        debugln()
    }
    debugln()
    val posn0 = decrypted.indexOf(0)
    part1 += decrypted[(posn0 + 1000) % decrypted.size]
    part1 += decrypted[(posn0 + 2000) % decrypted.size]
    part1 += decrypted[(posn0 + 3000) % decrypted.size]
    println("part 1 $part1")
    // wrong:
    // -5067
    // 8722 too low

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
