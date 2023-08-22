import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class Day1Test {
    private val loader = InputLoader()

    @Test
    fun example() {
        val result = Day1.solve(this.loader.getExample(2022, 1))
        assertEquals(Pair(24000, 45000), result)
    }

    @Test
    fun actual() {
        val result = Day1.solve(this.loader.get(2022, 1))
        assertEquals(Pair(69836, 207968), result)
    }
}