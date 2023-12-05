object Day3 {
    fun solve(input: String): Pair<Int, Int> {
        return Pair(partOne(input), partTwo(input))
    }

    private fun computeValue(c: Char): Int {
        return if (c.isLowerCase()) c - 'a' + 1 else c - 'A' + 27
    }

    private fun partOne(input: String): Int {
        val rucksacks = input.lines().map {
            it.slice(0..<it.length / 2).toHashSet().toList() +
                    it.slice(it.length / 2..<it.length).toHashSet().toList()
        }
        return rucksacks.sumOf { it ->
            val chars = HashSet<Char>()
            val char = it.find {
                !chars.add(it)
            }!!.toChar() // guaranteed to find char
            this.computeValue(char)
        }
    }

    private fun partTwo(input: String): Int {
        val groups = input.lines().windowed(3, 3).map { it ->
            it.map { it.toHashSet().toList() }
        }
        return groups.sumOf { group ->
            val counts = HashMap<Char, Int>()
            for (person in group) {
                for (c in person) {
                    val cur = counts[c] ?: 0
                    counts[c] = cur + 1
                }
            }
            val char = counts.filterValues { it == 3 }.keys.first() // guaranteed to resolve to badge char
            this.computeValue(char)
        }
    }
}

fun main() {
    val loader = InputLoader()
    println(Day3.solve(loader.getExample(2022, 3)))
    println(Day3.solve(loader.get(2022, 3)))
}