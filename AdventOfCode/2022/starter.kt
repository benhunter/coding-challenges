import java.io.BufferedReader
import java.io.InputStreamReader


fun dayx() {
    println("day x")

//    val filename = "x-input.txt"
    val filename = "x-test.txt"
    val resourceStream = ClassLoader.getSystemResourceAsStream(filename)
    val reader = BufferedReader(InputStreamReader(resourceStream))
    val text = reader.readText().trim()
    resourceStream.close()

    val lines = text.split("\n")
//    debugln(lines)


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
