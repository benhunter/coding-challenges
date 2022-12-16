fun day4() {
    println("day 4")

    val filename = "4-input.txt"
//    val filename = "4-test.txt"
    val text = getTextFromResource(filename).trim()
    val lines = text.split("\n")

    var part1 = 0
    lines.forEach { line ->
//        debug("$it ")
        val pairs = line.split(",")
        debug("$pairs ")

        val ranges = pairs.map { it.split('-').map { it.toInt() } }
        debug("ranges $ranges ")

        if (within(ranges[0], ranges[1])) part1 += 1
        else if (within(ranges[1], ranges[0])) part1 += 1

        debugln("score $part1 ")
    }
    println("part 1 $part1")
    // 580


    debugln("part 2")

    var part2 = 0
    lines.forEach { line ->
//        debug("input: $line ### ")

        val pairs = line.split(",")
        debug("$pairs ")

        val ranges = pairs.map { it.split('-').map { it.toInt() } }
        debug("ranges $ranges ")

        if (overlap(ranges[0], ranges[1])) part2 += 1
        else if (overlap(ranges[1], ranges[0])) part2 += 1

        debugln("score $part2")
    }
    println("part 2 $part2")
    // 895
}
