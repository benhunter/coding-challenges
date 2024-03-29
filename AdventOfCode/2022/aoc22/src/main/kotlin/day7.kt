fun day7() {
    val day = 7
    println("day $day")

    val filename = "$day-input.txt"
//    val filename = "$day-test.txt"
    val text = getTextFromResource(filename).trim()
    val lines = text.split("\n")
    debugln(lines.toString())


    var part1 = 0

    // build filesystem
    val filesystemSize = mutableMapOf("/" to 0)
    val filesystem = mutableMapOf("/" to mutableListOf<String>())
    var currentDir = "/"
    lines.forEach { line ->
        debugln("command: $line ")
        debugln("currentDir $currentDir")
        debugln("filesystem $filesystem")
        debugln("filesystemSize $filesystemSize")
        val tokens = line.split(" ")

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
                filesystemSize[current] = children.mapNotNull { filesystemSize[it] }.sum()
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
        .filter { it.value <= 100_000 }
        .values
        .sum()

    println("part 1 $part1") // 1454188


    debugln("part 2")

    val DISK_SIZE = 70_000_000
    val NEED_FOR_UPDATE = 30_000_000
    val free = DISK_SIZE - (filesystemSize["/"] ?: throw Exception("/ not found"))
    val needFree = NEED_FOR_UPDATE - free
    val candidates = filesystemSize.filter {
        filesystem[it.key]?.isNotEmpty() ?: throw Exception("$it not found")
    }
        .filter { it.value >= needFree }
        .toList()
        .sortedBy { (_, value) -> value }
    val part2 = candidates[0].second
    debugln(candidates)

    println("part 2 $part2") // 4183246
}
