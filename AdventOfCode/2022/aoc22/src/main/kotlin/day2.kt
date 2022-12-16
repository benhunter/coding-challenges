import java.io.BufferedReader
import java.io.InputStreamReader

val DEBUG = false

fun day2() {
    println("day 2")

    val filename = "2-input.txt"
//    val filename = "2-test.txt"
    val resourceStream = ClassLoader.getSystemResourceAsStream(filename)
    val reader = BufferedReader(InputStreamReader(resourceStream))
    val text = reader.readText().trim()
    resourceStream.close()

    val lines = text.split("\n")
//    debugln(lines)

    val leftMap = mapOf("A" to 1, "B" to 2, "C" to 3)
    val rightMap = mapOf("X" to 1, "Y" to 2, "Z" to 3)
    val playToScoreMap = mapOf("rock" to 1, "paper" to 2, "scissors" to 3)

    var part1 = 0
    lines.forEach {
        debug("$it ")
        val round = it.split(" ")
        val leftScore = leftMap[round[0]] ?: throw Exception("oof")
        debug("$leftScore ")
        val rightScore = rightMap[round[1]] ?: throw Exception("oof")
        debug("$rightScore ")

        val winScore = when {
            leftScore == 1 && rightScore == 3 -> 0
            leftScore == 3 && rightScore == 1 -> 6
            leftScore > rightScore -> 0
            leftScore == rightScore -> 3
            leftScore < rightScore -> 6
            else -> 6
        }
        debug("winScore $winScore ")
        val roundScore = rightScore + winScore
        debug("roundScore $roundScore ")
        part1 += roundScore

        debugln("score $part1 ")
    }
    println("part 1 $part1")
//     14531

    val leftMapWord = mapOf("A" to "rock", "B" to "paper", "C" to "scissors")
    val outcomeMap = mapOf("X" to "lose", "Y" to "draw", "Z" to "win")
    val outcomeScoreMap = mapOf("lose" to 0, "draw" to 3, "win" to 6)

    debugln("part 2")

    var part2 = 0
    lines.forEach {
        debug("input: $it ### ")
        val round = it.split(" ")
        val leftPlay = leftMapWord[round[0]] ?: throw Exception("oof")
        debug("$leftPlay ")

        val outcome = outcomeMap[round[1]]
        debug("$outcome ### ")


        val rightPlay = when (outcome) {
            "draw" -> leftPlay
            "lose" -> when (leftPlay) {
                "rock" -> "scissors"
                "paper" -> "rock"
                "scissors" -> "paper"
                else -> TODO("need outcome $outcome for leftPlay $leftPlay")
            }

            "win" -> when (leftPlay) {
                "scissors" -> "rock"
                "paper" -> "scissors"
                "rock" -> "paper"
                else -> TODO("need outcome $outcome for leftPlay $leftPlay")
            }

            else -> TODO("need outcome $outcome")
        }
        debug("rightPlay $rightPlay, ")

        val rightScore = playToScoreMap[rightPlay] ?: throw Exception("oof")
        debug("rightScore $rightScore, ")

        val winScore = outcomeScoreMap[outcome] ?: throw Exception("oof")
        debug("winScore $winScore, ")
        val roundScore = rightScore + winScore
        debug("roundScore $roundScore, ")

        part2 += roundScore

        debugln("score $part2")
    }
    println("part 2 $part2")
    // 11258
}

fun debug(out: String) {
    if (DEBUG) print(out)
}

fun debugln(out: String) {
    if (DEBUG) println(out)
}
