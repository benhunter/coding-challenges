import java.io.BufferedReader
import java.io.InputStreamReader

fun day1() {
    println("day 1")

//    val filename = "1-test.txt"
    val filename = "1-input.txt"
    val resourceStream = ClassLoader.getSystemResourceAsStream(filename)
    val reader = BufferedReader(InputStreamReader(resourceStream))

    // option 1
//    var line = reader.readLine()
//    println("$line")
//    while(line.isNotEmpty()){
//        line = reader.readLine()
//        println("$line")
//    }

    // option 2
//    val text = reader.readLines()

    val text = reader.readText()
    println(text)

    resourceStream.close()
    reader.close()

    var groups = text
        .split("\n\n")
    println(groups)

    val intGroups = groups
        .map {
            println("groups $it")
            it.split("\n")
                .filter { it.isNotEmpty() }
                .map {
                    println("line $it")
                    it.toInt()
                }
        }
    println(intGroups)

    val totals = intGroups.map { it.sum() }
    println(totals)

    val part1 = totals.max()
    println("part1 $part1")

    val part2 = totals.sortedDescending().subList(0, 3).sum()
    println("part2 $part2")
}
