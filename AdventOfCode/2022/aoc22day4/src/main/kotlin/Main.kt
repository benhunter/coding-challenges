import java.io.BufferedReader
import java.io.InputStreamReader

val DEBUG = true

fun main(args: Array<String>) {

    val filename = "input.txt"
//    val filename = "test.txt"
    val resourceStream = ClassLoader.getSystemResourceAsStream(filename)
    val reader = BufferedReader(InputStreamReader(resourceStream))
    val text = reader.readText().trim()
    resourceStream.close()

    val lines = text.split("\n")
//    debugln(lines)


    var part1 = 0
    lines.forEach { line ->
//        debug("$it ")
        val pairs = line.split(",")
        debug("$pairs ")

        val ranges = pairs.map { it.split('-').map { it.toInt() }}
        debug("ranges $ranges ")

        if (within(ranges[0], ranges[1])) part1 += 1
        else if (within(ranges[1], ranges[0])) part1 += 1

        debugln("score $part1 ")
    }
    println("part 1 $part1")
    // 580


//    debugln("part 2")
//
//    var part2 = 0
//    lines.forEach {
//        debug("input: $it ### ")
//        debugln("score $part2")
//    }
//    println("part 2 $part2")
}

fun within(range1: List<Int>, range2: List<Int>): Boolean {
    // example: 3,4 and 2,5
    if (range1[0] >= range2[0] && range1[1] <= range2[1]) {
        return true
    }
    return false
}


fun debug(out: String) {
    if (DEBUG) print(out)
}

fun debugln(out: String) {
    if (DEBUG) println(out)
}
