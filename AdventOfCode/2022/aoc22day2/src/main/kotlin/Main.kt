import java.io.BufferedReader
import java.io.InputStreamReader

fun main(args: Array<String>) {

    val filename = "input.txt"
//    val filename = "test.txt"
    val resourceStream = ClassLoader.getSystemResourceAsStream(filename)
    val reader = BufferedReader(InputStreamReader(resourceStream))
    val text = reader.readText().trim()
    resourceStream.close()

    val lines = text.split("\n")
//    println(lines)

    val leftMap = mapOf("A" to 1, "B" to 2, "C" to 3)
    val rightMap = mapOf("X" to 1, "Y" to 2, "Z" to 3)

    var part1 = 0
    lines.forEach {
        print("$it ")
        val round = it.split(" ")
        val leftScore = leftMap[round[0]] ?: throw Exception("oof")
        print("$leftScore ")
        val rightScore = rightMap[round[1]] ?: throw Exception("oof")
        print("$rightScore ")

        val winScore = when {
            leftScore == 1 && rightScore == 3 -> 0
            leftScore == 3 && rightScore == 1 -> 6
            leftScore > rightScore -> 0
            leftScore == rightScore -> 3
            leftScore < rightScore -> 6
            else -> 6
        }
        print("winScore $winScore ")
        val roundScore = rightScore + winScore
        print("roundScore $roundScore ")
        part1 += roundScore

        println("score $part1 ")
    }
    println("part 1 $part1")
    // 14531

    val outcomeMap = mapOf("X" to "lose", "Y" to "draw", "Z" to "win")

    var part2 = 0
    lines.forEach {
        print("$it ")
        val round = it.split(" ")
        val leftScore = leftMap[round[0]] ?: throw Exception("oof")
        print("$leftScore ")

        val outcome = outcomeMap[round[1]]
        print("$outcome ")
        val winScore = rightMap[round[1]] ?: throw Exception("oof")
        val rightScore = when (outcome) {
            "win"-> (leftScore + 1).mod(3)
            "draw"-> leftScore
            "lose"-> (leftScore - 1).mod(3)
            else -> throw Exception("wtf")
        }
        print("rightScore $rightScore ")

        val roundScore = rightScore + winScore
        print("roundScore $roundScore ")
        part2+= roundScore

        println("score $part2")
    }
    println("part 2 $part2")
    // 8383 too low
}
