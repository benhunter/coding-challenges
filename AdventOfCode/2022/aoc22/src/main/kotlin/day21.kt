class Day21 {
    init {
        val day = 21
        val testFilename = "$day.test"
        val test = getTrimmedLinesFromResource(testFilename)
        val inputFilename = "$day.input"
        val input = getTrimmedLinesFromResource(inputFilename)

        val startTime = System.nanoTime()
        val part1Test = part1(test)
        println("day $day part 1 test $part1Test")
        if (part1Test != 152.toLong()) throw Exception("wrong")
        println("day $day part 1 test time: ${elapsedTimeInSecondsSince(startTime)} seconds")

        val part1StartTime = System.nanoTime()
        val part1 = part1(input)
        println("day $day part 1 $part1")
        println("day $day part 1 time: ${elapsedTimeInSecondsSince(part1StartTime)} seconds")
        if (part1 <= 571353930) throw Exception("too low")
        if (part1 != 121868120894282) throw Exception("wrong")

        val part2TestStartTime = System.nanoTime()
        val part2Test = part2(test)
        println("day $day part 2 test $part2Test")
        if (part2Test != 301.toLong()) throw Exception("wrong")
        println("day $day part 2 test time: ${elapsedTimeInSecondsSince(part2TestStartTime)} seconds")

        val part2StartTime = System.nanoTime()
        val part2 = part2(input)
        println("day $day part 2 $part2")
        println("day $day part 2 time: ${elapsedTimeInSecondsSince(part2StartTime)} seconds")
        println("day $day total time: ${elapsedTimeInSecondsSince(startTime)} seconds")
    }

    fun part1(input: List<String>): Long {
        val operations = getOperations(input)

        return calc("root", operations)
    }

    fun part2(input: List<String>): Long {
        val operations = getOperations(input)
        val humn = operations["humn"]
        debugln("$humn")
        return calc("root", operations)
    }

    private fun calc(monkey: String, operations: Map<String, String>): Long {
        val operation = operations[monkey]?.split(" ") ?: throw RuntimeException("Monkey $monkey not found")
        if (operation.size == 1) return operation[0].toLong()
        val left = calc(operation[0], operations)
        val right = calc(operation[2], operations)
        val result = when (operation[1]) {
            "+" -> left + right
            "/" -> left / right
            "*" -> left * right
            "-" -> left - right
            else -> TODO("Not yet implemented operator ${operation[1]}")
        }
        return result
    }

    private fun getOperations(input: List<String>) = input.associate {
        val tokens = it.split(":")
        tokens[0] to tokens[1].trim()
    }
}
