import scala.io.Source

object Seat {

    def apply(path: String): Int = {
        val (rowPath, colPath) = path.splitAt(7)

        val row = findRow(rowPath)
        val col = findCol(colPath)

        seatID(row, col)
    }

    def findRow(path: String): Int = {
        var rows: Range = 0 to 127

        for (ch <- path) {
            rows = split(ch, rows)
        }

        rows.head
    }

    def findCol(path: String): Int = {
        var cols: Range = 0 to 7

        for (ch <- path) {
            cols = split(ch, cols)
        }

        cols.head
    }

    def seatID(row: Int, col: Int): Int = {
        row * 8 + col
    }

    def split(dir: Char, seq: Range): Range = {
        val (lower, upper) = seq.splitAt(seq.size/2)
        dir match {
            case 'F' | 'L' => lower
            case 'B' | 'R' => upper
        }
    }
} 


object Answer extends App {
    val passes = Source.fromFile("5_input.txt").getLines().toList

    val ids = passes.map(Seat.apply)

    val pairs = ids.sorted.sliding(2).toList
    
    val result: Int = pairs.find(l => l.last-l.head > 1) match {
        case Some(List(left, right)) => left + 1
        case _ => -1
    }

    println(s"Answer is $result")

}