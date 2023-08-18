import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class Day2Test {
    private val loader = InputLoader()

    @Test
    fun example() {
        val result = Day2.solve(this.loader.getExample(2022, 2))
        assertEquals(result, Pair(15, 12))
    }

    @Test
    fun actual() {
        val result = Day2.solve(this.loader.get(2022, 2))
        assertEquals(result, Pair(13565, 12424))
    }
}