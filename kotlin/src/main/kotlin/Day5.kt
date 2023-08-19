data class Instruction(val count: Int, val from: Int, val to: Int)
data class Offsets(val prelude: Int, val instrStart: Long)

object Day5 {
    private fun parse(input: String, offsets: Offsets): List<MutableList<Char>> {
        val stacks: MutableList<MutableList<Char>> = mutableListOf()
        var cur = 0
        for ((i, line) in input.lines().take(offsets.prelude).withIndex()) {
            for (j in 1..line.length step 4) {
                val char = line[j]
                if (i == 0) stacks.add(mutableListOf())
                if (char != ' ') stacks[cur].add(char)
                cur += 1
            }
            cur = 0
        }
        return stacks
    }

    fun solve(input: String, offsets: Offsets): Pair<String, String> {
        return Pair(
            moveStacks(input, offsets, false),
            moveStacks(input, offsets, true)
        )
    }

    private fun moveStacks(input: String, offsets: Offsets, partTwo: Boolean): String {
        val stacks = parse(input, offsets)
        val instrs = input.lines().stream().skip(offsets.instrStart).map {
            val s = it.split(' ')
            Instruction(s[1].toInt(), s[3].toInt(), s[5].toInt())
        }
        for (instr in instrs) {
            val from = stacks[instr.from - 1]
            val to = stacks[instr.to - 1]
            var moving = from.subList(0, instr.count).toList() // create a duplicate
            moving = if (partTwo) { moving.reversed() } else { moving }
            for (char in moving) {
                to.add(0, char)
                from.removeAt(0)
            }
        }
        return stacks.map { it.first() }.joinToString("")
    }
}

fun main() {
    val loader = InputLoader()
    println(Day5.solve(loader.getExample(2022, 5), Offsets(3, 5)))
    println(Day5.solve(loader.get(2022, 5), Offsets(8, 10)))
}