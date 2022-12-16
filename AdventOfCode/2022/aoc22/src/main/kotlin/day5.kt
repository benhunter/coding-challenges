fun day5() {
    println("day 5")

    val filename = "5-input.txt"
//    val filename = "5-test.txt"
    val text = getTextFromResource(filename)

    val input = text.split("\n\n")
    var inputLines = input[0].split("\n").toMutableList().map { line -> line.chunked(4) }
    inputLines = inputLines.slice(0 until inputLines.size - 1)
    val stackHeight = inputLines.size
    val numStacks = inputLines.maxOf { it.size }
    val moves = input[1].split("\n").filter { it.isNotEmpty() }

    inputLines = inputLines.map { line ->
        line.map {
            it.trim()
        }
            .map {
                if (it.isEmpty()) ""
                else it[1].toString()
            }
    }

    val inputStacks = MutableList<MutableList<String>>(numStacks) { MutableList<String>(stackHeight) { "" } }

    inputLines.forEachIndexed { lineIndex, line ->
        line.forEachIndexed { crateIndex, crate ->
            inputStacks[crateIndex][lineIndex] = crate
        }
    }

    inputStacks.forEachIndexed { stackIndex, stack ->
        inputStacks[stackIndex] = stack.reversed().filter { it.isNotEmpty() }.toMutableList()
    }

    val startingStacks = inputStacks.toList()
    var stacks = startingStacks.toMutableList()

    inputLines.forEach { debugln(it.toString()) }
    debugln(numStacks.toString())
    debugln(moves.toString())
    debugStacks(stacks)

    moveCrates(moves, stacks, false)

    var part1 = ""
    stacks.forEach { part1 += it[it.size - 1] }
    println("part 1 $part1") // JCMHLVGMG


    debugln("part 2")
    stacks = startingStacks.toMutableList()

    moveCrates(moves, stacks, true)

    var part2 = ""
    stacks.forEach { part2 += it[it.size - 1] }


    println("part 2 $part2") // LVMRWSSPZ
}

private fun moveCrates(
    moves: List<String>,
    stacks: MutableList<MutableList<String>>,
    moveInOrder: Boolean
) {
    moves.forEach { move ->
        val command = move.split(" ")
        val quantity = command[1].toInt()
        val from = command[3].toInt() - 1
        val to = command[5].toInt() - 1

        val moving = stacks[from].slice(stacks[from].size - quantity until stacks[from].size)
        stacks[from] = stacks[from].slice(0 until stacks[from].size - quantity).toMutableList()
        if (moveInOrder)
            stacks[to].addAll(moving.toMutableList())
        else
            stacks[to].addAll(moving.reversed().toMutableList())


        debugStacks(stacks)
    }
}

fun debugStacks(stacks: MutableList<MutableList<String>>) {
    if (!DEBUG) return
    println("Stacks:")
    val longestStack = stacks.maxOf { it.size }

    for (i in longestStack - 1 downTo 0) {
        for (j in 0 until stacks.size) {
            if (stacks[j].size <= i) print("[ ] ")
            else print("[${stacks[j][i]}] ")
        }
        println()
    }
    println("End of Stacks")
}
