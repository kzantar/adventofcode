import scala.io.Source
import scala.collection.mutable.ArrayBuffer


class Row(row: String) {
    val ab = ArrayBuffer.from(row)

    def apply(n: Int): Char = {
        while (n >= ab.size) ab ++= ArrayBuffer.from(row)
        ab(n)
    }

    def update(n: Int, elem: Char): Char = {
        ab(n) = elem
        elem
    }

    override def toString = ab.mkString
}


object Forest {
    val rows: ArrayBuffer[Row] = new ArrayBuffer

    def apply(n: Int): Row = {
        if (n >= rows.length)
            rows.last
        else
            rows(n)
    }

    def load(file: String): Unit = {
        val data = Source.fromFile(file).getLines()
        for (line <- data) rows += new Row(line)
    }

    override def toString = {
        val result = new StringBuilder

        for (row <- rows) {
            result ++= row.toString
            result += '\n'
        }

        result.result()
    }

    def height = rows.length
}


object Answer extends App {
    Forest.load("3_input.txt")

    val slopes = List(
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    )

    val treesCount: ArrayBuffer[Int] = new ArrayBuffer

    for ((dx, dy) <- slopes) {
        var countTrees = 0
        var x, y = 0

        do {
            y += dy
            x += dx
            if (Forest(y)(x) == '#') {
                countTrees += 1
                Forest(y)(x) = 'X'
            } else Forest(y)(x) = 'O'
        } while (y < Forest.height - 1)

        println(s"dx=$dx, dy=$dy, count=$countTrees")
        // println(Forest)

        treesCount += countTrees

    }

    println(treesCount.product)
}