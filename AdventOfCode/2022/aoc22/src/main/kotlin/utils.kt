import java.io.BufferedReader
import java.io.InputStreamReader

fun debug(out: Any) {
    if (DEBUG) print(out)
}

fun debugln(out: Any = "") {
    if (DEBUG) println(out)
}

// Extend MutableList to allow mapping in place.
//
//fun mapInPlaceExample(){
//    val mutableList = MutableList<Int>(10) { 0 }
//    mutableList.mapInPlace { value -> value + 1 }
//}
fun <T> MutableList<T>.mapInPlace(mutator: (T) -> (T)) {
    this.forEachIndexed { i, value ->
        val changedValue = mutator(value)

        if (value != changedValue) {
            this[i] = changedValue
        }
    }
}

fun getTextFromResource(resourceName: String): String {
    val resourceStream = ClassLoader.getSystemResourceAsStream(resourceName)
    val reader = BufferedReader(InputStreamReader(resourceStream))
    val text = reader.readText()
    resourceStream.close()
    return text
}

// Check if two ranges overlap.
fun overlap(range0: List<Int>, range1: List<Int>): Boolean {
    if (range0[0] >= range1[0] && range0[0] <= range1[1]) return true
    if (range0[1] >= range1[0] && range0[1] <= range1[1]) return true
    return false
}

// Check if the first range is inside the second range.
fun within(range0: List<Int>, range1: List<Int>): Boolean {
    // example: 3,4 and 2,5
    if (range0[0] >= range1[0] && range0[1] <= range1[1]) {
        return true
    }
    return false
}

fun isUniqueChars(string: String, size: Int): Boolean {
    return string.toSet().size == size
}

fun triangle(n: Int): Int {
    return n * (n + 1) / 2
}

fun elapsedTimeInSecondsSince(time: Long): Long {
    return (System.nanoTime() - time) / 1_000_000_000
}

