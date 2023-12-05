import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class Day3Test {
    private val loader = InputLoader()

    @Test
    fun example() {
        val result = Day3.solve(this.loader.getExample(2022, 3))
        assertEquals(Pair(157, 70), result)
    }

    @Test
    fun actual() {
        val result = Day3.solve(this.loader.get(2022, 3))
        assertEquals(Pair(8053, 2425), result)
    }
}