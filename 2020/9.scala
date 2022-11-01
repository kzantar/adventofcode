import scala.io.Source

object Answer extends App {
    val lenPreamble = args(1).toInt
    val numbers = Source.fromFile(args(0)).getLines().map(_.toLong).toVector

    def findInvalid(numbers: Vector[Long], preamble: Int = 5): Long = {
        for (slide <- numbers.sliding(preamble+1)) {
            if (!check(slide)) return slide.last
        }

        -1
    }

    def check(slide: Vector[Long]): Boolean = {
        val preamble = slide.init
        val item = slide.last

        preamble.combinations(2).map(_.sum).exists(_ == item)
    }

    def findRange(numbers: Vector[Long], result: Long): Vector[Long] = {
        val nums = numbers.filter(_ < result)
        for {
            i <- 2 to 100
            slide <- nums.sliding(i)
        } if (slide.sum == result) return slide

        Vector[Long]()
    }

    val result = findInvalid(numbers, lenPreamble)
    val range = findRange(numbers, result)
    val answer = range.min + range.max
    println(s"Invalid number is $result. Answer is $answer")
}