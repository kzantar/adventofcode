// --- Day 1: Report Repair ---

// Before you leave, the Elves in accounting just need you to fix your expense report (your puzzle input); apparently, something isn't quite adding up.
// Specifically, they need you to find the two entries that sum to 2020 and then multiply those two numbers together.
// For example, suppose your expense report contained the following:

// 1721
// 979
// 366
// 299
// 675
// 1456

// In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying them together produces 1721 * 299 = 514579, so the correct answer is 514579.

// Of course, your expense report is much larger. Find the two entries that sum to 2020; what do you get if you multiply them together?
import scala.io.Source

object TestInput {
    val data = List(
        1721,
        979,
        366,
        299,
        675,
        1456,
    )
}

object Input {
    val data = Source.fromFile("1_input.txt").getLines().map(_.toInt).toList
}

object PartOne {
    for {
        comb <- Input.data.combinations(2)
        if comb.sum == 2020
    } println(comb.product)
}


object PartTwo {
    for {
        comb <- Input.data.combinations(3)
        if comb.sum == 2020
    } println(comb.product)
}

object Answer extends App {
    PartTwo
}