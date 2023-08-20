import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class Day6Test {
    private val loader = InputLoader()

    @Test
    fun example() {
        val result = Day6.solve(this.loader.getExample(2022, 6))
        assertEquals(result, Pair(11, 26))
    }

    @Test
    fun actual() {
        val result = Day6.solve(this.loader.get(2022, 6))
        assertEquals(result, Pair(1582, 3588))
    }
}