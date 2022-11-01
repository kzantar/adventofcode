import scala.io.Source
import scala.collection.mutable

case class Bag(color: String)

object Answer extends App {
    val rules = Source.fromFile("7_input.txt").getLines()

    val rulePattern = raw"(\w+ \w+) bags contain (.+)\.".r
    val containPattern = raw"(\d+) (\w+ \w+) bags?".r

    val bags: mutable.HashMap[Bag, List[(Int, Bag)]] = mutable.HashMap()

    for (rule <- rules) {
        rule match {
            case rulePattern(color, contains) => {
                val bag = Bag(color)
                val containsBags: List[(Int, Bag)] = contains.split(", ")
                .filterNot(_ == "no other bags")
                .map({
                    case containPattern(num, color) => (num.toInt, Bag(color))
                })
                .toList
                
                bags(bag) = containsBags
            }
        }
    }

    def capacity(bag: Bag): Int = {
        var result = 0

        val lst = bags(bag)
        lst.foreach({ 
            case (num, b) => {
                result += num
                result += num * capacity(b)
        }})

        result
    } 


    val gold = Bag("shiny gold")
    val result = capacity(gold)

    println(s"Answer is $result")

}