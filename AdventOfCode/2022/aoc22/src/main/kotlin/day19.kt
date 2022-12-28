fun day19() {
    val day = 19
    println("day $day")

    val startTime = System.nanoTime()

//    val filename = "$day-input.txt"
    val filename = "$day-test.txt"
    val text = getTextFromResource(filename).trim()
    val blueprints = text.split("\n")
//    debugln(blueprints)

    val minutes = 24 // 24
    // 15 = 70 seconds with maps

    var part1 = 0
    blueprints.forEach { line ->
        debugln(line)

        calcMaxGeodes(line)

        val blueprint = line.split(" ")
        val orebot_ore = blueprint[6].toInt()
        val claybot_ore = blueprint[12].toInt()
        val obsidianbot_ore = blueprint[18].toInt()
        val obsidianbot_clay = blueprint[21].toInt()
        val geodebot_ore = blueprint[27].toInt()
        val geodebot_obsidian = blueprint[30].toInt()

        data class Rock(
            var ore: Int = 0,
            var clay: Int = 0,
            var obsidian: Int = 0,
            var geode: Int = 0
        ) // optimization test vs maps...
        {
            operator fun plusAssign(rock: Rock) {
                ore += rock.ore
                clay += rock.clay
                obsidian += rock.obsidian
                geode += rock.geode
            }
        }

        val emptyBots = Rock()
        val startRobots = Rock(1, 0, 0, 0)
        val startResources = Rock()

        data class State(
            val minute: Int,
            val robots: Rock,
            val resources: Rock,
            val action: Action
        )

        val startState = State(1, startRobots, startResources, Action.Mine)
        val nodes = mutableListOf(startState)
//        val leafNodes = mutableListOf<State>()
        var maxGeodes = 0
        var bestNode: State

        var count = 1
        var current = nodes[0]
        while (nodes.isNotEmpty()) {
            current = nodes.removeLast()
            count += 1

            // do current
            // take resources
            when (current.action) {
                Action.Mine -> {}
                Action.BuildOre -> {
                    if (current.resources.ore >= orebot_ore) {
                        current.resources.ore -= orebot_ore
                    } else continue
                }

                Action.BuildClay -> {
                    if (current.resources.ore >= claybot_ore) {
                        current.resources.ore -= claybot_ore
                    } else continue
                }

                Action.BuildObsidian -> {
                    if (current.resources.ore >= obsidianbot_ore && current.resources.clay >= obsidianbot_clay) {
                        current.resources.ore -= obsidianbot_ore
                        current.resources.clay -= obsidianbot_clay
                    } else continue
                }

                Action.BuildGeode -> {
                    if (current.resources.ore >= geodebot_ore && current.resources.obsidian >= geodebot_obsidian) {
                        current.resources.ore -= geodebot_ore
                        current.resources.obsidian -= geodebot_obsidian

                    } else continue
                }
            }

            // mine
            current.resources += current.robots

            // add robot
            when (current.action) {
                Action.Mine -> {}
                Action.BuildOre -> current.robots.ore += 1
                Action.BuildClay -> current.robots.clay += 1
                Action.BuildObsidian -> current.robots.obsidian += 1
                Action.BuildGeode -> current.robots.geode += 1
            }

            // add branches
            if (current.minute < minutes) {

                val minutesRemain = minutes - current.minute

                fun triangle(n: Int): Int {
                    return n * (n + 1) / 2
                }

                // Is it possible to get more than maxGeodes?
                val maxTheoreticalGeodes =
                    current.resources.geode + (current.robots.geode * minutesRemain) + triangle(minutesRemain - 1)
                if (maxTheoreticalGeodes <= maxGeodes) continue

                nodes.add(State(current.minute + 1, current.robots.copy(), current.resources.copy(), Action.Mine))
                nodes.add(State(current.minute + 1, current.robots.copy(), current.resources.copy(), Action.BuildOre))
                nodes.add(State(current.minute + 1, current.robots.copy(), current.resources.copy(), Action.BuildClay))
                nodes.add(
                    State(
                        current.minute + 1,
                        current.robots.copy(),
                        current.resources.copy(),
                        Action.BuildObsidian
                    )
                )
                nodes.add(State(current.minute + 1, current.robots.copy(), current.resources.copy(), Action.BuildGeode))

            } else {
//                leafNodes.add(current)
            }

            if (current.resources.geode > maxGeodes) {
                maxGeodes = current.resources.geode
                bestNode = current
                debugln("maxGeodes $maxGeodes. Best node $bestNode")
            }

//            debugln()
        }

//        val blueprintMaxGeodes = leafNodes.maxOf { it.resources.geode }
//        debugln("blueprintMaxGeodes $blueprintMaxGeodes")
        debugln("maxGeodes $maxGeodes")
        debugln()
    }
    println("part 1 $part1")


//    debugln("part 2")
//
//    var part2 = 0
//    lines.forEach { line ->
//        debug("input: $line ### ")
//
//        debugln("score $part2")
//    }
//    println("part 2 $part2")

    val endTime = System.nanoTime()
    val timeSeconds = (endTime - startTime) / 1_000_000_000.0
    println("Elapsed time: $timeSeconds seconds")
}

enum class Action { Mine, BuildOre, BuildClay, BuildObsidian, BuildGeode }
