import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class Day5Test {
    private val loader = InputLoader()

    @Test
    fun example() {
        val result = Day5.solve(this.loader.getExample(2022, 5), Offsets(3, 5))
        assertEquals(Pair("CMZ", "MCD"), result)
    }

    @Test
    fun actual() {
        val result = Day5.solve(this.loader.get(2022, 5), Offsets(8, 10))
        assertEquals(Pair("ZBDRNPMVH", "WDLPFNNNB"), result)
    }
}