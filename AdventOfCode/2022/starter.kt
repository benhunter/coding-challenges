fun dayx() {
    val day = 0
    println("day $day")
    val startTime = System.nanoTime()

//    val filename = "$day-input.txt"
    val filename = "$day-test.txt"
    val text = getTextFromResource(filename).trim()
    val lines = text.split("\n")
    debugln(lines)


    var part1 = 0
    lines.forEach { line ->
        debug("$line ")

        debugln("score $part1 ")
    }
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
//    val timeSeconds = elapsedTimeInSecondsSince(startTime)
//    println("total time: $timeSeconds seconds")
}
