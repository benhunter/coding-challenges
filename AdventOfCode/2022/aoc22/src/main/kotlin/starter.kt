class DayX {
    init {
        val day = 0
        val testFilename = "$day.test"
        val test = getTrimmedLinesFromResource(testFilename)
        val inputFilename = "$day.input"
        val input = getTrimmedLinesFromResource(inputFilename)

        val startTime = System.nanoTime()
        val part1Test = part1(test)
        println("day $day part 1 test $part1Test")
        println("day $day part 1 test time: ${elapsedTimeInSecondsSince(startTime)} seconds")

//        val part1StartTime = System.nanoTime()
//        val part1 = part1(input)
//        println("day $day part 1 $part1")
//        println("day $day part 1 time: ${elapsedTimeInSecondsSince(part1StartTime)} seconds")

//        val part2StartTime = System.nanoTime()
//        val part2 = part2(input)
//        println("day $day part 2 $part2")
//        println("day $day part 2 time: ${elapsedTimeInSecondsSince(part2StartTime)} seconds")
//        println("day $day total time: ${elapsedTimeInSecondsSince(startTime)} seconds")
    }

    fun part1(input: List<String>): Int {
        val score = 0
        input.forEach { line ->
            debug("$line ")
            debugln()
        }
        debugln("score $score")
        return score
    }

    fun part2(input: List<String>): Int {
        return part1(input)
    }
}
