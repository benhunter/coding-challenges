fun day8() {
    val day = 8
    println("day $day")

    val filename = "$day-input.txt"
//    val filename = "$day-test.txt"
    val text = getTextFromResource(filename).trim()
    val lines = text.split("\n")
    debugln(lines)

    val trees = lines.map { line -> line.map { it.digitToInt() } }
    debugln(trees)

    var part1 = 0
    trees.forEachIndexed { lineIndex, line ->
        debug("$line ")
        line.forEachIndexed { treeIndex, tree ->
            debugln(tree)
            part1 += if (Direction.values()
                    .map { direction -> isVisibleFromOutside(direction, lineIndex, treeIndex, trees) }
                    .any { it }
            ) 1 else 0
            debugln("part 1 $part1")
        }
    }
    debugln()
    println("part 1 $part1") // 1851


    debugln("part 2")

    val scenicScores = trees.mapIndexed { lineindex, line ->
        List(line.size) { treeindex ->
            scenicScore(lineindex, treeindex, trees)
        }
    }
    val part2 = scenicScores.flatten().max()
    println("part 2 $part2") // 574080
}

fun scenicScore(lineindex: Int, treeindex: Int, trees: List<List<Int>>): Int {
    return Direction.values().map { direction -> countVisibleFrom(direction, lineindex, treeindex, trees) }
        .reduce { acc, i -> acc * i }
}

fun countVisibleFrom(direction: Direction, lineIndex: Int, treeIndex: Int, trees: List<List<Int>>): Int {
    val tree = trees[lineIndex][treeIndex]

    val lineOfSight = when (direction) {
        Direction.LEFT -> {
            trees[lineIndex].slice(0 until treeIndex).reversed()
        }

        Direction.RIGHT -> {
            trees[lineIndex].slice(treeIndex + 1 until trees[lineIndex].size)
        }

        Direction.TOP -> {
            trees.mapIndexedNotNull { index, line ->
                if (index < lineIndex) {
                    line[treeIndex]
                } else {
                    null
                }
            }.reversed()
        }

        Direction.BOTTOM -> {
            trees.mapIndexedNotNull { index, line ->
                if (index > lineIndex) {
                    line[treeIndex]
                } else {
                    null
                }
            }
        }
    }
    return calculateLineOfSight(lineOfSight, tree)
}

private fun calculateLineOfSight(lineOfSight: List<Int>, tree: Int): Int {
    lineOfSight.forEachIndexed { index, current ->
        if (current >= tree) return index + 1
    }
    return lineOfSight.size
}

enum class Direction {
    LEFT, RIGHT, TOP, BOTTOM
}

fun isVisibleFromOutside(direction: Direction, lineIndex: Int, treeIndex: Int, trees: List<List<Int>>): Boolean {
    debug("$direction $lineIndex $treeIndex")
    val tree = trees[lineIndex][treeIndex]
    debugln(" tree $tree")
    when (direction) {
        Direction.LEFT -> {
            if (lineIndex == 0) return true
            val lineOfSight = trees[lineIndex].slice(0 until treeIndex)

            return if (lineOfSight.isEmpty())
                true
            else {
                tree > lineOfSight.max()
            }
        }

        Direction.RIGHT -> {
            val lineOfSight = trees[lineIndex].slice(treeIndex + 1 until trees[lineIndex].size)

            return if (lineOfSight.isEmpty())
                true
            else {
                tree > lineOfSight.max()
            }
        }

        Direction.TOP -> {
            if (lineIndex == 0) return true
            val lineOfSight = trees.mapIndexedNotNull { index, line ->
                if (index < lineIndex) {
                    line[treeIndex]
                } else {
                    null
                }
            }
            debugln(lineOfSight)
            return tree > lineOfSight.max()
        }

        Direction.BOTTOM -> {
            if (lineIndex == trees[lineIndex].size - 1) return true
            val lineOfSight = trees.mapIndexedNotNull { index, line ->
                if (index > lineIndex) {
                    line[treeIndex]
                } else {
                    null
                }
            }
            debugln(lineOfSight)
            return tree > lineOfSight.max()
        }
    }
}
