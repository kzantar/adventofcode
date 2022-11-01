import scala.io.Source
import scala.collection.mutable

case class Bag(color: String)

object Answer extends App {
    val rules = Source.fromFile("7_input.txt").getLines()

    val rulePattern = raw"(\w+ \w+) bags contain (.+)\.".r
    val containPattern = raw"(\d+) (\w+ \w+) bags?".r

    val bags: mutable.HashMap[Bag, List[Bag]] = mutable.HashMap()

    for (rule <- rules) {
        rule match {
            case rulePattern(color, contains) => {
                val bag = Bag(color)
                val containsBags: List[Bag] = contains.split(", ")
                .filterNot(_ == "no other bags")
                .map({
                    case containPattern(num, color) => Bag(color)
                })
                .toList
                
                bags(bag) = containsBags
            }
        }
    }

    val visited: mutable.Set[Bag] = mutable.Set()
    
    def count(bag: Bag): Int = {
        var counter = 0
        
        for ((b, lst) <- bags if lst.contains(bag)) {     
            if (!visited.contains(b)){
                visited += b
                counter += 1
            }
            
            counter += count(b)
        }

        counter
    }

    val gold = Bag("shiny gold")
    val result = count(gold)

    println(s"Answer is $result")

}