fun day19() {
    val day = 19
    println("day $day")

    val startTime = System.nanoTime()

//    val filename = "$day-input.txt"
    val filename = "$day-test.txt"
    val text = getTextFromResource(filename).trim()
    val blueprints = text.split("\n")
//    debugln(blueprints)

    val minutes = 15 // 24
    // 15 = 70 seconds with maps

    var part1 = 0
    blueprints.forEach { line ->
        debugln(line)

        val blueprint = line.split(" ")
        val orebot_ore = blueprint[6].toInt()
        val claybot_ore = blueprint[12].toInt()
        val obsidianbot_ore = blueprint[18].toInt()
        val obsidianbot_clay = blueprint[21].toInt()
        val geodebot_ore = blueprint[27].toInt()
        val geodebot_obsidian = blueprint[30].toInt()

//        data class Rock(var ore: Int = 0, var clay: Int = 0, var obsidian: Int = 0, var geode: Int = 0) // optimization test vs maps...

        val emptyBots = mapOf("ore" to 0, "clay" to 0, "obsidian" to 0, "geode" to 0)
        val startRobots = mapOf("ore" to 1, "clay" to 0, "obsidian" to 0, "geode" to 0).toMutableMap()
        val startResources = mapOf("ore" to 0, "clay" to 0, "obsidian" to 0, "geode" to 0).toMutableMap()

        data class State(
            val minute: Int,
            val robots: MutableMap<String, Int>,
            val resources: MutableMap<String, Int>,
            val action: Action
        )

        val startState = State(1, startRobots, startResources, Action.Mine)
        val nodes = mutableListOf<State>(startState)
        val leafNodes = mutableListOf<State>()
        var maxGeodes = 0

        var count = 1
        var current = nodes[0]
        while (nodes.isNotEmpty()) {
            current = nodes.removeFirst()
            count += 1

            // do current
            // take resources
            when (current.action) {
                Action.Mine -> {}
                Action.BuildOre -> {
                    if (current.resources["ore"]!! >= orebot_ore) {
//                        debug("Spending $orebot_ore ore on 1 ore-collecting robot. ")
                        current.resources["ore"] = current.resources["ore"]!! - orebot_ore
                    } else continue
                }

                Action.BuildClay -> {
                    if (current.resources["ore"]!! >= claybot_ore) {
//                        debug("Spending $claybot_ore ore on 1 clay-collecting robot. ")
                        current.resources["ore"] = current.resources["ore"]!! - claybot_ore
                    } else continue
                }

                Action.BuildObsidian -> {
                    if (current.resources["ore"]!! >= obsidianbot_ore && current.resources["clay"]!! >= obsidianbot_clay) {
//                        debug("Spending $obsidianbot_ore ore and $obsidianbot_clay clay on 1 obsidian-collecting robot. ")
                        current.resources["ore"] = current.resources["ore"]!! - obsidianbot_ore
                        current.resources["clay"] = current.resources["clay"]!! - obsidianbot_clay
                    } else continue
                }

                Action.BuildGeode -> {
                    if (current.resources["ore"]!! >= geodebot_ore && current.resources["obsidian"]!! >= geodebot_obsidian) {
//                        debug("Spending $geodebot_ore ore and $geodebot_obsidian obsidian on 1 geode-collecting robot. ")
                        current.resources["ore"] = current.resources["ore"]!! - geodebot_ore
                        current.resources["obsidian"] = current.resources["obsidian"]!! - geodebot_obsidian

                    } else continue
                }
            }
            // mine
            current.resources["ore"] = current.resources["ore"]!! + current.robots["ore"]!!
//            debug("ore ${current.resources["ore"]}. ")
            current.resources["clay"] = current.resources["clay"]!! + current.robots["clay"]!!
//            debug("clay ${current.resources["clay"]}. ")
            current.resources["obsidian"] = current.resources["obsidian"]!! + current.robots["obsidian"]!!
//            debug("obsidian ${current.resources["obsidian"]}. ")
            current.resources["geode"] = current.resources["geode"]!! + current.robots["geode"]!!
//            debug("geode ${current.resources["geode"]}. ")

            // add robot
            when (current.action) {
                Action.Mine -> {}
                Action.BuildOre -> current.robots["ore"] = current.robots["ore"]!! + 1
                Action.BuildClay -> current.robots["clay"] = current.robots["clay"]!! + 1
                Action.BuildObsidian -> current.robots["obsidian"] = current.robots["obsidian"]!! + 1
                Action.BuildGeode -> current.robots["geode"] = current.robots["geode"]!! + 1
            }

            // add branches
            if (current.minute < minutes) {
                nodes.add(
                    State(
                        current.minute + 1,
                        current.robots.toMutableMap(),
                        current.resources.toMutableMap(),
                        Action.Mine
                    )
                )
                nodes.add(
                    State(
                        current.minute + 1,
                        current.robots.toMutableMap(),
                        current.resources.toMutableMap(),
                        Action.BuildOre
                    )
                )
                nodes.add(
                    State(
                        current.minute + 1,
                        current.robots.toMutableMap(),
                        current.resources.toMutableMap(),
                        Action.BuildClay
                    )
                )
                nodes.add(
                    State(
                        current.minute + 1,
                        current.robots.toMutableMap(),
                        current.resources.toMutableMap(),
                        Action.BuildObsidian
                    )
                )
                nodes.add(
                    State(
                        current.minute + 1,
                        current.robots.toMutableMap(),
                        current.resources.toMutableMap(),
                        Action.BuildGeode
                    )
                )
            } else {
                leafNodes.add(current)

                if (current.resources["geode"]!! > maxGeodes) {
                    maxGeodes = current.resources["geode"]!!
                    debugln("maxGeodes $maxGeodes")
                }

            }

//            debugln()
        }

        val blueprintMaxGeodes = leafNodes.maxOf { it.resources["geode"]!! }
        debugln("blueprintMaxGeodes $blueprintMaxGeodes")
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
