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
    lines.forEach {
//        debugln("$it ")
        val left = it.slice(0..it.length / 2 - 1)
        debug("$left ")
        val right = it.slice(it.length / 2..it.length - 1)
        debug("$right, ")

        var appearsInBoth = ' '
        left.forEach { if (it in right) appearsInBoth = it }
        debug("appearsInBoth $appearsInBoth, ")

        var priority = priorityOf(appearsInBoth)
        debug("priority $priority")

        part1 += priority
        debugln("")
    }
    println("part 1 $part1")


    debugln("part 2")

    var part2 = 0
    lines.chunked(3).forEach { group->
        var priority = 0
        group[0].forEach { if (it in group[1] && it in group[2])  priority = priorityOf(it)}
        debug("$priority ")
        part2 += priority
        debugln("score $part2")
    }
    println("part 2 $part2")
}

fun priorityOf(char: Char): Int {
    val priorityMap = mapOf<Char, Int>().toMutableMap()
    ('a'..'z').forEachIndexed { index, c -> priorityMap[c] = index + 1 }
    ('A'..'Z').forEachIndexed { index, c -> priorityMap[c] = index + 27 }

    return priorityMap[char] ?: throw Exception("oof")
}


fun debug(out: String) {
    if (DEBUG) print(out)
}

fun debugln(out: String) {
    if (DEBUG) println(out)
}
