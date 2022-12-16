fun debug(out: String) {
    if (DEBUG) print(out)
}

fun debugln(out: String) {
    if (DEBUG) println(out)
}

fun <T> MutableList<T>.mapInPlace(mutator: (T) -> (T)) {
    this.forEachIndexed { i, value ->
        val changedValue = mutator(value)

        if (value != changedValue) {
            this[i] = changedValue
        }
    }
}
