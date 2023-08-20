object Day6 {
    fun solve(input: String): Pair<Int, Int> {
        return Pair(marker(input, 4), marker(input, 14))
    }

    private fun marker(input: String, size: Int): Int {
        val result = input.asSequence().withIndex().windowed(size).find { it ->
            val set = mutableSetOf<Char>()
            set.addAll(it.map { it.value })
            set.size == size
        }!!
        return result[size - 1].index + 1
    }
}

fun main() {
    val loader = InputLoader()
    println(Day6.solve(loader.getExample(2022, 6)))
    println(Day6.solve(loader.get(2022, 6)))
}
