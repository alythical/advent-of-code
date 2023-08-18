class InputLoader {
    private val loader: ClassLoader = Thread.currentThread().contextClassLoader

    fun getExample(year: Int, day: Int): String {
        return this.loader.getResource("$year/examples/day$day.txt")!!.readText()
    }

    fun get(year: Int, day: Int): String {
        return this.loader.getResource("$year/actual/day$day.txt")!!.readText()
    }
}