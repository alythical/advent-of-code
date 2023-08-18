import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class Day4Test {
    private val loader: InputLoader = InputLoader()

    @Test
    fun example() {
        val result = Day4.solve(this.loader.getExample(2022, 4))
        assertEquals(result, Pair(2, 4))
    }

    @Test
    fun actual() {
        val result = Day4.solve(this.loader.get(2022, 4))
        assertEquals(result, Pair(433, 852))
    }
}