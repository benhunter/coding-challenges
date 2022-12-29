fun day19() {
    val day = 19
    println("day $day")

    val startTime = System.nanoTime()

    val filename = "$day-input.txt"
//    val filename = "$day-test.txt"
    val text = getTextFromResource(filename).trim()
    val blueprints = text.split("\n")

    val minutesPart1 = 24 // 24
    var part1 = 0
    val maxGeodes = blueprints.map { line ->
        debugln(line)

        val blueprint = line.split(" ")
        val orebot_ore = blueprint[6].toInt()
        val claybot_ore = blueprint[12].toInt()
        val obsidianbot_ore = blueprint[18].toInt()
        val obsidianbot_clay = blueprint[21].toInt()
        val geodebot_ore = blueprint[27].toInt()
        val geodebot_obsidian = blueprint[30].toInt()

        val startTimeBlueprint = System.nanoTime()
        val maxGeode =
            calcMaxGeodes(
                orebot_ore,
                claybot_ore,
                obsidianbot_ore,
                obsidianbot_clay,
                geodebot_ore,
                geodebot_obsidian,
                minutesPart1
            )
        val timeSeconds = elapsedTimeInSecondsSince(startTimeBlueprint)
        println("Elapsed time: $timeSeconds seconds")

        return@map maxGeode
    }
    debugln(maxGeodes)
    part1 = maxGeodes.reduceIndexed { index, acc, i -> acc + i * (index + 1) }
    println("part 1 $part1") // 817


    debugln("part 2")

    val minutesPart2 = 32
    var part2 = 0
    val maxGeodes2 = blueprints.subList(0, 3).map { line ->
        debugln(line)

        val blueprint = line.split(" ")
        val orebot_ore = blueprint[6].toInt()
        val claybot_ore = blueprint[12].toInt()
        val obsidianbot_ore = blueprint[18].toInt()
        val obsidianbot_clay = blueprint[21].toInt()
        val geodebot_ore = blueprint[27].toInt()
        val geodebot_obsidian = blueprint[30].toInt()

        val startTimeBlueprint = System.nanoTime()
        val maxGeode = calcMaxGeodes(
            orebot_ore,
            claybot_ore,
            obsidianbot_ore,
            obsidianbot_clay,
            geodebot_ore,
            geodebot_obsidian,
            minutesPart2
        )
        val timeSeconds = elapsedTimeInSecondsSince(startTime)
        println("Elapsed time: $timeSeconds seconds")

        return@map maxGeode
    }
    debugln(maxGeodes2)
    part2 = maxGeodes2.reduce { acc, i -> acc * i }
    println("part 2 $part2") // 4216

    val timeSeconds = elapsedTimeInSecondsSince(startTime)
    println("Elapsed time: $timeSeconds seconds")
}

private fun calcMaxGeodes(
    oreBotOre: Int,
    clayBotOre: Int,
    obsidianBotOre: Int,
    obsidianBotClay: Int,
    geodeBotOre: Int,
    geodeBotObsidian: Int,
    minutes: Int
): Int {

    val startTime = System.nanoTime()
    val startRobots = Rock(1, 0, 0, 0)
    val startResources = Rock()
    val startState = State(1, startRobots, startResources, Action.Mine)
    val nodes = mutableListOf(startState)
    var maxGeodes = 0
    val maxOreNeeded = listOf(oreBotOre, clayBotOre, obsidianBotOre, geodeBotOre).max()

    var current: State
    while (nodes.isNotEmpty()) {
        current = nodes.removeLast()

        // do current action
        // take resources
        when (current.action) {
            Action.Mine -> {
                current.resources += current.robots
            }

            Action.BuildOre -> {
                if (current.resources.ore >= oreBotOre) {
                    current.resources.ore -= oreBotOre
                    // mine
                    current.resources += current.robots
                    // add robot
                    current.robots.ore += 1
                } else continue
            }

            Action.BuildClay -> {
                if (current.resources.ore >= clayBotOre) {
                    current.resources.ore -= clayBotOre
                    // mine
                    current.resources += current.robots
                    // add robot
                    current.robots.clay += 1
                } else continue
            }

            Action.BuildObsidian -> {
                if (current.resources.ore >= obsidianBotOre && current.resources.clay >= obsidianBotClay) {
                    current.resources.ore -= obsidianBotOre
                    current.resources.clay -= obsidianBotClay
                    // mine
                    current.resources += current.robots
                    // add robot
                    current.robots.obsidian += 1
                } else continue
            }

            Action.BuildGeode -> {
                if (current.resources.ore >= geodeBotOre && current.resources.obsidian >= geodeBotObsidian) {
                    current.resources.ore -= geodeBotOre
                    current.resources.obsidian -= geodeBotObsidian
                    // mine
                    current.resources += current.robots
                    // add robot
                    current.robots.geode += 1
                } else continue
            }
        }

        // TODO cache/memoize
        // - triangle()
        // - don't add any States that have already been seen!
        // Optimize here!
        // add branches
        if (current.minute < minutes) {

            val minutesRemain = minutes - current.minute

            // Is it possible to get more than maxGeodes?
            val maxTheoreticalGeodes =
                current.resources.geode + (current.robots.geode * minutesRemain) + triangle(minutesRemain - 1)
            // prune
            if (maxTheoreticalGeodes <= maxGeodes) {
                continue
            }

            nodes.add(State(current.minute + 1, current.robots.copy(), current.resources.copy(), Action.Mine))


            // don't add if there will be more of this robot than any robot needs for creation
            // don't add if there aren't enough resources!
            if (current.robots.ore < maxOreNeeded && current.resources.ore >= oreBotOre) {
                nodes.add(State(current.minute + 1, current.robots.copy(), current.resources.copy(), Action.BuildOre))
            }

            if (current.robots.clay < obsidianBotClay && current.resources.ore >= clayBotOre) {
                nodes.add(State(current.minute + 1, current.robots.copy(), current.resources.copy(), Action.BuildClay))
            }

            if (current.robots.obsidian < geodeBotObsidian && current.resources.clay >= obsidianBotClay && current.resources.ore >= obsidianBotOre) {
                nodes.add(
                    State(
                        current.minute + 1,
                        current.robots.copy(),
                        current.resources.copy(),
                        Action.BuildObsidian
                    )
                )
            }

            if (current.resources.obsidian >= geodeBotObsidian && current.resources.ore >= geodeBotOre) {
                nodes.add(State(current.minute + 1, current.robots.copy(), current.resources.copy(), Action.BuildGeode))
            }
        }

        if (current.resources.geode > maxGeodes) {
            maxGeodes = current.resources.geode

            val timeSeconds = elapsedTimeInSecondsSince(startTime)
            debug("$timeSeconds seconds... ")

            debugln("maxGeodes $maxGeodes. Best node $current")
        }
//        debugln()
    }

//        val blueprintMaxGeodes = leafNodes.maxOf { it.resources.geode }
//        debugln("blueprintMaxGeodes $blueprintMaxGeodes")
    debugln("maxGeodes $maxGeodes")
    debugln()
    return maxGeodes
}

enum class Action { Mine, BuildOre, BuildClay, BuildObsidian, BuildGeode }

data class Rock(
    var ore: Int = 0,
    var clay: Int = 0,
    var obsidian: Int = 0,
    var geode: Int = 0
) {
    operator fun plusAssign(rock: Rock) {
        ore += rock.ore
        clay += rock.clay
        obsidian += rock.obsidian
        geode += rock.geode
    }
}

data class State(
    val minute: Int,
    val robots: Rock,
    val resources: Rock,
    val action: Action
)
