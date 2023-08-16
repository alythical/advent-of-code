enum class Shape(val score: Int) {
    Rock(1),
    Paper(2),
    Scissors(3),
    Unknown(4)
}

object Day2 {
    private fun parse(input: String): List<List<Shape>> {
        return input
            .lines()
            .map { it ->
                it
                    .split(" ")
                    .map {
                        when (it.trim()) {
                            "A", "X" -> Shape.Rock
                            "B", "Y" -> Shape.Paper
                            "C", "Z" -> Shape.Scissors
                            else -> Shape.Unknown
                        }
                    }
            }
    }

    fun solve(input: String): Pair<Int, Int> {
        val parsed = this.parse(input)
        return Pair(partOne(parsed), partTwo(parsed))
    }

    private fun partOne(parsed: List<List<Shape>>): Int {
        return parsed.sumOf {
            val a = it[0]
            val b = it[1]
            if (a == b) {
                b.score + 3
            } else {
                when (Pair(b, a)) {
                    Pair(Shape.Rock, Shape.Scissors) -> b.score + 6
                    Pair(Shape.Paper, Shape.Rock) -> b.score + 6
                    Pair(Shape.Scissors, Shape.Paper) -> b.score + 6
                    else -> b.score
                }
            }
        }
    }

    private fun partTwo(parsed: List<List<Shape>>): Int {
        return parsed.sumOf {
            val opponent = it[0]
            val win = it[1]
            val choices = listOf(Shape.Rock.score, Shape.Paper.score, Shape.Scissors.score)
            when (win) {
                Shape.Scissors -> choices[opponent.score % 3] + 6
                Shape.Paper -> 3 + opponent.score
                Shape.Rock -> choices[(opponent.score + 1) % 3]
                Shape.Unknown -> 0
            }
        }
    }
}

fun main() {
    val loader = InputLoader()
    println(Day2.solve(loader.getExample(2022, 2)))
    println(Day2.solve(loader.get(2022, 2)))
}