fun day7() {
    val day = 7
    println("day $day")

    val filename = "$day-input.txt"
//    val filename = "$day-test.txt"
    val text = getTextFromResource(filename).trim()
    val lines = text.split("\n")
    debugln(lines.toString())

    // build filesystem

    var part1 = 0

    data class FilesystemObject(val name: String, val size: Int = 0)

    val filesystemSize = mutableMapOf("/" to 0)
    val filesystem = mutableMapOf("/" to mutableListOf<String>())
    var currentDir = "/"
    lines.forEach { line ->
        debugln("command: $line ")
        debugln("currentDir $currentDir")
        debugln("filesystem $filesystem")
        debugln("filesystemSize $filesystemSize")
        val tokens = line.split(" ")
//        tokens.forEach { debugln(it) }

        when (tokens[0]) {
            "$" -> when (tokens[1]) {
                "cd" -> {
                    currentDir = when (tokens[2]) {
                        "/" -> "/"

                        ".." -> {
                            val path = currentDir
                                .split("/")
                                .toMutableList()
                            path.removeLast()
                            path.removeLast()
                            path.joinToString("/") + "/"
                        }

                        else -> {
                            currentDir + "${tokens[2]}/"
                            // TODO must already be in filesystem?
                        }
                    }
                    debugln("currentDir $currentDir")
                }

                "ls" -> return@forEach
                else -> TODO("not implemented yet $tokens")
            }

            "dir" -> {
                val child = "$currentDir${tokens[1]}/"
                filesystemSize[child] = 0
                filesystem[child] = mutableListOf()

                // append child to parent's child list
                filesystem.putIfAbsent(currentDir, mutableListOf())
                filesystem[currentDir]?.add(child)
            }

            else -> {
                val size = tokens[0].toInt()
                val child = "$currentDir${tokens[1]}"
                filesystemSize[child] = size
                filesystem[child] = mutableListOf()

                // append child to parent's child list
                filesystem.putIfAbsent(currentDir, mutableListOf())
                filesystem[currentDir]?.add(child)
            }
        }
    }

    debugln("filesystem $filesystem")
    debugln("filesystemSize $filesystemSize")

    // calculate all dir size
    val needToCalcSize = filesystem.keys.toMutableList()
    while (needToCalcSize.isNotEmpty()) {
        filesystem.forEach { (current, children) ->
            if (current !in needToCalcSize) return@forEach
            debugln(current)
            debugln(children)

            if (children.isEmpty()) {
                debugln("removing $current")
                needToCalcSize.remove(current)
                return@forEach
            }

            val childrenNeedToCalcSize = children.filter { it in needToCalcSize }
            if (childrenNeedToCalcSize.isEmpty()) {
                var sum = 0
                children.forEach { sum += filesystemSize[it] ?: throw Exception("$it not found") }
                filesystemSize[current] = sum
                needToCalcSize.remove(current)
            }
        }
    }
    debugln("filesystemSize $filesystemSize")

    // part 1: sum all dirs at most 100,000
    // dirs are objects that have children
    part1 = filesystemSize.filter {
        filesystem[it.key]?.isNotEmpty() ?: throw Exception("$it not found")
    }
        .filter { it.value <= 100000 }
        .values
        .sum()

    println("part 1 $part1") // 1454188


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
