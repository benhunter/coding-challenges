fun day6() {
    println("day 6")

    val filename = "6-input.txt"
//    val filename = "6-test.txt"
    val text = getTextFromResource(filename).trim()
    val lines = text.split("\n")
    debugln(lines.toString())


    var part1 = 0
    lines.forEach { line ->
        debug("$line ")

        for (i in 4 until line.length) {
            debug("$i ")

            if (isUniqueChars(line.slice(i - 4 until i), 4)) {
                debugln("")
                part1 = i
                break
            }
        }
        println("part 1 $part1") // 1275
    }


    debugln("part 2")
    var part2 = 0
    val distinctPart2 = 14
    lines.forEach { line ->
        debug("$line ")

        for (i in distinctPart2 until line.length) {
            debug("$i ")

            if (isUniqueChars(line.slice(i - distinctPart2 until i), distinctPart2)) {
                debugln("")
                part2 = i
                break
            }
        }
        println("part 2 $part2") // 3605
    }
}
