import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class Day3Test {
    private val loader: InputLoader = InputLoader()

    @Test
    fun example() {
        val result = Day3.solve(this.loader.getExample(2022, 3))
        assertEquals(result, Pair(157, 70))
    }

    @Test
    fun actual() {
        val result = Day3.solve(this.loader.get(2022, 3))
        assertEquals(result, Pair(8053, 2425))
    }
}