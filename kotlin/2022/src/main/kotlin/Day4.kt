object Day4 {
    private fun parse(input: String): List<List<IntRange>> {
        return input.lines().map { it ->
            it.split(",").map {
                val (leftBound, rightBound) = it.split("-")
                leftBound.toInt()..rightBound.toInt()
            }
        }
    }

    fun solve(input: String): Pair<Int, Int> {
        val parsed = parse(input)
        return Pair(partOne(parsed), partTwo(parsed))
    }

    private fun partOne(pairs: List<List<IntRange>>): Int {
        return pairs.count {
            val left = it[0]
            val right = it[1]
            left.contains(right.first) && left.contains(right.last) || right.contains(left.first) && right.contains(left.last)
        }
    }

    private fun partTwo(pairs: List<List<IntRange>>): Int {
        return pairs.count {
            val left = it[0]
            val right = it[1]
            left.contains(right.first) || left.contains(right.last) || right.contains(left.first) || right.contains(left.last)
        }
    }
}

fun main() {
    val loader = InputLoader()
    println(Day4.solve(loader.getExample(2022, 4)))
    println(Day4.solve(loader.get(2022, 4)))
}