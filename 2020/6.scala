import scala.io.Source
import scala.collection.mutable.Set


object Answer extends App {

    def anyone_answered(group: String): Int = {
        val chars: Set[Char] = Set()

        for (ch <- group if ch != '\n')
            chars += ch

        chars.size
    }

    def everyone_answered(group: String): Int = {
        val answersByPerson = group.split('\n')

        answersByPerson.map(_.toSet).reduce(_ & _).size
    }

    val groups = Source.fromFile("6_input.txt").mkString.split("\n\n")

    val resultOne = groups.map(anyone_answered).sum
    val resultTwo = groups.map(everyone_answered).sum

    println(s"Answer for one is $resultOne")
    println(s"Answer for two is $resultTwo")

}