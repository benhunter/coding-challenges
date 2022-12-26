import java.io.BufferedReader
import java.io.InputStreamReader


fun dayx() {
    val day = 0
    println("day $day")

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


//    debugln("part 2")
//
//    var part2 = 0
//    lines.forEach { line ->
//        debug("input: $line ### ")
//
//        debugln("score $part2")
//    }
//    println("part 2 $part2")
}
