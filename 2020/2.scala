import scala.io.Source

object TestInput {
    val data = """
    1-3 a: abcde
    1-3 b: cdefg
    2-9 c: ccccccccc
    """.split('\n').map(_.trim).filter(_.size > 0)
}


object Input {
    val data = Source.fromFile("2_input.txt").getLines()
}


object PartOne {
    def checkPassword(min: Int, max: Int, ch: Char, password: String): Boolean = {
        val cnt = password.count(_ == ch)
        min <= cnt && cnt <= max
    }
}

object PartTwo {
    def checkPassword(first: Int, second: Int, ch: Char, password: String): Boolean = {
        password(first-1) == ch ^ password(second-1) == ch
    }
}

object Answer extends App {
    val checkPassword = PartTwo.checkPassword _
    val data = Input.data

    val pattern = raw"(\d+)-(\d+) ([a-z]): ([a-z]+)".r
    var validPassword = 0
    for (line <- data) {
        val pattern(num1, num2, ch, password) = line
        if (checkPassword(num1.toInt, num2.toInt, ch(0), password)) validPassword += 1
    }
    println(validPassword)
}