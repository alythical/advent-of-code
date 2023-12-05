object Day1 {
    fun solve(input: String): Pair<Int, Int> {
        val totals = input
            .split("\n\n")
            .map { it ->
                it.split("\n").sumOf { it.toInt() }
            }
        val most = totals.max()
        val top3 = totals.sortedDescending().slice(0..2).sum()
        return Pair(most, top3)
    }
}

fun main() {
    val loader = InputLoader()
    println(Day1.solve(loader.getExample(2022, 1)))
    println(Day1.solve(loader.get(2022, 1)))
}