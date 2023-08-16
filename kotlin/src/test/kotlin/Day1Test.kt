import org.junit.jupiter.api.Test

import org.junit.jupiter.api.Assertions.*

class Day1Test {
    private val loader: InputLoader = InputLoader()

    @Test
    fun example() {
        val result = solve(this.loader.getExample(2022, 1))
        assertEquals(result, Pair(24000, 45000))
    }

    @Test
    fun actual() {
        val result = solve(this.loader.get(2022, 1))
        assertEquals(result, Pair(69836, 207968))
    }
}